# Rubik

A fast CLI tool that computes cyclomatic code complexity and Halstead metrics for Rust, Python, JavaScript, and C source files. It reports complexity and cognitive metrics per function and per file. By default, closures and anonymous functions are excluded from analysis so they don't skew aggregate metrics — you can opt to include them with `--include-closures`.

## Features

- **Multi-language support:** Rust, Python, JavaScript/JSX, and C
- **Cyclomatic complexity:** Classic decision-point counting per function and file
- **Halstead metrics:** Volume, difficulty, effort, and estimated time per function
- **Nesting depth analysis:** Maximum and average control-flow nesting per function and file
- **Per-function & per-file reporting:** See complexity and Halstead metrics at every level
- **Project-level summary:** Aggregated statistics across all analyzed files
- **Closure handling:** Closures, lambdas, and arrow functions are excluded by default so they don't inflate function counts or dilute averages. Use `--include-closures` to analyze them
- **Two output formats:** Pretty-printed tables (default) and JSON
- **Directory scanning:** Analyze entire codebases recursively
- **Graceful error handling:** Unparseable files are reported to stderr but do not stop the analysis

## Installation

Build from source with Cargo:

```bash
git clone <repo-url>
cd rubik
cargo build --release
```

The binary will be available at `target/release/rubik`.

## Usage

Analyze a single file:

```bash
rubik src/main.rs
```

Analyze an entire directory:

```bash
rubik src/
```

Output as JSON:

```bash
rubik src/ -f json
```

Include closures and lambdas in the analysis:

```bash
rubik src/ --include-closures
```

### CLI Options

```
Usage: rubik [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to a file or directory to analyze

Options:
  -f, --format <FORMAT>     Output format: pretty or json [default: pretty]
      --include-closures    Include closures, lambdas, and arrow functions in the analysis
  -h, --help                Print help
  -V, --version             Print version
```

## Example Output

### Pretty format (default)

## Summary Statistics

| Metric | Value |
|--------|-------|
| Files Analyzed | 3 |
| Total Functions | 12 |
| Total Lines | 450 |
| Total Complexity | 34 |
| Avg Complexity / Function | 2.83 |
| Max Nesting Depth | 4 |
| Avg Nesting Depth | 1.50 |
| Avg Halstead Volume | 78.34 |
| Avg Halstead Difficulty | 4.20 |
| Avg Halstead Effort | 329.03 |
| Avg Halstead Time | 18.28 |

### src/main.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 1 |
| Total Lines | 42 |
| Total Function Lines | 20 |
| Total Complexity | 5 |
| Avg Complexity / Function | 5.00 |
| Max Complexity | 5 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 2.00 |
| Max Function Lines | 20 |
| Avg Halstead Volume | 45.60 |
| Max Halstead Volume | 45.60 |
| Avg Halstead Difficulty | 3.20 |
| Max Halstead Difficulty | 3.20 |
| Avg Halstead Effort | 145.92 |
| Max Halstead Effort | 145.92 |
| Avg Halstead Time | 8.11 |
| Max Halstead Time | 8.11 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| main | 20 | 16-35 | 5 | 2 | 45.60 | 3.20 | 145.92 | 8.11 |

### JSON format

```json
{
  "summary": {
    "files_analyzed": 3,
    "total_functions": 12,
    "total_lines": 450,
    "total_complexity": 34,
    "avg_complexity_per_function": 2.83,
    "max_nesting_depth": 4,
    "avg_nesting_depth": 1.50,
    "avg_halstead_volume": 78.34,
    "avg_halstead_difficulty": 4.20,
    "avg_halstead_effort": 329.03,
    "avg_halstead_time": 18.28
  },
  "files": [
    {
      "path": "src/main.rs",
      "total_complexity": 5,
      "total_lines": 42,
      "function_count": 1,
      "functions": [
        {
          "name": "main",
          "line_start": 16,
          "line_end": 35,
          "lines": 20,
          "complexity": 5,
          "nesting_depth": 2,
          "halstead_volume": 45.60,
          "halstead_difficulty": 3.20,
          "halstead_effort": 145.92,
          "halstead_time": 8.11
        }
      ],
      "max_nesting_depth": 2,
      "avg_nesting_depth": 2.00,
      "avg_halstead_volume": 45.60,
      "avg_halstead_difficulty": 3.20,
      "avg_halstead_effort": 145.92,
      "avg_halstead_time": 8.11,
      "max_complexity": 5,
      "max_function_lines": 20,
      "total_function_lines": 20,
      "max_halstead_volume": 45.60,
      "max_halstead_difficulty": 3.20,
      "max_halstead_effort": 145.92,
      "max_halstead_time": 8.11
    }
  ]
}
```

## How Complexity is Calculated

For each function or closure, complexity starts at **1** and increments by **1** for each decision point:

| Decision Point | Rust | Python | JavaScript | C |
|---|---|---|---|---|
| `if` / `elif` | `if_expression` | `if_statement`, `elif_clause` | `if_statement` | `if_statement` |
| `match` / `switch` / `case` | `match_expression` (per arm) | `match_statement` (per case) | `switch_statement` (per case) | `case_statement` |
| `for` | `for_expression` | `for_statement` | `for_statement` | `for_statement` |
| `while` | `while_expression` | `while_statement` | `while_statement`, `do_statement` | `while_statement`, `do_statement` |
| `loop` | `loop_expression` | — | — | — |
| `try` / `except` / `catch` | `try_expression` | `except_clause` | `catch_clause` | — |
| `&&` / `\|\|` | binary operators | `and` / `or` | binary operators | binary operators |
| Ternary | — | `conditional_expression` | `ternary_expression` | `conditional_expression` |
| Lambda / Closure* | `closure_expression` | `lambda` | `arrow_function` | — |

\* Only counted when `--include-closures` is passed. By default, closures are excluded so they don't inflate function counts or dilute average metrics.

Per-file complexity is the sum of all function complexities in that file.

## How Metrics are Calculated

### Nesting Depth
The maximum depth of control-flow block nesting inside a function. For example, an `if` inside a `for` loop has a nesting depth of 2. Nested functions are not counted.

### Halstead Metrics
Derived from counting distinct operators and operands within each function:
- **Volume** — `N × log₂(η)` where `N` is total tokens and `η` is distinct tokens
- **Difficulty** — `(η₁ / 2) × (N₂ / η₂)` where `η₁` is distinct operators, `N₂` is total operands, and `η₂` is distinct operands
- **Effort** — `Volume × Difficulty`
- **Time** — `Effort / 18` (estimated time to implement, in seconds)

### Per-File Aggregates
- **avg_complexity_per_function** — average cyclomatic complexity across all functions
- **max_complexity** — highest complexity found in any function
- **max_nesting_depth** — deepest nesting found in any function
- **avg_nesting_depth** — average nesting depth across all functions
- **max_function_lines** — longest function in lines
- **total_function_lines** — sum of all function line counts
- **avg/max_halstead_volume** — average and maximum Halstead volume
- **avg/max_halstead_difficulty** — average and maximum Halstead difficulty
- **avg/max_halstead_effort** — average and maximum Halstead effort
- **avg/max_halstead_time** — average and maximum estimated implementation time

### Project-Level Summary
When analyzing multiple files, the JSON and pretty output include a top-level summary aggregating statistics across all successfully analyzed files with functions.

## Supported File Extensions

| Language | Extensions |
|---|---|
| Rust | `.rs` |
| Python | `.py` |
| JavaScript | `.js`, `.jsx` |
| C | `.c`, `.h` |

Files with unsupported extensions are silently skipped.

## Testing

Run the full test suite:

```bash
cargo test
```

The suite includes:
- **Unit tests** for each language analyzer and the cognitive metrics module (32 tests)
- **Integration tests** that exercise the CLI against fixture files (7 tests)

## Architecture

Rubik uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) to parse source code into ASTs. Each language has a dedicated analyzer that walks the AST to find function boundaries and count decision points. A shared `cognitive` module computes nesting depth and Halstead metrics for every function. A shared dispatcher routes files to the correct analyzer based on extension.

## License

MIT
