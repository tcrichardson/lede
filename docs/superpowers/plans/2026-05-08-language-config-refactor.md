# Language Config Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Introduce a `LanguageConfig` struct and extend the `LanguageAnalyzer` trait so the 10-parameter `collect_functions` function shrinks to 4 parameters and the boilerplate `analyze()` body is shared via a trait default.

**Architecture:** Add `LanguageConfig` to `language/mod.rs` bundling all per-language constants and function pointers. Extend `LanguageAnalyzer` with required `parser()` and `config()` methods and an optional `language_name()` method, then add a default `analyze()` that uses them. All four language files drop their explicit `analyze()` implementation and local `collect_functions` wrapper.

**Tech Stack:** Rust, tree-sitter 0.24, tree-sitter-{rust,javascript,python,c} 0.23

---

## File Map

- Modify: `src/language/mod.rs` — add `LanguageConfig`, extend `LanguageAnalyzer` trait, update `collect_functions`
- Modify: `src/language/rust.rs` — add `parser`, `config`, `language_name`; remove `analyze` and wrapper
- Modify: `src/language/javascript.rs` — add `parser`, `config`, `language_name`; remove `analyze` and wrapper
- Modify: `src/language/c.rs` — add `parser`, `config`, `language_name`; remove `analyze` and wrapper
- Modify: `src/language/python.rs` — add `parser`, `config`, `language_name`; remove `analyze` and wrapper

> **Note on Python:** Although the spec mentioned Python would keep an explicit `analyze()` override, this is not needed. `count_decisions_for_python` has the same signature as the generic `count_decisions` and can be stored in `config.count_decisions_fn`. The trait default handles Python identically to the other languages.

---

### Task 1: Verify baseline

**Files:** (none modified)

- [ ] **Step 1: Run the full test suite**

```bash
cargo test
```

Expected: all tests pass. If any fail, do not proceed — investigate first.

---

### Task 2: Add `LanguageConfig` struct (non-breaking)

**Files:**
- Modify: `src/language/mod.rs`

- [ ] **Step 1: Add `LanguageConfig` struct and the `Parser` import to `language/mod.rs`**

Add `use tree_sitter::Parser;` to the existing imports at the top, then add the struct immediately after the `LanguageAnalyzer` trait definition. Replace the entire file with:

```rust
use crate::FunctionComplexity;
use std::path::Path;
use tree_sitter::{Node, Parser};

pub struct LanguageConfig {
    pub function_kinds: &'static [&'static str],
    pub decision_kinds: &'static [&'static str],
    pub operator_kinds: &'static [&'static str],
    pub operand_kinds: &'static [&'static str],
    pub extract_name: fn(Node, &str) -> String,
    pub count_decisions_fn: fn(Node, &str, &[&str], &[&str]) -> u32,
    pub require_children: bool,
}

pub trait LanguageAnalyzer: Send + Sync {
    fn can_analyze(&self, path: &Path) -> bool;
    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String>;
}

/// Generic function collector. Each language analyzer calls this with its own configuration.
pub fn collect_functions(
    node: Node,
    source: &str,
    functions: &mut Vec<FunctionComplexity>,
    function_kinds: &[&str],
    decision_kinds: &[&str],
    operator_kinds: &[&str],
    operand_kinds: &[&str],
    extract_name: fn(Node, &str) -> String,
    count_decisions_fn: fn(Node, &str, &[&str], &[&str]) -> u32,
    require_children: bool,
) {
    if function_kinds.contains(&node.kind()) && (!require_children || node.child_count() > 0) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions_fn(node, source, decision_kinds, function_kinds);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, decision_kinds, function_kinds);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, operator_kinds, operand_kinds, function_kinds,
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
        collect_functions(
            child, source, functions,
            function_kinds, decision_kinds, operator_kinds, operand_kinds,
            extract_name, count_decisions_fn, require_children,
        );
    }
}

pub fn count_decisions(
    node: Node,
    source: &str,
    decision_kinds: &[&str],
    function_kinds: &[&str],
) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if function_kinds.contains(&child.kind()) {
            continue;
        }
        if decision_kinds.contains(&child.kind()) {
            count += 1;
        }
        if crate::complexity::is_boolean_operator(child, source) {
            count += 1;
        }
        count += count_decisions(child, source, decision_kinds, function_kinds);
    }
    count
}

pub mod c;
pub mod javascript;
pub mod python;
pub mod rust;
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build
```

Expected: compiles with no errors. The `Parser` import and `LanguageConfig` struct are additive — nothing else changes yet.

- [ ] **Step 3: Commit**

```bash
git add src/language/mod.rs
git commit -m "refactor(language): introduce LanguageConfig struct"
```

---

### Task 3: Update `collect_functions` to accept `&LanguageConfig` and update all call sites

This task changes the `collect_functions` signature across `mod.rs` and all four language files at once — they must all be updated together or the build will fail.

**Files:**
- Modify: `src/language/mod.rs`
- Modify: `src/language/rust.rs`
- Modify: `src/language/javascript.rs`
- Modify: `src/language/c.rs`
- Modify: `src/language/python.rs`

- [ ] **Step 1: Replace `collect_functions` in `src/language/mod.rs` with the config-based version**

Replace only the `collect_functions` function body (leave everything else in the file unchanged):

```rust
pub fn collect_functions(
    node: Node,
    source: &str,
    functions: &mut Vec<FunctionComplexity>,
    config: &LanguageConfig,
) {
    if config.function_kinds.contains(&node.kind())
        && (!config.require_children || node.child_count() > 0)
    {
        let name = (config.extract_name)(node, source);
        let complexity = 1 + (config.count_decisions_fn)(node, source, config.decision_kinds, config.function_kinds);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, config.decision_kinds, config.function_kinds);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, config.operator_kinds, config.operand_kinds, config.function_kinds,
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
        collect_functions(child, source, functions, config);
    }
}
```

- [ ] **Step 2: Update the `collect_functions` wrapper in `src/language/rust.rs`**

Replace the local `collect_functions` function:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        &crate::language::LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
        },
    );
}
```

Also add `LanguageConfig` to the import at the top of `rust.rs`:

```rust
use crate::{FunctionComplexity, language::{LanguageAnalyzer, LanguageConfig}};
```

- [ ] **Step 3: Update the `collect_functions` wrapper in `src/language/javascript.rs`**

Replace the local `collect_functions` function:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        &crate::language::LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
        },
    );
}
```

Also add `LanguageConfig` to the import:

```rust
use crate::{FunctionComplexity, language::{LanguageAnalyzer, LanguageConfig}};
```

- [ ] **Step 4: Update the `collect_functions` wrapper in `src/language/c.rs`**

Replace the local `collect_functions` function:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        &crate::language::LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
        },
    );
}
```

Also add `LanguageConfig` to the import:

```rust
use crate::{FunctionComplexity, language::{LanguageAnalyzer, LanguageConfig}};
```

- [ ] **Step 5: Update the `collect_functions` call in `src/language/python.rs`**

Python's local wrapper currently passes `count_decisions_for_python`. Replace the local `collect_functions` function:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        &crate::language::LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: count_decisions_for_python,
            require_children: true,
        },
    );
}
```

Also add `LanguageConfig` to the import:

```rust
use crate::{FunctionComplexity, language::{LanguageAnalyzer, LanguageConfig}};
```

- [ ] **Step 6: Run the full test suite**

```bash
cargo test
```

Expected: all tests pass. `collect_functions` now takes 4 parameters everywhere.

- [ ] **Step 7: Commit**

```bash
git add src/language/mod.rs src/language/rust.rs src/language/javascript.rs src/language/c.rs src/language/python.rs
git commit -m "refactor(language): update collect_functions to accept &LanguageConfig"
```

---

### Task 4: Extend the trait and eliminate per-language boilerplate

This task adds `parser()`, `config()`, and `language_name()` as trait methods, adds a default `analyze()` to the trait, and removes the now-redundant `analyze()` implementations and local `collect_functions` wrappers from all four language files. All files must be updated together.

**Files:**
- Modify: `src/language/mod.rs`
- Modify: `src/language/rust.rs`
- Modify: `src/language/javascript.rs`
- Modify: `src/language/c.rs`
- Modify: `src/language/python.rs`

- [ ] **Step 1: Replace the `LanguageAnalyzer` trait in `src/language/mod.rs`**

Replace the existing trait definition with the extended version including the three new methods and the default `analyze()`:

```rust
pub trait LanguageAnalyzer: Send + Sync {
    fn can_analyze(&self, path: &Path) -> bool;
    fn config(&self) -> LanguageConfig;
    fn parser(&self) -> Result<Parser, String>;
    fn language_name(&self) -> &'static str {
        "source"
    }
    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = self.parser()?;
        let config = self.config();
        let msg = format!("Failed to parse {} source", self.language_name());
        let tree = parser.parse(source, None).ok_or_else(|| msg.clone())?;
        if tree.root_node().has_error() {
            return Err(msg);
        }
        let mut functions = Vec::new();
        collect_functions(tree.root_node(), source, &mut functions, &config);
        Ok(functions)
    }
}
```

- [ ] **Step 2: Replace `src/language/rust.rs` entirely**

```rust
use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};

pub struct RustAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_item", "closure_expression"];
const DECISION_KINDS: &[&str] = &[
    "if_expression",
    "if_let_expression",
    "for_expression",
    "while_expression",
    "while_let_expression",
    "loop_expression",
    "try_expression",
    "match_arm",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%", "&&", "||", "!", "==", "!=", "<", ">", "<=", ">=",
    "=", "+=", "-=" , "*=", "/=", "%=",
    "&", "|", "^", "<<", ">>",
    ".", "..", "...", "->", "=>",
    "return_expression", "break_expression", "continue_expression",
    "await_expression", "try_expression",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "integer_literal", "float_literal", "string_literal",
    "char_literal", "bool_literal", "self",
];

impl LanguageAnalyzer for RustAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "rs")
    }

    fn language_name(&self) -> &'static str {
        "Rust"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
        }
    }
}

fn extract_name(node: Node, source: &str) -> String {
    if node.kind() == "closure_expression" {
        return format!("<closure>@line {}", node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
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
        let source = "fn foo() { if true {} }";
        let analyzer = RustAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else_if() {
        let source = r#"
fn bar() {
    if x {}
    else if y {}
    else {}
}
"#;
        let analyzer = RustAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + else-if 1
    }

    #[test]
    fn test_match() {
        let source = r#"
fn baz() {
    match x {
        1 => {}
        2 => {}
        _ => {}
    }
}
"#;
        let analyzer = RustAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 arms
    }

    #[test]
    fn test_closure() {
        let source = "fn outer() { let f = |x| if x > 0 { 1 } else { 0 }; }";
        let analyzer = RustAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 2);
        let closure = result.iter().find(|f| f.name.starts_with("<closure>")).unwrap();
        assert_eq!(closure.complexity, 2);
    }

    #[test]
    fn test_boolean_ops() {
        let source = "fn b() { a && b || c; }";
        let analyzer = RustAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }
}
```

- [ ] **Step 3: Replace `src/language/javascript.rs` entirely**

```rust
use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};

pub struct JavaScriptAnalyzer;

const FUNCTION_KINDS: &[&str] = &[
    "function_declaration",
    "function_expression",
    "arrow_function",
    "method_definition",
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
    "=", "+=", "-=" , "*=", "/=", "%=", "**=",
    "&", "|", "^", "<<", ">>", ">>>", "~",
    "++", "--",
    ".", ":", "=>",
    "return_statement", "yield", "await",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "number", "string", "true", "false", "null", "undefined",
];

impl LanguageAnalyzer for JavaScriptAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "js" || e == "jsx")
    }

    fn language_name(&self) -> &'static str {
        "JS"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_javascript::LANGUAGE.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
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
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else() {
        let source = "function bar() { if (x) {} else if (y) {} else {} }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + else-if 1
    }

    #[test]
    fn test_switch() {
        let source = "function baz() { switch(x) { case 1: break; case 2: break; default: break; } }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 cases
    }

    #[test]
    fn test_arrow_function() {
        let source = "const f = (x) => x > 0 ? 1 : 0;";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].name.starts_with("<closure>"));
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_try_catch() {
        let source = "function err() { try {} catch (a) { if (b) {} } }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + catch 1 + if 1
    }

    #[test]
    fn test_boolean_ops() {
        let source = "function b() { return a && b || c; }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }
}
```

- [ ] **Step 4: Replace `src/language/c.rs` entirely**

```rust
use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};

pub struct CAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_definition"];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "while_statement",
    "do_statement",
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
];

impl LanguageAnalyzer for CAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "c" || e == "h")
    }

    fn language_name(&self) -> &'static str {
        "C"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: crate::language::count_decisions,
            require_children: false,
        }
    }
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
        assert_eq!(result[0].complexity, 4); // base 1 + 2 cases + 1 default
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

    #[test]
    fn test_pointer_return_name() {
        let source = "int *foo() { return 0; }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
    }

    #[test]
    fn test_parse_error() {
        let source = "int foo() {";
        let analyzer = CAnalyzer;
        assert!(analyzer.analyze(source).is_err());
    }

    #[test]
    fn test_can_analyze_header() {
        let analyzer = CAnalyzer;
        assert!(analyzer.can_analyze(std::path::Path::new("foo.h")));
    }
}
```

- [ ] **Step 5: Replace `src/language/python.rs` entirely**

> Note: Python does NOT need an explicit `analyze()` override. `count_decisions_for_python` is stored in `config.count_decisions_fn`, and the trait default calls it through the config.

```rust
use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};

pub struct PythonAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_definition", "lambda"];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "elif_clause",
    "for_statement",
    "while_statement",
    "except_clause",
    "conditional_expression",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%", "//", "**",
    "==", "!=", "<", ">", "<=", ">=",
    "and", "or", "not", "in", "is",
    "=", "+=", "-=" , "*=", "/=", "%=", "//=", "**=",
    "&", "|", "^", "<<", ">>", "~",
    ".", ":", "->",
    "return_statement", "yield", "await",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "integer", "float", "string", "true", "false", "none",
];

impl LanguageAnalyzer for PythonAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "py")
    }

    fn language_name(&self) -> &'static str {
        "Python"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_python::LANGUAGE.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: count_decisions_for_python,
            require_children: true,
        }
    }
}

fn count_decisions_for_python(
    node: Node,
    source: &str,
    decision_kinds: &[&str],
    function_kinds: &[&str],
) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if function_kinds.contains(&child.kind()) {
            continue;
        }
        if decision_kinds.contains(&child.kind()) {
            count += 1;
        }
        if child.kind() == "match_statement" {
            count += crate::complexity::count_descendants_of_kind(child, &["case_clause"], function_kinds);
        }
        if crate::complexity::is_boolean_operator(child, source) {
            count += 1;
        }
        count += count_decisions_for_python(child, source, decision_kinds, function_kinds);
    }
    count
}

fn extract_name(node: Node, source: &str) -> String {
    if node.kind() == "lambda" {
        return format!("<lambda>@line {}", node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
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
        let source = "def foo():\n    if x:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_elif_else() {
        let source = "def bar():\n    if x:\n        pass\n    elif y:\n        pass\n    else:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + elif 1
    }

    #[test]
    fn test_match() {
        let source = "def baz():\n    match x:\n        case 1:\n            pass\n        case 2:\n            pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + 2 cases
    }

    #[test]
    fn test_lambda() {
        let source = "f = lambda x: 1 if x > 0 else 0\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].name.starts_with("<lambda>"));
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_try_except() {
        let source = "def err():\n    try:\n        pass\n    except A:\n        pass\n    except B:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + except A 1 + except B 1
    }
}
```

- [ ] **Step 6: Run the full test suite**

```bash
cargo test
```

Expected: all tests pass with no warnings about unused imports or dead code.

- [ ] **Step 7: Commit**

```bash
git add src/language/mod.rs src/language/rust.rs src/language/javascript.rs src/language/c.rs src/language/python.rs
git commit -m "refactor(language): extend LanguageAnalyzer trait with config/parser/default analyze"
```

---

### Task 5: Final verification

**Files:** (none modified)

- [ ] **Step 1: Run tests with output**

```bash
cargo test -- --nocapture 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 2: Build in release mode to confirm no warnings**

```bash
cargo build --release 2>&1
```

Expected: compiles clean with no warnings.

- [ ] **Step 3: Smoke-test the binary on the src directory**

```bash
cargo run -- src/ --format markdown 2>&1 | head -20
```

Expected: markdown output beginning with `## Summary Statistics` and file sections, matching prior behavior.
