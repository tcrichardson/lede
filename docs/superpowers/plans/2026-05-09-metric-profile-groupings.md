# Metric-Profile Groupings Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add cross-file structural duplication detection by grouping named functions with identical metric profiles and surfacing them in all output formats.

**Architecture:** A new `src/duplicates.rs` module contains pure clustering logic (`compute_duplicates`). The `OutputFormatter` trait gains a `clusters` parameter. Each formatter renders a "Structural Duplication Candidates" section. The CLI in `main.rs` calls `compute_duplicates` once after analysis.

**Tech Stack:** Rust, serde, tree-sitter, clap, comfy-table

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `src/duplicates.rs` | Create | Core clustering algorithm + unit tests |
| `src/lib.rs` | Modify | Add `pub mod duplicates;`, update `AnalysisOutput` with `clusters` field |
| `src/output/mod.rs` | Modify | Update `OutputFormatter` trait signature to accept `clusters` |
| `src/output/json.rs` | Modify | Serialize `clusters` into `AnalysisOutput` |
| `src/output/markdown.rs` | Modify | Render clusters as markdown tables |
| `src/output/pretty.rs` | Modify | Render clusters as compact text blocks |
| `src/main.rs` | Modify | Call `compute_duplicates` and pass result to formatter |

---

### Task 1: Core duplicates module with unit tests

**Files:**
- Create: `src/duplicates.rs`

- [ ] **Step 1: Write the failing tests**

Create `src/duplicates.rs`:

```rust
use std::collections::HashMap;
use crate::{FileResult, FunctionComplexity};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateCluster {
    pub name: String,
    pub instances: Vec<ClusterInstance>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterInstance {
    pub path: std::path::PathBuf,
    pub line_start: usize,
    pub complexity: u32,
    pub lines: usize,
    pub nesting_depth: u32,
    pub halstead_volume: f64,
}

pub fn compute_duplicates(_results: &[FileResult]) -> Vec<DuplicateCluster> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FunctionComplexity;

    fn make_func(name: &str, lines: usize, complexity: u32, nesting: u32, volume: f64) -> FunctionComplexity {
        FunctionComplexity {
            name: name.to_string(),
            line_start: 1,
            line_end: lines,
            lines,
            complexity,
            nesting_depth: nesting,
            halstead_volume: volume,
            halstead_difficulty: 0.0,
            halstead_effort: 0.0,
            halstead_time: 0.0,
        }
    }

    fn make_file(path: &str, functions: Vec<FunctionComplexity>) -> FileResult {
        crate::FileResult::from_functions(std::path::Path::new(path), 100, functions)
    }

    #[test]
    fn test_no_duplicates() {
        let files = vec![
            make_file("a.rs", vec![make_func("foo", 10, 2, 1, 100.0)]),
            make_file("b.rs", vec![make_func("bar", 10, 2, 1, 100.0)]),
        ];
        let clusters = compute_duplicates(&files);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_exact_match_cluster() {
        let files = vec![
            make_file("a.rs", vec![make_func("collect", 28, 3, 1, 469.13)]),
            make_file("b.rs", vec![make_func("collect", 28, 3, 1, 469.13)]),
        ];
        let clusters = compute_duplicates(&files);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].name, "collect");
        assert_eq!(clusters[0].instances.len(), 2);
    }

    #[test]
    fn test_same_name_different_metrics() {
        let files = vec![
            make_file("a.rs", vec![make_func("collect", 28, 3, 1, 469.13)]),
            make_file("b.rs", vec![make_func("collect", 30, 3, 1, 469.13)]),
        ];
        let clusters = compute_duplicates(&files);
        assert!(clusters.is_empty());
    }
}
```

- [ ] **Step 2: Register the new module in `src/lib.rs`**

Add after the existing `pub mod` declarations (~line 304):

```rust
pub mod duplicates;
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test duplicates::tests --lib`

Expected: 2 FAIL (`test_exact_match_cluster`, `test_same_name_different_metrics`) because `compute_duplicates` returns an empty vec.

- [ ] **Step 4: Implement `compute_duplicates`**

Replace the stub `compute_duplicates` in `src/duplicates.rs` with:

```rust
pub fn compute_duplicates(results: &[FileResult]) -> Vec<DuplicateCluster> {
    let mut by_name: HashMap<String, Vec<(&FileResult, &FunctionComplexity)>> = HashMap::new();

    for file in results {
        for func in &file.functions {
            by_name.entry(func.name.clone()).or_default().push((file, func));
        }
    }

    let mut clusters = Vec::new();

    for (name, instances) in by_name {
        if instances.len() < 2 {
            continue;
        }

        let first = instances[0].1;
        let all_match = instances.iter().all(|(_, func)| {
            func.lines == first.lines
                && func.complexity == first.complexity
                && func.nesting_depth == first.nesting_depth
                && func.halstead_volume == first.halstead_volume
        });

        if all_match {
            let mut cluster_instances: Vec<ClusterInstance> = instances
                .iter()
                .map(|(file, func)| ClusterInstance {
                    path: file.path.clone(),
                    line_start: func.line_start,
                    complexity: func.complexity,
                    lines: func.lines,
                    nesting_depth: func.nesting_depth,
                    halstead_volume: func.halstead_volume,
                })
                .collect();

            cluster_instances.sort_by(|a, b| a.path.cmp(&b.path));

            clusters.push(DuplicateCluster {
                name,
                instances: cluster_instances,
            });
        }
    }

    clusters.sort_by(|a, b| {
        b.instances
            .len()
            .cmp(&a.instances.len())
            .then_with(|| a.name.cmp(&b.name))
    });

    clusters
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test duplicates::tests --lib`

Expected: 3 PASS

- [ ] **Step 6: Commit**

```bash
git add src/duplicates.rs src/lib.rs
git commit -m "feat: add duplicates module with compute_duplicates and unit tests"
```

---

### Task 2: Update data model for JSON serialization

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Update `AnalysisOutput` to include clusters**

In `src/lib.rs`, find `AnalysisOutput` (~line 207) and replace it with:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub summary: SummaryStatistics,
    pub files: Vec<FileResult>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub clusters: Option<Vec<crate::duplicates::DuplicateCluster>>,
}
```

- [ ] **Step 2: Commit**

```bash
git add src/lib.rs
git commit -m "feat: add clusters field to AnalysisOutput for JSON serialization"
```

---

### Task 3: Update output trait and all formatters

**Files:**
- Modify: `src/output/mod.rs`
- Modify: `src/output/json.rs`
- Modify: `src/output/markdown.rs`
- Modify: `src/output/pretty.rs`

- [ ] **Step 1: Update `OutputFormatter` trait signature**

In `src/output/mod.rs`, replace the trait definition with:

```rust
use crate::duplicates::DuplicateCluster;

pub trait OutputFormatter {
    fn format(&self, results: &[FileResult], clusters: &[DuplicateCluster]) -> String;
}
```

- [ ] **Step 2: Update JSON formatter**

In `src/output/json.rs`, replace the entire file with:

```rust
use crate::{AnalysisOutput, FileResult, SummaryStatistics, duplicates::DuplicateCluster, output::OutputFormatter};

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &[FileResult], clusters: &[DuplicateCluster]) -> String {
        let summary = SummaryStatistics::from_results(results);

        let output = AnalysisOutput {
            summary,
            files: results.to_vec(),
            clusters: if clusters.is_empty() { None } else { Some(clusters.to_vec()) },
        };

        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
    }
}
```

- [ ] **Step 3: Update markdown formatter**

In `src/output/markdown.rs`, replace the imports and `format` method:

Replace line 1:
```rust
use crate::{FileResult, FunctionComplexity, SummaryStatistics, duplicates::DuplicateCluster, output::OutputFormatter};
```

Replace the `format` method body in `impl OutputFormatter for MarkdownFormatter` (~line 5-19):

```rust
    fn format(&self, results: &[FileResult], clusters: &[DuplicateCluster]) -> String {
        let mut out = String::new();

        let summary = SummaryStatistics::from_results(results);
        if summary.files_analyzed > 0 {
            out.push_str(&format_summary(&summary));
        }

        if !clusters.is_empty() {
            out.push_str(&format_clusters(clusters));
        }

        for file in results {
            out.push_str(&format_file(file));
        }

        out
    }
```

Add this new helper function at the end of the file (after `format_function_row`):

```rust
fn format_clusters(clusters: &[DuplicateCluster]) -> String {
    let mut out = String::from("## Structural Duplication Candidates\n\n");
    for cluster in clusters {
        let n = cluster.instances.len();
        let suffix = if n == 1 { "" } else { "es" };
        out.push_str(&format!("### {} ({} exact match{})\n\n", cluster.name, n, suffix));
        out.push_str("| File | Line | Complexity | Lines | Halstead Volume |\n");
        out.push_str("|------|------|------------|-------|-----------------|\n");
        for inst in &cluster.instances {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {:.2} |\n",
                inst.path.display(),
                inst.line_start,
                inst.complexity,
                inst.lines,
                inst.halstead_volume
            ));
        }
        out.push('\n');
    }
    out
}
```

- [ ] **Step 4: Update pretty formatter**

In `src/output/pretty.rs`, replace the imports and implementation:

Replace line 1:
```rust
use crate::{FileResult, duplicates::DuplicateCluster, output::OutputFormatter};
```

Replace the `format` method body in `impl OutputFormatter for PrettyFormatter` (~line 6-9):

```rust
    fn format(&self, results: &[FileResult], clusters: &[DuplicateCluster]) -> String {
        let mut out = String::new();
        if !clusters.is_empty() {
            out.push_str("Structural Duplication Candidates\n\n");
            for cluster in clusters {
                let n = cluster.instances.len();
                let suffix = if n == 1 { "" } else { "es" };
                out.push_str(&format!("{} ({} exact match{})\n", cluster.name, n, suffix));
                for inst in &cluster.instances {
                    out.push_str(&format!(
                        "  {}:{}  CC={}  lines={}  nest={}  vol={:.2}\n",
                        inst.path.display(),
                        inst.line_start,
                        inst.complexity,
                        inst.lines,
                        inst.nesting_depth,
                        inst.halstead_volume
                    ));
                }
                out.push('\n');
            }
        }
        out.push_str(&results.iter().map(format_file_entry).collect::<String>());
        out
    }
```

- [ ] **Step 5: Run cargo check to verify compilation**

Run: `cargo check`

Expected: clean compile with zero errors

- [ ] **Step 6: Commit**

```bash
git add src/output/mod.rs src/output/json.rs src/output/markdown.rs src/output/pretty.rs
git commit -m "feat: update all formatters to accept and render duplicate clusters"
```

---

### Task 4: Wire up CLI

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update imports and formatter call**

In `src/main.rs`, replace line 2:
```rust
use rubik::{analyze_path, duplicates::compute_duplicates, output};
```

Replace lines 37-38:
```rust
    let formatter = output::get_formatter(&args.format);
    let clusters = compute_duplicates(&results);
    println!("{}", formatter.format(&results, &clusters));
```

- [ ] **Step 2: Run full test suite**

Run: `cargo test`

Expected: all unit tests and integration tests pass

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire compute_duplicates into CLI pipeline"
```

---

## Spec Coverage Checklist

| Spec Requirement | Implementing Task |
|------------------|-------------------|
| New `src/duplicates.rs` module with `DuplicateCluster` and `ClusterInstance` | Task 1 |
| `compute_duplicates` groups named functions by exact metric match | Task 1 |
| Unit tests: no duplicates, exact match cluster, same name different metrics | Task 1 |
| `pub mod duplicates;` in `lib.rs` | Task 1 |
| `AnalysisOutput` gains `clusters` field | Task 2 |
| `OutputFormatter` trait accepts `clusters` parameter | Task 3 |
| JSON formatter serializes clusters | Task 3 |
| Markdown formatter renders cluster tables | Task 3 |
| Pretty formatter renders compact cluster blocks | Task 3 |
| CLI calls `compute_duplicates` and passes to formatter | Task 4 |

## Self-Review

- **Placeholder scan:** No TBD, TODO, or vague requirements found.
- **Type consistency:** `DuplicateCluster` and `ClusterInstance` use consistent field names across all tasks. `compute_duplicates` signature matches in Task 1 and Task 4.
- **Compilation continuity:** Task 3 updates all formatters atomically so `cargo check` passes. No intermediate broken states.
