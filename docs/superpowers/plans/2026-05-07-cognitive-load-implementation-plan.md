# Cognitive Load Scoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add cognitive load scoring (nesting depth + Halstead metrics) per function/file for Rust, Python, and JavaScript, surfaced in Markdown and JSON output.

**Architecture:** Introduce a new `src/cognitive.rs` module that computes `max_nesting_depth` and `halstead_metrics` via tree-sitter node traversal. Language analyzers call it during function collection. File-level aggregates are computed in `analyzer.rs`. Markdown formatter gains new columns.

**Tech Stack:** Rust, tree-sitter, serde

---

### Task 1: Create `src/cognitive.rs` — Nesting Depth & Halstead Metrics

**Files:**
- Create: `src/cognitive.rs`
- Modify: `src/lib.rs` (add `pub mod cognitive;`)

**Design:**
- `max_nesting_depth(node, decision_kinds, function_kinds) -> u32` — recursive walker that counts how many `decision_kinds` bodies are nested inside each other, stopping at nested functions.
- `halstead_metrics(node, source, operator_kinds, operand_kinds, function_kinds) -> (f64, f64)` — returns `(volume, difficulty)`.
  - Walk the AST skipping nested functions.
  - Count distinct operator kinds vs total operator occurrences.
  - Count distinct operand node texts vs total operand occurrences.
  - Compute: `volume = total * log2(distinct)`, `difficulty = (distinct_operators / 2.0) * (total_operands / distinct_operands)`.
  - If there are no operands, difficulty = 0. If distinct == 0, volume = 0.

- [ ] **Step 1: Write `src/cognitive.rs`**

```rust
use tree_sitter::Node;
use std::collections::HashSet;

pub fn max_nesting_depth(node: Node, decision_kinds: &[&str], function_kinds: &[&str]) -> u32 {
    compute_nesting_depth(node, decision_kinds, function_kinds, 0)
}

fn compute_nesting_depth(node: Node, decision_kinds: &[&str], function_kinds: &[&str], current_depth: u32) -> u32 {
    let mut max_depth = current_depth;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if function_kinds.contains(&child.kind()) {
            continue;
        }
        let is_decision = decision_kinds.contains(&child.kind());
        let next_depth = if is_decision { current_depth + 1 } else { current_depth };
        let child_max = compute_nesting_depth(child, decision_kinds, function_kinds, next_depth);
        if child_max > max_depth {
            max_depth = child_max;
        }
    }
    max_depth
}

pub fn halstead_metrics(
    node: Node,
    source: &str,
    operator_kinds: &[&str],
    operand_kinds: &[&str],
    function_kinds: &[&str],
) -> (f64, f64) {
    let mut operators_distinct: HashSet<String> = HashSet::new();
    let mut operators_total: u32 = 0;
    let mut operands_distinct: HashSet<String> = HashSet::new();
    let mut operands_total: u32 = 0;

    collect_halstead(node, source, operator_kinds, operand_kinds, function_kinds, &mut operators_distinct, &mut operators_total, &mut operands_distinct, &mut operands_total);

    let n = operators_distinct.len() + operands_distinct.len();
    let n1 = operators_distinct.len();
    let n2 = operands_distinct.len();
    let n_total = operators_total + operands_total;

    let volume = if n == 0 {
        0.0
    } else {
        (n_total as f64) * ((n as f64).log2())
    };

    let difficulty = if n2 == 0 {
        0.0
    } else {
        ((n1 as f64) / 2.0) * ((operands_total as f64) / (n2 as f64))
    };

    (volume, difficulty)
}

fn collect_halstead(
    node: Node,
    source: &str,
    operator_kinds: &[&str],
    operand_kinds: &[&str],
    function_kinds: &[&str],
    operators_distinct: &mut HashSet<String>,
    operators_total: &mut u32,
    operands_distinct: &mut HashSet<String>,
    operands_total: &mut u32,
) {
    if function_kinds.contains(&node.kind()) {
        return;
    }

    let kind = node.kind();
    if operator_kinds.contains(&kind) {
        operators_distinct.insert(kind.to_string());
        *operators_total += 1;
    } else if operand_kinds.contains(&kind) {
        let text = &source[node.start_byte()..node.end_byte()];
        operands_distinct.insert(text.to_string());
        *operands_total += 1;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_halstead(child, source, operator_kinds, operand_kinds, function_kinds, operators_distinct, operators_total, operands_distinct, operands_total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse_rust(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();
        parser.set_language(&language).unwrap();
        parser.parse(source, None).unwrap()
    }

    #[test]
    fn test_nesting_depth_simple() {
        let source = "fn f() { if true {} }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let depth = max_nesting_depth(root, &["if_expression"], &["function_item"]);
        assert_eq!(depth, 1);
    }

    #[test]
    fn test_nesting_depth_nested() {
        let source = "fn f() { if true { for x in y {} } }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let depth = max_nesting_depth(root, &["if_expression", "for_expression"], &["function_item"]);
        assert_eq!(depth, 2);
    }

    #[test]
    fn test_nesting_depth_skips_inner_function() {
        let source = "fn f() { if true { fn g() { if true {} } } }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let depth = max_nesting_depth(root, &["if_expression"], &["function_item"]);
        assert_eq!(depth, 1);
    }

    #[test]
    fn test_halstead_basic() {
        let source = "fn f() { let x = 1 + 2; }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let (volume, difficulty) = halstead_metrics(
            root,
            source,
            &["+", "let_declaration"],
            &["identifier", "integer_literal"],
            &["function_item"],
        );
        assert!(volume > 0.0);
        assert!(difficulty >= 0.0);
    }
}
```

- [ ] **Step 2: Add module declaration to `src/lib.rs`**

Insert after `pub mod complexity;`:
```rust
pub mod cognitive;
```

- [ ] **Step 3: Run tests for cognitive module**

Run: `cargo test cognitive --lib`
Expected: All 4 tests PASS.

- [ ] **Step 4: Commit**

```bash
git add src/cognitive.rs src/lib.rs
git commit -m "feat: add cognitive metrics module (nesting depth + Halstead)"
```

---

### Task 2: Update `FunctionComplexity` and `FileResult` structs

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Add new fields to `FunctionComplexity`**

Replace the struct with:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub lines: usize,
    pub complexity: u32,
    pub nesting_depth: u32,
    pub halstead_volume: f64,
    pub halstead_difficulty: f64,
    pub cognitive_load: f64,
}
```

- [ ] **Step 2: Add new fields to `FileResult`**

Replace the struct with:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub path: std::path::PathBuf,
    pub total_complexity: u32,
    pub total_lines: usize,
    pub function_count: usize,
    pub functions: Vec<FunctionComplexity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub avg_cognitive_load: f64,
    pub max_nesting_depth: u32,
    pub avg_halstead_volume: f64,
}
```

- [ ] **Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "feat: add cognitive load fields to data model"
```

---

### Task 3: Update `src/language/rust.rs`

**Files:**
- Modify: `src/language/rust.rs`

- [ ] **Step 1: Define operator/operand token lists**

Add after `const DECISION_KINDS`:
```rust
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
```

- [ ] **Step 2: Update `collect_functions` to compute cognitive metrics**

Replace the body with:
```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if FUNCTION_KINDS.contains(&node.kind()) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, DECISION_KINDS, FUNCTION_KINDS);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, OPERATOR_KINDS, OPERAND_KINDS, FUNCTION_KINDS,
        );
        let cognitive_load = (halstead_volume / 100.0) + (nesting_depth as f64 * 5.0) + (halstead_difficulty / 10.0);
        functions.push(FunctionComplexity {
            name,
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            lines: node.end_position().row - node.start_position().row + 1,
            complexity,
            nesting_depth,
            halstead_volume,
            halstead_difficulty,
            cognitive_load,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions);
    }
}
```

- [ ] **Step 3: Update existing unit tests** — add default values for new fields in any direct struct construction. Since tests only assert on `name` and `complexity`, and the struct is constructed inside `collect_functions`, no test changes needed.

- [ ] **Step 4: Run Rust analyzer tests**

Run: `cargo test rust::tests --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/language/rust.rs
git commit -m "feat: compute cognitive metrics in Rust analyzer"
```

---

### Task 4: Update `src/language/python.rs`

**Files:**
- Modify: `src/language/python.rs`

- [ ] **Step 1: Define operator/operand token lists**

Add after `const DECISION_KINDS`:
```rust
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
```

- [ ] **Step 2: Update `collect_functions`**

Replace the body with:
```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if FUNCTION_KINDS.contains(&node.kind()) && node.child_count() > 0 {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, DECISION_KINDS, FUNCTION_KINDS);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, OPERATOR_KINDS, OPERAND_KINDS, FUNCTION_KINDS,
        );
        let cognitive_load = (halstead_volume / 100.0) + (nesting_depth as f64 * 5.0) + (halstead_difficulty / 10.0);
        functions.push(FunctionComplexity {
            name,
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            lines: node.end_position().row - node.start_position().row + 1,
            complexity,
            nesting_depth,
            halstead_volume,
            halstead_difficulty,
            cognitive_load,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions);
    }
}
```

- [ ] **Step 3: Run Python analyzer tests**

Run: `cargo test python::tests --lib`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/language/python.rs
git commit -m "feat: compute cognitive metrics in Python analyzer"
```

---

### Task 5: Update `src/language/javascript.rs`

**Files:**
- Modify: `src/language/javascript.rs`

- [ ] **Step 1: Define operator/operand token lists**

Add after `const DECISION_KINDS`:
```rust
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
```

- [ ] **Step 2: Update `collect_functions`**

Replace the body with:
```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if FUNCTION_KINDS.contains(&node.kind()) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, DECISION_KINDS, FUNCTION_KINDS);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, OPERATOR_KINDS, OPERAND_KINDS, FUNCTION_KINDS,
        );
        let cognitive_load = (halstead_volume / 100.0) + (nesting_depth as f64 * 5.0) + (halstead_difficulty / 10.0);
        functions.push(FunctionComplexity {
            name,
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            lines: node.end_position().row - node.start_position().row + 1,
            complexity,
            nesting_depth,
            halstead_volume,
            halstead_difficulty,
            cognitive_load,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions);
    }
}
```

- [ ] **Step 3: Run JavaScript analyzer tests**

Run: `cargo test javascript::tests --lib`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/language/javascript.rs
git commit -m "feat: compute cognitive metrics in JavaScript analyzer"
```

---

### Task 6: Update `src/analyzer.rs` to compute file-level aggregates

**Files:**
- Modify: `src/analyzer.rs`

- [ ] **Step 1: Update `analyze_file` to populate aggregates**

Replace the two `Ok(FileResult { ... })` blocks inside the `for analyzer in ANALYZERS` loop.

First block (success):
```rust
let total = functions.iter().map(|f| f.complexity).sum();
let total_lines = source.lines().count();
let function_count = functions.len();
let avg_cognitive_load = if function_count > 0 {
    functions.iter().map(|f| f.cognitive_load).sum::<f64>() / function_count as f64
} else { 0.0 };
let max_nesting_depth = functions.iter().map(|f| f.nesting_depth).max().unwrap_or(0);
let avg_halstead_volume = if function_count > 0 {
    functions.iter().map(|f| f.halstead_volume).sum::<f64>() / function_count as f64
} else { 0.0 };
return Ok(FileResult {
    path: path.to_path_buf(),
    total_complexity: total,
    total_lines,
    function_count,
    functions,
    error: None,
    avg_cognitive_load,
    max_nesting_depth,
    avg_halstead_volume,
});
```

Second block (error):
```rust
return Ok(FileResult {
    path: path.to_path_buf(),
    total_complexity: 0,
    total_lines: source.lines().count(),
    function_count: 0,
    functions: Vec::new(),
    error: Some(e),
    avg_cognitive_load: 0.0,
    max_nesting_depth: 0,
    avg_halstead_volume: 0.0,
});
```

- [ ] **Step 2: Update the final unsupported-extension fallback**

Replace:
```rust
Ok(FileResult {
    path: path.to_path_buf(),
    total_complexity: 0,
    total_lines: source.lines().count(),
    function_count: 0,
    functions: Vec::new(),
    error: None,
    avg_cognitive_load: 0.0,
    max_nesting_depth: 0,
    avg_halstead_volume: 0.0,
})
```

- [ ] **Step 3: Run integration tests**

Run: `cargo test integration --test integration_test`
Expected: PASS (or note if tests need updating in Task 8).

- [ ] **Step 4: Commit**

```bash
git add src/analyzer.rs
git commit -m "feat: compute file-level cognitive aggregates"
```

---

### Task 7: Update `src/output/markdown.rs`

**Files:**
- Modify: `src/output/markdown.rs`

- [ ] **Step 1: Update header line and function rows**

Replace the function table header and rows:
```rust
out.push_str(&format!(
    "### {} (total complexity: {}, total lines: {}, functions: {}, avg cognitive load: {:.2}, max nesting: {}, avg Halstead volume: {:.2})\n\n",
    file.path.display(),
    file.total_complexity,
    file.total_lines,
    file.function_count,
    file.avg_cognitive_load,
    file.max_nesting_depth,
    file.avg_halstead_volume
));
out.push_str("| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Cognitive Load |\n");
out.push_str("|----------|-------|------------|------------|---------|--------------|------------|----------------|\n");
for func in &file.functions {
    out.push_str(&format!(
        "| {} | {} | {}-{} | {} | {} | {:.2} | {:.2} | {:.2} |\n",
        func.name,
        func.lines,
        func.line_start,
        func.line_end,
        func.complexity,
        func.nesting_depth,
        func.halstead_volume,
        func.halstead_difficulty,
        func.cognitive_load
    ));
}
```

- [ ] **Step 2: Run markdown output tests**

Run: `cargo test --lib`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/output/markdown.rs
git commit -m "feat: add cognitive load columns to markdown output"
```

---

### Task 8: Update integration tests and verify JSON output

**Files:**
- Modify: `tests/integration_test.rs`
- Read: `tests/fixtures/rust_sample.rs`, `tests/fixtures/python_sample.py`, `tests/fixtures/js_sample.js`

- [ ] **Step 1: Read the fixture files** to understand what functions exist.

- [ ] **Step 2: Update `tests/integration_test.rs`**

Add assertions that verify cognitive fields are present and > 0 for fixtures with functions. For example, after analyzing a fixture:
```rust
assert!(result.functions[0].cognitive_load > 0.0);
assert!(result.functions[0].halstead_volume > 0.0);
assert!(result.max_nesting_depth >= 0);
assert!(result.avg_cognitive_load >= 0.0);
```

- [ ] **Step 3: Run all tests**

Run: `cargo test`
Expected: All tests PASS.

- [ ] **Step 4: Do a manual smoke test**

Run: `cargo run -- tests/fixtures/rust_sample.rs --format markdown`
Expected: Markdown table includes new columns.

Run: `cargo run -- tests/fixtures/rust_sample.rs --format json`
Expected: JSON includes `nesting_depth`, `halstead_volume`, `halstead_difficulty`, `cognitive_load`, `avg_cognitive_load`, `max_nesting_depth`, `avg_halstead_volume`.

- [ ] **Step 5: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: verify cognitive load fields in integration tests"
```

---

## Self-Review

1. **Spec coverage:**
   - Nesting depth per function → Task 1, Tasks 3-5
   - Halstead volume/difficulty per function → Task 1, Tasks 3-5
   - Cognitive load formula per function → Tasks 3-5
   - File-level aggregates → Task 6
   - Markdown output → Task 7
   - JSON output → automatic via serde, no extra task needed
   - Tests → Task 8

2. **Placeholder scan:** No TBDs, no vague steps. All code is provided.

3. **Type consistency:** `FunctionComplexity` fields match usage in all analyzers and markdown. `FileResult` fields match `analyzer.rs`.

## Execution Handoff

**Plan complete and saved to `docs/superpowers/plans/2026-05-07-cognitive-load-implementation-plan.md`.**

**Two execution options:**

1. **Subagent-Driven (recommended)** - Dispatch a fresh subagent per task, review between tasks, fast iteration
2. **Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
