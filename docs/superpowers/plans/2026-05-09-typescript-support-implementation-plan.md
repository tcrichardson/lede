# TypeScript Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add TypeScript and TSX file analysis to Rubik using the `tree-sitter-typescript` parser.

**Architecture:** A single `TypeScriptAnalyzer` struct implementing `LanguageAnalyzer` handles both `.ts` and `.tsx` files by selecting the appropriate tree-sitter grammar at parse time. Config (function kinds, decision points, operators, operands, name extraction) mirrors the existing JavaScript analyzer.

**Tech Stack:** Rust, tree-sitter, tree-sitter-typescript, Cargo

---

## File Structure

| File | Responsibility |
|------|--------------|
| `Cargo.toml` | Add `tree-sitter-typescript = "0.23"` dependency |
| `src/language/typescript.rs` | New `TypeScriptAnalyzer` implementing `LanguageAnalyzer` |
| `src/language/mod.rs` | Add `pub mod typescript;` |
| `src/analyzer.rs` | Add `TypeScriptAnalyzer` to `ANALYZERS` array |
| `tests/fixtures/typescript_sample.ts` | Fixture file for integration tests |
| `tests/integration_test.rs` | Add TS fixture test and update directory scan assertion |
| `README.md` | Add TypeScript to supported languages and extensions tables |

---

### Task 1: Add dependency

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add `tree-sitter-typescript` dependency**

Add the following line to the `[dependencies]` section of `Cargo.toml`, after `tree-sitter-rust = "0.23"`:

```toml
tree-sitter-typescript = "0.23"
```

- [ ] **Step 2: Verify dependency resolves**

Run: `cargo check`
Expected: Completes successfully (downloads and compiles the new dependency).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "deps: add tree-sitter-typescript"
```

---

### Task 2: Create TypeScript analyzer module

**Files:**
- Create: `src/language/typescript.rs`

- [ ] **Step 1: Write the TypeScript analyzer**

Create `src/language/typescript.rs` with the following content:

```rust
use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};
use std::path::Path;

pub struct TypeScriptAnalyzer;

const FUNCTION_KINDS: &[&str] = &[
    "function_declaration",
    "function_expression",
    "arrow_function",
    "method_definition",
];
const CLOSURE_KINDS: &[&str] = &[
    "function_expression",
    "arrow_function",
];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "while_statement",
    "do_statement",
    "catch_clause",
    "ternary_expression",
    "switch_case",
    "switch_default",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%", "**",
    "==", "!=", "===", "!==", "<", ">", "<=", ">=",
    "&&", "||", "!", "??", "?.",
    "=", "+=", "-=", "*=", "/=", "%=", "**=",
    "&", "|", "^", "<<", ">>", ">>>", "~",
    "++", "--",
    ".", ":", "=>",
    "return_statement", "yield", "await",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "number", "string", "true", "false", "null", "undefined",
];

impl LanguageAnalyzer for TypeScriptAnalyzer {
    fn can_analyze(&self, path: &Path) -> bool {
        path.extension().map_or(false, |e| e == "ts" || e == "tsx")
    }

    fn language_name(&self) -> &'static str {
        "TS"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            closure_kinds: CLOSURE_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            match_case_kinds: &[],
            skip_childless_nodes: false,
        }
    }
}

fn extract_name(node: Node, source: &str) -> String {
    if node.kind() == "arrow_function" || node.kind() == "function_expression" {
        return format!("<closure>@line {}", node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "property_identifier" {
            return source[child.start_byte()..child.end_byte()].to_string();
        }
    }
    format!("<anon>@line {}", node.start_position().row + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let source = "function foo() { if (x) {} }";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else() {
        let source = "function bar() { if (x) {} else if (y) {} else {} }";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + else-if 1
    }

    #[test]
    fn test_switch() {
        let source = "function baz() { switch(x) { case 1: break; case 2: break; default: break; } }";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 cases
    }

    #[test]
    fn test_arrow_function_included() {
        let source = "const f = (x) => x > 0 ? 1 : 0;";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, true).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].name.starts_with("<closure>"));
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_arrow_function_excluded_by_default() {
        let source = "const f = (x) => x > 0 ? 1 : 0;";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_try_catch() {
        let source = "function err() { try {} catch (a) { if (b) {} } }";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + catch 1 + if 1
    }

    #[test]
    fn test_boolean_ops() {
        let source = "function b() { return a && b || c; }";
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }

    #[test]
    fn test_type_annotations_ignored() {
        let source = r#"function greet(name: string): string {
            if (name) {
                return "hello " + name;
            }
            return "hello";
        }"#;
        let analyzer = TypeScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "greet");
        assert_eq!(result[0].complexity, 2); // base 1 + if 1
    }
}
```

- [ ] **Step 2: Register the module**

Modify `src/language/mod.rs`. Add `pub mod typescript;` after `pub mod rust;`:

```rust
pub mod c;
pub mod javascript;
pub mod python;
pub mod rust;
pub mod typescript;
```

- [ ] **Step 3: Register the analyzer**

Modify `src/analyzer.rs`. Import `TypeScriptAnalyzer` and add it to the `ANALYZERS` array:

Replace the existing imports with:
```rust
use crate::{
    FileResult,
    language::{c::CAnalyzer, javascript::JavaScriptAnalyzer, python::PythonAnalyzer, rust::RustAnalyzer, typescript::TypeScriptAnalyzer, LanguageAnalyzer},
};
```

Replace the existing `ANALYZERS` array with:
```rust
static ANALYZERS: &[&dyn LanguageAnalyzer] = &[
    &RustAnalyzer,
    &PythonAnalyzer,
    &JavaScriptAnalyzer,
    &TypeScriptAnalyzer,
    &CAnalyzer,
];
```

- [ ] **Step 4: Run unit tests**

Run: `cargo test typescript::`
Expected: All 7 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/language/typescript.rs src/language/mod.rs src/analyzer.rs
git commit -m "feat: add TypeScript analyzer"
```

---

### Task 3: Add fixture file and integration test

**Files:**
- Create: `tests/fixtures/typescript_sample.ts`
- Modify: `tests/integration_test.rs`

- [ ] **Step 1: Create the fixture**

Create `tests/fixtures/typescript_sample.ts`:

```typescript
function simple() {
    console.log("hello");
}

function withIf(x: number): number {
    if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

function withSwitch(x: number): number {
    switch (x) {
        case 1: return 1;
        case 2: return 2;
        default: return 0;
    }
}

function nested() {
    const f = (y: number) => y > 0 ? 1 : 0;
}
```

- [ ] **Step 2: Add integration test**

Add the following test to `tests/integration_test.rs`, after `test_c_fixture_json` (before `test_invalid_file_skips`):

```rust
#[test]
fn test_typescript_fixture_json() {
    let output = rubik()
        .arg("tests/fixtures/typescript_sample.ts")
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.summary.files_analyzed, 1);
    let file = &parsed.files[0];
    assert!(file.path.to_string_lossy().contains("typescript_sample.ts"));
    let names: Vec<&str> = file.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"simple"));
    assert!(names.contains(&"withIf"));
    assert!(names.contains(&"withSwitch"));
    assert!(names.contains(&"nested"));
    assert!(file.functions.iter().any(|f| f.halstead_effort > 0.0));
    assert!(file.avg_halstead_effort > 0.0);
    assert!(file.avg_halstead_volume >= 0.0);
    assert!(parsed.summary.total_functions > 0);
    assert!(parsed.summary.total_lines > 0);
}
```

- [ ] **Step 3: Update directory scan assertion**

In `test_directory_scan`, update the `files_analyzed` assertion from `>= 4` to `>= 5`:

```rust
assert!(parsed.summary.files_analyzed >= 5);
```

And add an assertion for the TypeScript fixture:

After:
```rust
assert!(paths.iter().any(|p| p.contains("c_sample.c")));
```

Add:
```rust
assert!(paths.iter().any(|p| p.contains("typescript_sample.ts")));
```

- [ ] **Step 4: Run integration tests**

Run: `cargo test --test integration_test`
Expected: All integration tests pass, including the new `test_typescript_fixture_json`.

- [ ] **Step 5: Commit**

```bash
git add tests/fixtures/typescript_sample.ts tests/integration_test.rs
git commit -m "test: add TypeScript fixture and integration tests"
```

---

### Task 4: Update README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update the features list**

In the Features section, change:
```
- **Multi-language support:** Rust, Python, JavaScript/JSX, and C
```
to:
```
- **Multi-language support:** Rust, Python, JavaScript/JSX, TypeScript/TSX, and C
```

- [ ] **Step 2: Update the decision point table**

Add a "TypeScript" column to the decision point table. Insert it between JavaScript and C. All TypeScript rows mirror JavaScript exactly:

| Decision Point | Rust | Python | JavaScript | TypeScript | C |
|---|---|---|---|---|---|
| `if` / `elif` | `if_expression` | `if_statement`, `elif_clause` | `if_statement` | `if_statement` | `if_statement` |
| `match` / `switch` / `case` | `match_expression` (per arm) | `match_statement` (per case) | `switch_statement` (per case) | `switch_statement` (per case) | `case_statement` |
| `for` | `for_expression` | `for_statement` | `for_statement` | `for_statement` | `for_statement` |
| `while` | `while_expression` | `while_statement` | `while_statement`, `do_statement` | `while_statement`, `do_statement` | `while_statement`, `do_statement` |
| `loop` | `loop_expression` | — | — | — | — |
| `try` / `except` / `catch` | `try_expression` | `except_clause` | `catch_clause` | `catch_clause` | — |
| `&&` / `\|\|` | binary operators | `and` / `or` | binary operators | binary operators | binary operators |
| Ternary | — | `conditional_expression` | `ternary_expression` | `ternary_expression` | `conditional_expression` |
| Lambda / Closure* | `closure_expression` | `lambda` | `arrow_function` | `arrow_function` | — |

- [ ] **Step 3: Update the supported file extensions table**

Add a TypeScript row after JavaScript:

| Language | Extensions |
|---|---|
| Rust | `.rs` |
| Python | `.py` |
| JavaScript | `.js`, `.jsx` |
| TypeScript | `.ts`, `.tsx` |
| C | `.c`, `.h` |

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs: add TypeScript to README"
```

---

## Self-Review

### Spec coverage
- [x] `tree-sitter-typescript` dependency → Task 1
- [x] `src/language/typescript.rs` with `TypeScriptAnalyzer` → Task 2
- [x] Grammar selection (`.ts` vs `.tsx`) → Task 2 (Step 1, `parser()` method)
- [x] Shared config matching JS → Task 2 (Step 1)
- [x] Module registration (`mod.rs`) → Task 2 (Step 2)
- [x] Analyzer registration (`analyzer.rs`) → Task 2 (Step 3)
- [x] Unit tests (simple, if/else, switch, arrow included/excluded, try/catch, boolean ops, type annotations) → Task 2 (Step 1)
- [x] Fixture file → Task 3 (Step 1)
- [x] Integration test → Task 3 (Step 2)
- [x] Directory scan update → Task 3 (Step 3)
- [x] README updates → Task 4

### Placeholder scan
No TBD, TODO, "implement later", or vague steps found. All code is complete and exact.

### Type consistency
- `TypeScriptAnalyzer` follows the exact `LanguageAnalyzer` trait pattern used by `JavaScriptAnalyzer`, `CAnalyzer`, etc.
- `parser()` returns `Result<Parser, String>` matching all existing analyzers.
- `config()` returns `LanguageConfig` with identical field names and types.
