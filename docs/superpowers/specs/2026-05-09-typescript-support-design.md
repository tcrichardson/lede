# TypeScript Support Design

## Summary
Add support for analyzing TypeScript and TSX source files (`.ts`, `.tsx`) using the `tree-sitter-typescript` parser, following the same architecture as the existing language analyzers.

## Architecture

### New Components
- `src/language/typescript.rs` — `TypeScriptAnalyzer` implementing `LanguageAnalyzer`
- `tests/fixtures/typescript_sample.ts` — fixture file for integration tests

### Modified Components
- `Cargo.toml` — add `tree-sitter-typescript = "0.23"` dependency
- `src/language/mod.rs` — add `pub mod typescript;`
- `src/analyzer.rs` — add `TypeScriptAnalyzer` to the `ANALYZERS` array
- `tests/integration_test.rs` — add TypeScript fixture test and update directory scan assertion
- `README.md` — add TypeScript to supported languages and file extensions tables

## Data Flow
Identical to existing analyzers:
1. `analyze_path` → `analyze_file`
2. `TypeScriptAnalyzer::can_analyze` matches `.ts` and `.tsx` extensions
3. `TypeScriptAnalyzer::analyze` parses the source with the appropriate `tree-sitter-typescript` grammar
4. `collect_functions` walks the AST, identifying function nodes
5. For each function, compute complexity (1 + decision count), nesting depth, and Halstead metrics
6. Return `Vec<FunctionComplexity>`

## Grammar Node Mappings

Because TS and TSX are two distinct tree-sitter dialects, the analyzer selects the grammar based on file extension:
- `.ts` → `tree_sitter_typescript::LANGUAGE_TYPESCRIPT`
- `.tsx` → `tree_sitter_typescript::LANGUAGE_TSX`

Both dialects share identical runtime syntax, so the `LanguageConfig` is the same for both.

### Function Kinds
- `function_declaration`
- `function_expression`
- `arrow_function`
- `method_definition`

### Closure Kinds (excluded by default)
- `function_expression`
- `arrow_function`

### Decision Kinds
- `if_statement`
- `for_statement`
- `while_statement`
- `do_statement`
- `catch_clause`
- `ternary_expression`
- `switch_case`
- `switch_default`

### Operator Kinds
- `+`, `-`, `*`, `/`, `%`, `**`
- `==`, `!=`, `===`, `!==`, `<`, `>`, `<=`, `>=`
- `&&`, `||`, `!`, `??`, `?.`
- `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=`
- `&`, `|`, `^`, `<<`, `>>`, `>>>`, `~`
- `++`, `--`
- `.`, `:`, `=>`
- `return_statement`, `yield`, `await`

### Operand Kinds
- `identifier`
- `number`
- `string`
- `true`
- `false`
- `null`
- `undefined`

## Name Extraction
- `arrow_function` and `function_expression` → `<closure>@line {line}`
- Otherwise, search children for `identifier` or `property_identifier`
- Fallback → `<anon>@line {line}`

## Error Handling
Parse failures and trees containing errors return `Err("Failed to parse TS source")`, matching existing analyzer behavior. Unsupported extensions are skipped silently.

## Testing
- Unit tests in `src/language/typescript.rs` covering: simple function, `if`/`else`, `switch`/`case`, arrow function included/excluded, `try`/`catch`, and boolean operators (`&&`/`||`)
- Integration test `test_typescript_fixture_json` in `tests/integration_test.rs`
- Update `test_directory_scan` to assert the TypeScript fixture is discovered
