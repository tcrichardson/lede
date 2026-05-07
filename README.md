# Rubik

A fast CLI tool that computes cyclomatic code complexity and cognitive load for Rust, Python, and JavaScript source files. It reports complexity and cognitive metrics per function (including closures and anonymous functions) and per file.

## Features

- **Multi-language support:** Rust, Python, and JavaScript/JSX
- **Cyclomatic complexity:** Classic decision-point counting per function and file
- **Cognitive load scoring:** Combines nesting depth, Halstead metrics, and complexity into a single readability score
- **Per-function & per-file reporting:** See complexity and cognitive metrics at every level
- **Closure inclusion:** Anonymous functions, lambdas, and arrow functions are counted separately
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

### CLI Options

```
Usage: rubik [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to a file or directory to analyze

Options:
  -f, --format <FORMAT>  Output format: pretty or json [default: pretty]
  -h, --help             Print help
  -V, --version          Print version
```

## Example Output

### Pretty format (default)

```
### src/main.rs (total complexity: 5, total lines: 42, functions: 1, avg cognitive load: 12.34, max nesting: 2, avg Halstead volume: 45.60)

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Cognitive Load |
|----------|-------|------------|------------|---------|--------------|------------|----------------|
| main     | 20    | 16-35      | 5          | 2       | 45.60        | 3.20       | 12.34          |
```

### JSON format

```json
[
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
        "cognitive_load": 12.34
      }
    ],
    "avg_cognitive_load": 12.34,
    "max_nesting_depth": 2,
    "avg_halstead_volume": 45.60
  }
]
```

## How Complexity is Calculated

For each function or closure, complexity starts at **1** and increments by **1** for each decision point:

| Decision Point | Rust | Python | JavaScript |
|---|---|---|---|
| `if` / `elif` | `if_expression` | `if_statement`, `elif_clause` | `if_statement` |
| `match` / `switch` / `case` | `match_expression` (per arm) | `match_statement` (per case) | `switch_statement` (per case) |
| `for` | `for_expression` | `for_statement` | `for_statement` |
| `while` | `while_expression` | `while_statement` | `while_statement`, `do_statement` |
| `loop` | `loop_expression` | — | — |
| `try` / `except` / `catch` | `try_expression` | `except_clause` | `catch_clause` |
| `&&` / `\|\|` | binary operators | `and` / `or` | binary operators |
| Ternary | — | `conditional_expression` | `ternary_expression` |
| Lambda / Closure | `closure_expression` | `lambda` | `arrow_function` |

Per-file complexity is the sum of all function complexities in that file.

## How Cognitive Load is Calculated

Cognitive load is a composite score designed to estimate how difficult a function is to understand. It combines three metrics:

### Nesting Depth
The maximum depth of control-flow block nesting inside a function. For example, an `if` inside a `for` loop has a nesting depth of 2. Nested functions are not counted.

### Halstead Metrics
Derived from counting distinct operators and operands within each function:
- **Volume** — `N × log₂(η)` where `N` is total tokens and `η` is distinct tokens
- **Difficulty** — `(η₁ / 2) × (N₂ / η₂)` where `η₁` is distinct operators, `N₂` is total operands, and `η₂` is distinct operands

### Cognitive Load Formula
```
cognitive_load = (halstead_volume / 100)
               + (max_nesting_depth × 5)
               + (halstead_difficulty / 10)
```

Per-file aggregates include:
- **avg_cognitive_load** — average across all functions in the file
- **max_nesting_depth** — deepest nesting found in any function
- **avg_halstead_volume** — average Halstead volume across all functions

## Supported File Extensions

| Language | Extensions |
|---|---|
| Rust | `.rs` |
| Python | `.py` |
| JavaScript | `.js`, `.jsx` |

Files with unsupported extensions are silently skipped.

## Testing

Run the full test suite:

```bash
cargo test
```

The suite includes:
- **Unit tests** for each language analyzer and the cognitive metrics module (20 tests)
- **Integration tests** that exercise the CLI against fixture files (5 tests)

## Architecture

Rubik uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) to parse source code into ASTs. Each language has a dedicated analyzer that walks the AST to find function boundaries and count decision points. A shared `cognitive` module computes nesting depth and Halstead metrics for every function. A shared dispatcher routes files to the correct analyzer based on extension.

## License

MIT
