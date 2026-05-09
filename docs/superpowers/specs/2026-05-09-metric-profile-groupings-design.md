# Metric-Profile Groupings Design

**Date:** 2026-05-09
**Topic:** Cross-file structural duplication detection via metric-profile grouping

---

## Problem Statement

Rubik computes per-function complexity metrics independently for each file and reports them independently. When the same function is duplicated across multiple files, the tool computes identical metric profiles but never compares them across the file boundary. This leaves structural duplication invisible to users.

The retrospective analysis of the rubik codebase itself revealed this gap: functions like `collect_functions`, `count_decisions`, and `analyze` had bit-for-bit identical implementations across `rust.rs`, `javascript.rs`, and `c.rs`, yet rubik only reported them as separate per-file entries.

## Goal

Add a lightweight cross-file comparison step that groups named functions with identical metric profiles and surfaces them in every output format as "Structural Duplication Candidates."

## Constraints

- Named functions only; closure/anonymous clustering deferred.
- Exact metric match only; no tolerance bands.
- Minimal changes to existing data model and formatter architecture.
- Must work with data the tool already produces (no new AST analysis).

## Architecture

### New Module: `src/duplicates.rs`

Pure computation module. No I/O, no formatting.

```rust
pub struct DuplicateCluster {
    pub name: String,
    pub instances: Vec<ClusterInstance>,
}

pub struct ClusterInstance {
    pub path: PathBuf,
    pub line_start: usize,
    pub complexity: u32,
    pub lines: usize,
    pub nesting_depth: u32,
    pub halstead_volume: f64,
}

pub fn compute_duplicates(results: &[FileResult]) -> Vec<DuplicateCluster>
```

Algorithm:
1. Flatten every function from every file into a temporary vec.
2. Group by `function.name`.
3. For each name appearing in 2+ files, check whether all instances have identical `(lines, complexity, nesting_depth, halstead_volume)`.
4. If yes, emit a `DuplicateCluster` sorted by `path`.
5. Return clusters sorted by instance count descending, then alphabetically by name.

If no clusters are found, return an empty vec.

### Updated Data Model

`src/lib.rs`:
- Add `pub mod duplicates;`
- `AnalysisOutput` gains `pub clusters: Option<Vec<DuplicateCluster>>`.

### Formatter Changes

All three formatters receive pre-computed clusters from the CLI layer and render them when present.

**Markdown (`src/output/markdown.rs`)**
- After the summary and before per-file sections, emit `## Structural Duplication Candidates`.
- For each cluster, emit `### {name} ({n} exact match[es])` followed by a table of instances.

**Pretty (`src/output/pretty.rs`)**
- Before per-file entries, emit a compact duplication block.

**JSON (`src/output/json.rs`)**
- Serialize `clusters` directly into `AnalysisOutput`.

### CLI Integration

`main.rs` calls `compute_duplicates(&results)` once after analysis. The result is passed into the formatter alongside `Vec<FileResult>`.

## Output Examples

### Markdown

```markdown
## Structural Duplication Candidates

### collect_functions (3 exact matches)
| File | Line | Complexity | Lines | Halstead Volume |
|------|------|------------|-------|-----------------|
| c.rs | 49 | 3 | 28 | 469.13 |
| javascript.rs | 57 | 3 | 28 | 469.13 |
| rust.rs | 51 | 3 | 28 | 469.13 |

### analyze (4 exact matches)
| File | Line | Complexity | Lines | Halstead Volume |
|------|------|------------|-------|-----------------|
| c.rs | 33 | 4 | 14 | 253.82 |
| javascript.rs | 41 | 4 | 14 | 253.82 |
| python.rs | 33 | 4 | 14 | 253.82 |
| rust.rs | 35 | 4 | 14 | 253.82 |
```

### JSON

```json
{
  "summary": { ... },
  "files": [ ... ],
  "clusters": [
    {
      "name": "collect_functions",
      "instances": [
        { "path": "c.rs", "line_start": 49, "complexity": 3, "lines": 28, "nesting_depth": 1, "halstead_volume": 469.13 },
        { "path": "javascript.rs", "line_start": 57, "complexity": 3, "lines": 28, "nesting_depth": 1, "halstead_volume": 469.13 },
        { "path": "rust.rs", "line_start": 51, "complexity": 3, "lines": 28, "nesting_depth": 1, "halstead_volume": 469.13 }
      ]
    }
  ]
}
```

### Pretty

```
Structural Duplication Candidates

collect_functions (3 exact matches)
  c.rs:49           CC=3  lines=28  nest=1  vol=469.13
  javascript.rs:57  CC=3  lines=28  nest=1  vol=469.13
  rust.rs:51        CC=3  lines=28  nest=1  vol=469.13

analyze (4 exact matches)
  c.rs:33           CC=4  lines=14  nest=1  vol=253.82
  javascript.rs:41  CC=4  lines=14  nest=1  vol=253.82
  python.rs:33      CC=4  lines=14  nest=1  vol=253.82
  rust.rs:35        CC=4  lines=14  nest=1  vol=253.82
```

## Testing Strategy

**Unit tests in `duplicates.rs`:**
1. `test_no_duplicates` — two files with unique function names → empty vec.
2. `test_exact_match_cluster` — two files with identical `collect_functions` → one cluster with two instances.
3. `test_same_name_different_metrics` — two files with same name but different complexity → no cluster.

**Integration test:**
- Add `tests/fixtures/duplicates/` with two `.rs` files containing identical functions.
- Run `analyze_path` and assert `compute_duplicates` returns the expected cluster.

## Out of Scope

- Anonymous/closure clustering.
- Tolerance-based near-match clustering.
- AST structural fingerprinting.
- Configurable thresholds or CLI flags to disable the feature.

These are valid future extensions but are deliberately excluded to keep the initial implementation minimal and focused.

## Open Questions

None. All design decisions have been validated through the brainstorming process.
