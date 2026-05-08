# Complexity Reduction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce cyclomatic complexity, nesting depth, and Halstead metrics across the rubik codebase by extracting helpers, flattening control flow, and eliminating closure proliferation.

**Architecture:** Decompose high-complexity functions into smaller named helpers. Introduce shared utility functions for repeated metric aggregation (max, avg, sum). Replace inline closures with standalone functions. Flatten deep nesting via early returns and iterator combinators. Extract table-formatting logic into focused sub-functions.

**Tech Stack:** Rust 2024, tree-sitter, cargo test, clippy

---

## File Structure

| File | Responsibility | Change |
|------|---------------|--------|
| `src/analyzer.rs` | File/directory analysis orchestration | Extract metric helpers; flatten `analyze_path` |
| `src/lib.rs` | Core types and summary statistics | Extract `weighted_avg` and `sum`/`max` helpers |
| `src/complexity.rs` | Generic tree-sitter complexity utilities | Replace loop+if chain in `is_boolean_operator` with const array + `any` |
| `src/output/markdown.rs` | Markdown report formatter | Decompose `format_file` into summary-table and function-table helpers |
| `src/output/pretty.rs` | Pretty-print table formatter | Extract per-file formatting into `format_file_entry` |
| `src/cognitive.rs` | Nesting depth and Halstead metrics | Refactor `find_function` to return `Option<Node>` |
| `src/language/mod.rs` | Language analyzer trait + shared helpers | Add generic `collect_functions` and `count_decisions` helpers (Task 8) |
| `src/language/rust.rs` | Rust language analyzer | Delegate to shared helpers (Task 8) |
| `src/language/javascript.rs` | JS language analyzer | Delegate to shared helpers (Task 8) |
| `src/language/python.rs` | Python language analyzer | Delegate to shared helpers (Task 8) |
| `src/language/c.rs` | C language analyzer | Delegate to shared helpers (Task 8) |

---

### Task 1: analyzer.rs — Extract Metric Helpers and Eliminate Closures

**Files:**
- Modify: `src/analyzer.rs:53-101`

**Context:** `build_success_result` contains ~15 one-line closures that inflate Halstead Volume to 599 and Effort to 4,963. Replace them with standalone helper functions.

- [ ] **Step 1: Add helper functions above `build_success_result`**

```rust
fn max_u32(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> u32) -> u32 {
    functions.iter().map(extractor).max().unwrap_or(0)
}

fn max_usize(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> usize) -> usize {
    functions.iter().map(extractor).max().unwrap_or(0)
}

fn max_f64(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> f64) -> f64 {
    functions.iter().map(extractor).fold(0.0_f64, f64::max)
}

fn avg_f64(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> f64) -> f64 {
    if functions.is_empty() {
        0.0
    } else {
        functions.iter().map(extractor).sum::<f64>() / functions.len() as f64
    }
}
```

- [ ] **Step 2: Replace `build_success_result` body to use helpers**

Replace lines 53-101 with:

```rust
fn build_success_result(path: &Path, total_lines: usize, functions: Vec<FunctionComplexity>) -> FileResult {
    let function_count = functions.len();
    let total_complexity: u32 = functions.iter().map(|f| f.complexity).sum();
    let total_function_lines: usize = functions.iter().map(|f| f.lines).sum();

    FileResult {
        path: path.to_path_buf(),
        total_complexity,
        total_lines,
        function_count,
        functions,
        error: None,
        max_nesting_depth: max_u32(&functions, |f| f.nesting_depth),
        avg_nesting_depth: avg_f64(&functions, |f| f.nesting_depth as f64),
        avg_halstead_volume: avg_f64(&functions, |f| f.halstead_volume),
        avg_halstead_difficulty: avg_f64(&functions, |f| f.halstead_difficulty),
        avg_halstead_effort: avg_f64(&functions, |f| f.halstead_effort),
        avg_halstead_time: avg_f64(&functions, |f| f.halstead_time),
        max_complexity: max_u32(&functions, |f| f.complexity),
        max_function_lines: max_usize(&functions, |f| f.lines),
        total_function_lines,
        max_halstead_volume: max_f64(&functions, |f| f.halstead_volume),
        max_halstead_difficulty: max_f64(&functions, |f| f.halstead_difficulty),
        max_halstead_effort: max_f64(&functions, |f| f.halstead_effort),
        max_halstead_time: max_f64(&functions, |f| f.halstead_time),
    }
}
```

- [ ] **Step 3: Run tests to verify behavior unchanged**

Run: `cargo test --lib`
Expected: All unit tests pass.

Run: `cargo test --test integration_test`
Expected: All integration tests pass.

- [ ] **Step 4: Commit**

```bash
git add src/analyzer.rs
git commit -m "refactor(analyzer): extract metric helpers to eliminate closures in build_success_result"
```

---

### Task 2: analyzer.rs — Flatten Nesting in `analyze_path`

**Files:**
- Modify: `src/analyzer.rs:15-35`

**Context:** `analyze_path` has nesting depth 5 (the max in the project). Use early return for the error case and separate file vs directory logic.

- [ ] **Step 1: Refactor `analyze_path` with early returns**

Replace lines 15-35 with:

```rust
pub fn analyze_path(path: &Path) -> Result<Vec<FileResult>, std::io::Error> {
    if !path.is_file() && !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} is not a file or directory", path.display()),
        ));
    }

    let mut results = Vec::new();

    if path.is_file() {
        results.push(analyze_file(path)?);
        return Ok(results);
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            results.push(analyze_file(p)?);
        }
    }

    Ok(results)
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --test integration_test::test_directory_scan`
Expected: PASS

Run: `cargo test --test integration_test::test_rust_fixture_pretty`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/analyzer.rs
git commit -m "refactor(analyzer): flatten analyze_path nesting with early returns"
```

---

### Task 3: complexity.rs — Simplify `is_boolean_operator`

**Files:**
- Modify: `src/complexity.rs:31-44`

**Context:** `is_boolean_operator` has cyclomatic complexity 5. Replace the `for`+`if`+double-`||` chain with a const array and `any()`.

- [ ] **Step 1: Replace `is_boolean_operator` implementation**

Replace lines 31-44 with:

```rust
const BOOLEAN_OPS: &[&str] = &["&&", "||"];

/// Check whether a binary_expression node uses `&&` or `||`.
pub fn is_boolean_operator(node: Node, source: &str) -> bool {
    if node.kind() != "binary_expression" {
        return false;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        BOOLEAN_OPS.contains(&source[child.start_byte()..child.end_byte()])
    })
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --lib`
Expected: All unit tests pass (including language-specific boolean op tests).

- [ ] **Step 3: Commit**

```bash
git add src/complexity.rs
git commit -m "refactor(complexity): simplify is_boolean_operator with const lookup"
```

---

### Task 4: markdown.rs — Decompose `format_file`

**Files:**
- Modify: `src/output/markdown.rs:47-95`

**Context:** `format_file` is 49 lines, complexity 6, Halstead Effort 9,428. Extract summary-table and function-table builders.

- [ ] **Step 1: Replace `format_file` and add `format_file_summary` + `format_function_table`**

Replace lines 47-95 with:

```rust
fn format_file(file: &FileResult) -> String {
    if let Some(ref err) = file.error {
        return format!("**{}**: ERROR: {}\n\n", file.path.display(), err);
    }
    if file.functions.is_empty() {
        return String::new();
    }

    let mut out = format!("### {}\n\n", file.path.display());
    out.push_str("#### File Summary\n\n");
    out.push_str(&format_file_summary(file));
    out.push_str(&format_function_table(&file.functions));
    out.push('\n');
    out
}

fn format_file_summary(file: &FileResult) -> String {
    let fc = file.function_count;
    let avg_complexity = if fc > 0 {
        file.total_complexity as f64 / fc as f64
    } else {
        0.0
    };

    let rows = vec![
        metric_row("Total Functions", fc),
        metric_row("Total Lines", file.total_lines),
        metric_row("Total Function Lines", file.total_function_lines),
        metric_row("Total Complexity", file.total_complexity),
        metric_row_f64("Avg Complexity / Function", avg_complexity, 2),
        metric_row("Max Complexity", file.max_complexity),
        metric_row("Max Nesting Depth", file.max_nesting_depth),
        metric_row_f64("Avg Nesting Depth", file.avg_nesting_depth, 2),
        metric_row_f64("Max Function Lines", file.max_function_lines as f64, 2),
        metric_row_f64("Avg Halstead Volume", file.avg_halstead_volume, 2),
        metric_row_f64("Max Halstead Volume", file.max_halstead_volume, 2),
        metric_row_f64("Avg Halstead Difficulty", file.avg_halstead_difficulty, 2),
        metric_row_f64("Max Halstead Difficulty", file.max_halstead_difficulty, 2),
        metric_row_f64("Avg Halstead Effort", file.avg_halstead_effort, 2),
        metric_row_f64("Max Halstead Effort", file.max_halstead_effort, 2),
        metric_row_f64("Avg Halstead Time", file.avg_halstead_time, 2),
        metric_row_f64("Max Halstead Time", file.max_halstead_time, 2),
    ];

    let mut out = String::from("| Metric | Value |\n|--------|-------|\n");
    for row in rows {
        out.push_str(&row);
    }
    out.push('\n');
    out
}

fn format_function_table(functions: &[FunctionComplexity]) -> String {
    let mut out = String::from(
        "| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |\n",
    );
    out.push_str("|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|\n");
    for func in functions {
        out.push_str(&format_function_row(func));
    }
    out.push('\n');
    out
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --test integration_test`
Expected: All integration tests pass.

Run: `cargo test --lib`
Expected: All unit tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/output/markdown.rs
git commit -m "refactor(markdown): decompose format_file into summary and table helpers"
```

---

### Task 5: pretty.rs — Extract `format_file_entry`

**Files:**
- Modify: `src/output/pretty.rs:7-33`

**Context:** `format` is 27 lines, complexity 5, Halstead Effort 3,502. Extract per-file formatting.

- [ ] **Step 1: Replace `format` impl and add `format_file_entry`**

Replace lines 7-33 with:

```rust
impl OutputFormatter for PrettyFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        results.iter().map(format_file_entry).collect()
    }
}

fn format_file_entry(file: &FileResult) -> String {
    let mut out = String::new();
    if let Some(ref err) = file.error {
        out.push_str(&format!("{}: ERROR: {}\n", file.path.display(), err));
        return out;
    }
    if file.functions.is_empty() {
        return out;
    }

    out.push_str(&format!(
        "{} (total complexity: {}, total lines: {}, functions: {})\n",
        file.path.display(),
        file.total_complexity,
        file.total_lines,
        file.function_count
    ));

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Function", "Lines", "Line Range", "Complexity"]);
    for func in &file.functions {
        table.add_row(vec![
            &func.name,
            &func.lines.to_string(),
            &format!("{}-{}", func.line_start, func.line_end),
            &func.complexity.to_string(),
        ]);
    }
    out.push_str(&table.to_string());
    out.push('\n');
    out
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --test integration_test::test_rust_fixture_pretty`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/output/pretty.rs
git commit -m "refactor(pretty): extract format_file_entry to reduce format complexity"
```

---

### Task 6: cognitive.rs — Refactor `find_function` to `Option<Node>`

**Files:**
- Modify: `src/cognitive.rs:136-148`
- Modify: `src/cognitive.rs:151-195` (test call sites)

**Context:** `find_function` has Halstead Difficulty 12.57. It does an unnecessary second `kind()` check after recursion and returns a fallback node instead of signaling "not found".

- [ ] **Step 1: Replace `find_function` and update call sites in tests**

Replace lines 136-148 with:

```rust
fn find_function<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(found) = find_function(child, kind) {
            return Some(found);
        }
    }
    None
}
```

In the tests (lines 155, 165, 175, 185), replace `find_function(...)` with `find_function(...).expect("function not found in tree")`.

Specifically:
- Line 155: `let func = find_function(root, "function_item");` → `let func = find_function(root, "function_item").expect("function not found");`
- Line 165: same
- Line 175: same
- Line 185: same

- [ ] **Step 2: Run tests**

Run: `cargo test --lib cognitive`
Expected: All cognitive module tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/cognitive.rs
git commit -m "refactor(cognitive): return Option from find_function to eliminate redundant kind check"
```

---

### Task 7: lib.rs — Extract Named Helpers for `from_results`

**Files:**
- Modify: `src/lib.rs:106-156`

**Context:** `from_results` is 50 lines with an inline `weighted_avg` closure. Extract it and the aggregation helpers.

- [ ] **Step 1: Add helper functions above `from_results`**

Insert before line 106:

```rust
fn sum_usize(files: &[&FileResult], extractor: fn(&FileResult) -> usize) -> usize {
    files.iter().map(|f| extractor(f)).sum()
}

fn sum_u32(files: &[&FileResult], extractor: fn(&FileResult) -> u32) -> u32 {
    files.iter().map(|f| extractor(f)).sum()
}

fn max_u32_from_files(files: &[&FileResult], extractor: fn(&FileResult) -> u32) -> u32 {
    files.iter().map(|f| extractor(f)).max().unwrap_or(0)
}

fn safe_div(numerator: f64, denominator: f64) -> f64 {
    if denominator > 0.0 {
        numerator / denominator
    } else {
        0.0
    }
}

fn weighted_avg(
    files: &[&FileResult],
    total_functions: usize,
    extractor: fn(&FileResult) -> f64,
) -> f64 {
    if total_functions == 0 {
        return 0.0;
    }
    files
        .iter()
        .map(|f| extractor(f) * f.function_count as f64)
        .sum::<f64>()
        / total_functions as f64
}
```

- [ ] **Step 2: Replace `from_results` body**

Replace lines 106-156 with:

```rust
impl SummaryStatistics {
    pub fn from_results(results: &[FileResult]) -> Self {
        let successful: Vec<&FileResult> = results
            .iter()
            .filter(|f| f.error.is_none() && !f.functions.is_empty())
            .collect();

        if successful.is_empty() {
            return SummaryStatistics::default();
        }

        let total_files = successful.len();
        let total_functions = sum_usize(&successful, |f| f.function_count);
        let total_lines = sum_usize(&successful, |f| f.total_lines);
        let total_complexity = sum_u32(&successful, |f| f.total_complexity);
        let avg_complexity = safe_div(total_complexity as f64, total_functions as f64);
        let max_nesting = max_u32_from_files(&successful, |f| f.max_nesting_depth);

        SummaryStatistics {
            files_analyzed: total_files,
            total_functions,
            total_lines,
            total_complexity,
            avg_complexity_per_function: avg_complexity,
            max_nesting_depth: max_nesting,
            avg_nesting_depth: weighted_avg(&successful, total_functions, |f| f.avg_nesting_depth),
            avg_halstead_volume: weighted_avg(&successful, total_functions, |f| f.avg_halstead_volume),
            avg_halstead_difficulty: weighted_avg(&successful, total_functions, |f| f.avg_halstead_difficulty),
            avg_halstead_effort: weighted_avg(&successful, total_functions, |f| f.avg_halstead_effort),
            avg_halstead_time: weighted_avg(&successful, total_functions, |f| f.avg_halstead_time),
        }
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test --test integration_test`
Expected: All integration tests pass.

- [ ] **Step 4: Commit**

```bash
git add src/lib.rs
git commit -m "refactor(lib): extract named helpers from from_results to eliminate closure"
```

---

### Task 8: language modules — Abstract `collect_functions` and `count_decisions`

**Files:**
- Modify: `src/language/mod.rs`
- Modify: `src/language/rust.rs`
- Modify: `src/language/javascript.rs`
- Modify: `src/language/python.rs`
- Modify: `src/language/c.rs`

**Context:** Four language modules have near-identical `collect_functions` (28 lines, Halstead Effort ~5,300 each) and `count_decisions`. Extract shared traversal logic into `language/mod.rs`, leaving only language-specific `extract_name` in each module.

- [ ] **Step 1: Add shared helpers to `src/language/mod.rs`**

Replace the entire file with:

```rust
use crate::FunctionComplexity;
use std::path::Path;
use tree_sitter::Node;

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
) {
    if function_kinds.contains(&node.kind()) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source, decision_kinds, function_kinds);
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
            extract_name,
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

- [ ] **Step 2: Update `src/language/rust.rs`**

Replace `collect_functions` (lines 51-78) and `count_decisions` (lines 80-96) with calls to the shared helpers:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        FUNCTION_KINDS, DECISION_KINDS, OPERATOR_KINDS, OPERAND_KINDS,
        extract_name,
    );
}
```

Keep `extract_name` unchanged (lines 98-109).

- [ ] **Step 3: Update `src/language/javascript.rs`**

Replace `collect_functions` (lines 57-84) and `count_decisions` (lines 86-102) with:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        FUNCTION_KINDS, DECISION_KINDS, OPERATOR_KINDS, OPERAND_KINDS,
        extract_name,
    );
}
```

Keep `extract_name` unchanged (lines 104-115).

- [ ] **Step 4: Update `src/language/c.rs`**

Replace `collect_functions` (lines 49-76) and `count_decisions` (lines 78-94) with:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        FUNCTION_KINDS, DECISION_KINDS, OPERATOR_KINDS, OPERAND_KINDS,
        extract_name,
    );
}
```

Keep `extract_name` (lines 96-109) and `find_identifier_in_declarator` (lines 111-123) unchanged.

- [ ] **Step 5: Update `src/language/python.rs`**

Python has two differences: `node.child_count() > 0` guard in `collect_functions`, and `match_statement` handling in `count_decisions`. Preserve these by keeping a local `collect_functions` wrapper and a local `count_decisions` that delegates after its special case.

Replace `collect_functions` (lines 49-76) with:

```rust
fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if node.kind() == "function_definition" && node.child_count() == 0 {
        // Skip empty function stubs (e.g., decorators without body)
        return;
    }
    crate::language::collect_functions(
        node, source, functions,
        FUNCTION_KINDS, DECISION_KINDS, OPERATOR_KINDS, OPERAND_KINDS,
        extract_name,
    );
}
```

Replace `count_decisions` (lines 78-97) with:

```rust
fn count_decisions(node: Node, source: &str) -> u32 {
    let mut count = crate::language::count_decisions(node, source, DECISION_KINDS, FUNCTION_KINDS);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "match_statement" {
            count += crate::complexity::count_descendants_of_kind(child, &["case_clause"], FUNCTION_KINDS);
        }
    }
    count
}
```

Wait — this double-counts because `language::count_decisions` already recurses through children. The Python special case needs to run **instead of** the generic recursion for `match_statement` nodes, or be added **on top** only once per `match_statement`.

Looking at the original Python `count_decisions`:
```rust
for child in node.children(&mut cursor) {
    if FUNCTION_KINDS.contains(&child.kind()) { continue; }
    if DECISION_KINDS.contains(&child.kind()) { count += 1; }
    if child.kind() == "match_statement" {
        count += crate::complexity::count_descendants_of_kind(child, &["case_clause"], FUNCTION_KINDS);
    }
    if crate::complexity::is_boolean_operator(child, source) { count += 1; }
    count += count_decisions(child, source);
}
```

So `match_statement` is NOT in `DECISION_KINDS`. The special case adds case_clause counts for each match_statement. The generic `language::count_decisions` will see `match_statement`, not count it as a decision (since it's not in `DECISION_KINDS`), recurse into it, and find nothing because `case_clause` is not in `DECISION_KINDS` either.

So the Python-specific wrapper should add the match_statement bonus **after** calling the generic version:

```rust
fn count_decisions(node: Node, source: &str) -> u32 {
    let mut count = crate::language::count_decisions(node, source, DECISION_KINDS, FUNCTION_KINDS);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "match_statement" {
            count += crate::complexity::count_descendants_of_kind(child, &["case_clause"], FUNCTION_KINDS);
        }
    }
    count
}
```

But wait — this adds match_statement bonuses at every level, not just where match_statement appears as a direct child. That's actually what the original did too (it recursed and checked each child). So this is correct.

Actually, there's a subtle bug: the generic `count_decisions` already recurses and would process match_statement's children. Then our wrapper ALSO iterates direct children looking for match_statement. This means match_statement at deeper levels gets counted correctly (because recursion in generic handles it, and wrapper at each level checks direct children). This matches the original behavior.

But wait — the wrapper's loop does NOT recurse. It only checks direct children. So match_statement at depth 2 would be caught by the generic recursion at depth 1 (which calls `count_decisions(child, ...)` where child is depth 1), and then at depth 1's call, the wrapper checks its direct children and finds match_statement at depth 2.

So yes, the behavior is preserved.

However, we need to be careful: `language::count_decisions` also includes `is_boolean_operator`. Our wrapper adds match_statement bonus. This is correct.

- [ ] **Step 6: Run tests**

Run: `cargo test --lib`
Expected: All unit tests pass for rust, javascript, python, c modules.

Run: `cargo test --test integration_test`
Expected: All integration tests pass.

- [ ] **Step 7: Commit**

```bash
git add src/language/mod.rs src/language/rust.rs src/language/javascript.rs src/language/python.rs src/language/c.rs
git commit -m "refactor(language): extract shared collect_functions and count_decisions helpers"
```

---

## Self-Review Checklist

**Spec coverage:** Every high-complexity function identified in the complexity report has a corresponding task:
- `build_success_result` → Task 1
- `analyze_path` → Task 2
- `is_boolean_operator` → Task 3
- `format_file` (markdown) → Task 4
- `format` (pretty) → Task 5
- `find_function` → Task 6
- `from_results` → Task 7
- `collect_functions` / `count_decisions` (×4) → Task 8

**Placeholder scan:** No "TBD", "TODO", "implement later", or "similar to" found. Every step includes exact file paths, exact code, exact commands, and exact expected output.

**Type consistency:** All helper signatures use consistent naming (`max_u32`, `avg_f64`, `sum_usize`, `weighted_avg`). `find_function` returns `Option<Node>` and all call sites use `.expect(...)`. The `collect_functions` shared helper takes the same parameters in the same order in all call sites.

---

## Execution Handoff

**Plan complete and saved to `docs/superpowers/plans/2026-05-07-complexity-reduction.md`.**

**Two execution options:**

1. **Subagent-Driven (recommended)** — Dispatch a fresh subagent per task, review between tasks, fast iteration.
2. **Inline Execution** — Execute tasks in this session using `executing-plans`, batch execution with checkpoints.

**Which approach?**
