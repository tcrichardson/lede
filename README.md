# Rubik

A fast CLI tool that computes cyclomatic code complexity for Rust, Python, and JavaScript source files. It reports complexity per function (including closures and anonymous functions) and per file.

## Features

- **Multi-language support:** Rust, Python, and JavaScript/JSX
- **Per-function & per-file reporting:** See complexity at every level
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
src/main.rs (total complexity: 5, total lines: 42, functions: 1)
+----------+-------+------------+------------+
| Function | Lines | Line Range | Complexity |
+==========+=======+============+============+
| main     | 20    | 16-35      | 5          |
+----------+-------+------------+------------+
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
        "complexity": 5
      }
    ]
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
- **Unit tests** for each language analyzer (16 tests)
- **Integration tests** that exercise the CLI against fixture files (5 tests)

## Architecture

Rubik uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) to parse source code into ASTs. Each language has a dedicated analyzer that walks the AST to find function boundaries and count decision points. A shared dispatcher routes files to the correct analyzer based on extension.

## License

MIT
