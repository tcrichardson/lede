# C Language Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `CAnalyzer` that parses `.c` and `.h` files with tree-sitter-c and reports per-function complexity, nesting depth, and Halstead metrics.

**Architecture:** A new `src/language/c.rs` module implementing the `LanguageAnalyzer` trait, wired into `analyzer.rs` alongside the existing analyzers. The module mirrors the structure of `src/language/rust.rs` with C-specific tree-sitter node kind constants.

**Tech Stack:** Rust, tree-sitter, tree-sitter-c v0.23

---

### Task 1: Add tree-sitter-c dependency

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add dependency**

Add `tree-sitter-c = "0.23"` to the `[dependencies]` section of `Cargo.toml`, aligned with the other tree-sitter language crates.

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
comfy-table = "7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tree-sitter = "0.24"
tree-sitter-c = "0.23"
tree-sitter-javascript = "0.23"
tree-sitter-python = "0.23"
tree-sitter-rust = "0.23"
walkdir = "2"
```

- [ ] **Step 2: Verify dependency resolves**

Run: `cargo check`
Expected: Successful dependency resolution (no `tree-sitter-c` code used yet, so it should compile).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "deps: add tree-sitter-c 0.23"
```

---

### Task 2: Create CAnalyzer module

**Files:**
- Create: `src/language/c.rs`

- [ ] **Step 1: Create the module file**

Create `src/language/c.rs` with the following content. It follows the exact pattern of `src/language/rust.rs`, adapted for C grammar node kinds.

```rust
use crate::{FunctionComplexity, language::LanguageAnalyzer};
use tree_sitter::{Node, Parser};

pub struct CAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_definition"];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "else_clause",
    "for_statement",
    "while_statement",
    "do_statement",
    "switch_statement",
    "case_statement",
    "conditional_expression",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%",
    "==", "!=", "<", ">", "<=", ">=",
    "&&", "||", "!",
    "=", "+=", "-=", "*=", "/=", "%=",
    "&", "|", "^", "<<", ">>", "~",
    ".", "->", ":",
    "return_statement", "break_statement", "continue_statement", "goto_statement",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "number_literal", "string_literal", "char_literal",
    "true", "false", "null",
];

impl LanguageAnalyzer for CAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "c" || e == "h")
    }

    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
        parser
            .set_language(&language)
            .map_err(|e| format!("{e:?}"))?;
        let tree = parser.parse(source, None).ok_or("Failed to parse C source")?;
        if tree.root_node().has_error() {
            return Err("Failed to parse C source".to_string());
        }
        let mut functions = Vec::new();
        collect_functions(tree.root_node(), source, &mut functions);
        Ok(functions)
    }
}

fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if FUNCTION_KINDS.contains(&node.kind()) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, DECISION_KINDS, FUNCTION_KINDS);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, OPERATOR_KINDS, OPERAND_KINDS, FUNCTION_KINDS,
        );
        let halstead_effort = halstead_volume * halstead_difficulty;
        let halstead_time = halstead_effort / 18.0;
        functions.push(FunctionComplexity {
            name,
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            lines: node.end_position().row - node.start_position().row + 1,
            complexity,
            nesting_depth,
            halstead_volume,
            halstead_difficulty,
            halstead_effort,
            halstead_time,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions);
    }
}

fn count_decisions(node: Node, source: &str) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if FUNCTION_KINDS.contains(&child.kind()) {
            continue;
        }
        if DECISION_KINDS.contains(&child.kind()) {
            count += 1;
        }
        if crate::complexity::is_boolean_operator(child, source) {
            count += 1;
        }
        count += count_decisions(child, source);
    }
    count
}

fn extract_name(node: Node, source: &str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return source[child.start_byte()..child.end_byte()].to_string();
        }
        // The declarator may be wrapped in pointer_declarator or function_declarator
        let name = find_identifier_in_declarator(child, source);
        if !name.is_empty() {
            return name;
        }
    }
    format!("<anon>@line {}", node.start_position().row + 1)
}

fn find_identifier_in_declarator(node: Node, source: &str) -> String {
    if node.kind() == "identifier" {
        return source[node.start_byte()..node.end_byte()].to_string();
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let name = find_identifier_in_declarator(child, source);
        if !name.is_empty() {
            return name;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let source = "int foo() { if (1) {} }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else() {
        let source = "int bar() { if (x) {} else {} }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 2); // base 1 + if 1
    }

    #[test]
    fn test_switch() {
        let source = r#"
int baz() {
    switch (x) {
        case 1: break;
        case 2: break;
        default: break;
    }
}
"#;
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 cases
    }

    #[test]
    fn test_for_while_do() {
        let source = r#"
int loop() {
    for (;;) {}
    while (1) {}
    do {} while (1);
}
"#;
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + for 1 + while 1 + do 1
    }

    #[test]
    fn test_ternary() {
        let source = "int t() { return x > 0 ? 1 : 0; }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_boolean_ops() {
        let source = "int b() { return a && b || c; }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }
}
```

- [ ] **Step 2: Register the module**

Modify `src/language/mod.rs` to add `pub mod c;`:

```rust
pub mod c;
pub mod javascript;
pub mod python;
pub mod rust;
```

- [ ] **Step 3: Wire into the analyzer registry**

Modify `src/analyzer.rs` to import and register `CAnalyzer`:

Add to imports:
```rust
use crate::language::c::CAnalyzer;
```

Add to `ANALYZERS`:
```rust
static ANALYZERS: &[&dyn LanguageAnalyzer] = &[
    &RustAnalyzer,
    &PythonAnalyzer,
    &JavaScriptAnalyzer,
    &CAnalyzer,
];
```

- [ ] **Step 4: Run unit tests**

Run: `cargo test language::c -- --nocapture`
Expected: All 6 unit tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/language/c.rs src/language/mod.rs src/analyzer.rs
git commit -m "feat: add C language analyzer"
```

---

### Task 3: Create C fixture file

**Files:**
- Create: `tests/fixtures/c_sample.c`

- [ ] **Step 1: Write fixture**

Create `tests/fixtures/c_sample.c`:

```c
#include <stdio.h>

void simple() {
    printf("hello");
}

int withIf(int x) {
    if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

int withSwitch(int x) {
    switch (x) {
        case 1: return 1;
        case 2: return 2;
        default: return 0;
    }
}

void nested() {
    for (int i = 0; i < 10; i++) {
        if (i % 2 == 0) {
            continue;
        }
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add tests/fixtures/c_sample.c
git commit -m "test: add C fixture file"
```

---

### Task 4: Add integration tests for C

**Files:**
- Modify: `tests/integration_test.rs`

- [ ] **Step 1: Add C fixture JSON test**

Add the following test to `tests/integration_test.rs` after the `test_js_fixture_json` test:

```rust
#[test]
fn test_c_fixture_json() {
    let output = rubik()
        .arg("tests/fixtures/c_sample.c")
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.summary.files_analyzed, 1);
    let file = &parsed.files[0];
    assert!(file.path.to_string_lossy().contains("c_sample.c"));
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

- [ ] **Step 2: Update directory scan test**

In `test_directory_scan`, add an assertion for the C fixture:

Find this block:
```rust
    assert!(paths.iter().any(|p| p.contains("rust_sample.rs")));
    assert!(paths.iter().any(|p| p.contains("python_sample.py")));
    assert!(paths.iter().any(|p| p.contains("js_sample.js")));
    assert!(parsed.summary.files_analyzed >= 3);
```

Replace with:
```rust
    assert!(paths.iter().any(|p| p.contains("rust_sample.rs")));
    assert!(paths.iter().any(|p| p.contains("python_sample.py")));
    assert!(paths.iter().any(|p| p.contains("js_sample.js")));
    assert!(paths.iter().any(|p| p.contains("c_sample.c")));
    assert!(parsed.summary.files_analyzed >= 4);
```

- [ ] **Step 3: Run integration tests**

Run: `cargo test --test integration_test`
Expected: All integration tests pass, including the new C test.

- [ ] **Step 4: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add C integration tests"
```

---

### Task 5: Final verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 2: Manual CLI check**

Run: `cargo run -- tests/fixtures/c_sample.c --format pretty`
Expected: Table output showing `simple`, `withIf`, `withSwitch`, and `nested` with complexity, nesting depth, and Halstead metrics.

- [ ] **Step 3: Commit**

```bash
git commit -m "feat: complete C language support"
```

---

## Self-Review Checklist

1. **Spec coverage:**
   - `CAnalyzer` module implementing `LanguageAnalyzer` — Task 2
   - `.c` and `.h` extension matching — Task 2
   - tree-sitter-c parsing with error handling — Task 2
   - Complexity, nesting depth, Halstead metrics — Task 2
   - Unit tests for simple, if/else, switch, loops, ternary, boolean ops — Task 2
   - C fixture file — Task 3
   - Integration test for C fixture JSON output — Task 4
   - Updated directory scan test — Task 4

2. **Placeholder scan:** No TBD, TODO, or vague steps. Every step includes exact file paths and code.

3. **Type consistency:** `CAnalyzer` follows the same trait and struct patterns as existing analyzers. `tree_sitter_c::LANGUAGE.into()` matches the pattern used by other analyzers. Function names and signatures are consistent.
