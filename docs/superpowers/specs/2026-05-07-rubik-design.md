# Rubik — Cyclomatic Complexity Analyzer

## Overview

Rubik is a Rust CLI application that computes cyclomatic code complexity for individual source files or entire directories. It supports Rust, Python, and JavaScript, reporting complexity per function/closure and per file.

## Goals

- Compute cyclomatic complexity accurately for Rust, Python, and JavaScript source code.
- Report complexity per named function and per file.
- Include closures and anonymous functions in the per-function report.
- Provide both human-readable console tables and machine-readable JSON output.
- Skip unparseable files with an error message and continue processing.

## Non-Goals

- Complexity thresholds or warnings (e.g., "this function is too complex").
- Languages other than Rust, Python, and JavaScript.
- Integration with CI pipelines or Git hooks (the CLI can be used manually for that).

## Architecture

### Module Layout

```
src/
├── main.rs              # CLI entry point (clap argument parsing), I/O orchestration
├── lib.rs               # Public API: analyze(path) -> Results
├── analyzer.rs          # File discovery, dispatch to language analyzers
├── complexity.rs        # Shared decision-point counter logic
├── language/
│   ├── mod.rs           # LanguageAnalyzer trait definition
│   ├── rust.rs          # Tree-sitter Rust parser & analyzer
│   ├── python.rs        # Tree-sitter Python parser & analyzer
│   └── javascript.rs    # Tree-sitter JavaScript parser & analyzer
└── output/
    ├── mod.rs           # Output trait + dispatcher
    ├── pretty.rs        # Console tables (comfy-table)
    └── json.rs          # JSON serialization (serde_json)
```

### Core Trait

```rust
trait LanguageAnalyzer {
    fn can_analyze(&self, path: &Path) -> bool;
    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String>;
}
```

Each language module implements this trait. `analyzer.rs` selects the correct analyzer based on file extension and delegates.

## Cyclomatic Complexity Calculation

For each function or closure body:

```
complexity = 1 (base) + number of decision points
```

Per-file complexity is the sum of all function complexities in that file.

### Decision Points by Language

| Decision Point | Rust | Python | JavaScript |
|---|---|---|---|
| `if` / `elif` | `if_expression`, `if_let_expression` | `if_statement`, `elif_clause` | `if_statement` |
| `match` / `switch` / `case` | `match_expression` (+1 per arm) | `match_statement` / `case_clause` (+1 per case) | `switch_statement` (+1 per case) |
| `for` | `for_expression` | `for_statement` | `for_statement` |
| `while` | `while_expression`, `while_let_expression` | `while_statement` | `while_statement`, `do_statement` |
| `loop` | `loop_expression` | — | — |
| `?` (try) | `try_expression` | — | — |
| `catch` / `except` | — | `except_clause` | `catch_clause` |
| `&&` / `\|\|` | `and_expression`, `or_expression` | `and`, `or` | `&&`, `\|\|` |
| Ternary | — | `conditional_expression` | `ternary_expression` |
| Lambda / Closure | `closure_expression` | `lambda` | `arrow_function` |

## Data Structures

```rust
struct FunctionComplexity {
    name: String,       // e.g. "my_func" or "<closure>@line 42"
    line_start: usize,
    line_end: usize,
    complexity: u32,
}

struct FileResult {
    path: PathBuf,
    total_complexity: u32,
    functions: Vec<FunctionComplexity>,
    error: Option<String>,  // populated if parsing failed
}
```

## CLI Interface

```bash
rubik [PATH]                     # default: pretty-printed table
rubik [PATH] --format json       # JSON output
rubik [PATH] -f json
```

## Error Handling

- Parse errors are captured in `FileResult.error`, printed to stderr, and the process continues (skip-and-continue behavior).
- Other I/O errors (file not found, permission denied) are printed to stderr and the process exits with code 1.

## Testing Strategy

- **Unit tests:** For each language analyzer, test decision-point counting against small source snippets with known complexity.
- **Integration tests:** Invoke the CLI on fixture files and verify JSON output structure and values.
- **Edge cases:** Nested functions, empty files, files with no functions, invalid syntax (error capture), and multi-line boolean expressions.

## Dependencies

| Crate | Purpose |
|---|---|
| `clap` | CLI argument parsing |
| `tree-sitter` | Core tree-sitter parser framework |
| `tree-sitter-rust` | Rust grammar |
| `tree-sitter-python` | Python grammar |
| `tree-sitter-javascript` | JavaScript grammar |
| `comfy-table` | Pretty-printed console tables |
| `serde` + `serde_json` | JSON serialization |
| `walkdir` | Recursive directory traversal |

## Design Decisions

- **Tree-sitter over native parser crates:** One consistent AST API across all three languages. `syn`, `rustpython-parser`, and `swc_ecma_parser` are all excellent but have disparate APIs and heavier combined dependency trees. Tree-sitter is battle-tested for polyglot tooling.
- **Regex/text scanning rejected:** Too fragile for a tool meant to be reliable on real code. Comments, strings, and nested constructs make regex-based counting error-prone.
- **Closure inclusion:** The user explicitly requested closures/anonymous functions be treated as standalone functions. This aligns with many linters (e.g., ESLint's complexity rule).
- **Pretty + JSON output:** JSON enables piping to other tools (CI dashboards, editors), while tables are the default for human readability.
