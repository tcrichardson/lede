# C Language Support Design

## Summary
Add support for analyzing C source and header files (`.c`, `.h`) using the `tree-sitter-c` parser, following the same architecture as the existing Rust, Python, and JavaScript analyzers.

## Architecture

### New Components
- `src/language/c.rs` — `CAnalyzer` implementing `LanguageAnalyzer`
- `tests/fixtures/c_sample.c` — fixture file for integration tests

### Modified Components
- `Cargo.toml` — add `tree-sitter-c = "0.23"` dependency
- `src/language/mod.rs` — add `pub mod c;`
- `src/analyzer.rs` — add `CAnalyzer` to the `ANALYZERS` array
- `tests/integration_test.rs` — add C fixture test and update directory scan assertion

## Data Flow
Identical to existing analyzers:
1. `analyze_path` → `analyze_file`
2. `CAnalyzer::can_analyze` matches `.c` and `.h` extensions
3. `CAnalyzer::analyze` parses the source with `tree-sitter-c`
4. `collect_functions` walks the AST, identifying `function_definition` nodes
5. For each function, compute complexity (1 + decision count), nesting depth, and Halstead metrics
6. Return `Vec<FunctionComplexity>`

## C Grammar Node Mappings

Based on tree-sitter-c grammar:

### Function Kinds
- `function_definition`

### Decision Kinds
- `if_statement`
- `for_statement`
- `while_statement`
- `do_statement`
- `case_statement`
- `conditional_expression`

### Operator Kinds
- `+`, `-`, `*`, `/`, `%`
- `==`, `!=`, `<`, `>`, `<=`, `>=`
- `&&`, `||`, `!`
- `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- `&`, `|`, `^`, `<<`, `>>`, `~`
- `.`, `->`, `:`
- `return_statement`, `break_statement`, `continue_statement`, `goto_statement`

### Operand Kinds
- `identifier`
- `number_literal`
- `string_literal`
- `char_literal`
- `true`
- `false`
- `null`

## Error Handling
Parse failures and trees containing errors return `Err("Failed to parse C source")`, matching existing analyzer behavior. Unsupported extensions are skipped silently.

## Testing
- Unit tests in `src/language/c.rs` covering: simple function, `if`/`else`, `switch`/`case`, `for`/`while`/`do`, ternary `?:`, and boolean operators (`&&`/`||`)
- Integration test `test_c_fixture_json` in `tests/integration_test.rs`
- Update `test_directory_scan` to assert the C fixture is discovered
