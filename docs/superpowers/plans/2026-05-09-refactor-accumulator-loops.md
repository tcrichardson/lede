# Refactor Accumulator Loops Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the 13-accumulator loops in `FileResult::from_functions` and `SummaryStatistics::from_results` with dedicated accumulator structs.

**Architecture:** Introduce `FileResultAccumulator` and `SummaryAccumulator` structs that own running aggregation state, with `add` methods for the fold step and private `from_accumulator` helpers on `FileResult` and `SummaryStatistics` to build the final structs.

**Tech Stack:** Rust, cargo, tree-sitter

---

### Task 1: `FileResultAccumulator` struct and methods

**Files:**
- Modify: `src/lib.rs:68-133`

- [ ] **Step 1: Add `FileResultAccumulator` struct above `FileResult::from_functions`**

Insert the following struct and its `impl` block immediately before `impl FileResult`:

```rust
struct FileResultAccumulator {
    total_complexity: u32,
    total_function_lines: usize,
    max_complexity: u32,
    max_function_lines: usize,
    max_nesting_depth: u32,
    sum_nesting: f64,
    max_halstead_volume: f64,
    sum_halstead_volume: f64,
    max_halstead_difficulty: f64,
    sum_halstead_difficulty: f64,
    max_halstead_effort: f64,
    sum_halstead_effort: f64,
    max_halstead_time: f64,
    sum_halstead_time: f64,
}

impl FileResultAccumulator {
    fn new() -> Self {
        Self {
            total_complexity: 0,
            total_function_lines: 0,
            max_complexity: 0,
            max_function_lines: 0,
            max_nesting_depth: 0,
            sum_nesting: 0.0,
            max_halstead_volume: 0.0,
            sum_halstead_volume: 0.0,
            max_halstead_difficulty: 0.0,
            sum_halstead_difficulty: 0.0,
            max_halstead_effort: 0.0,
            sum_halstead_effort: 0.0,
            max_halstead_time: 0.0,
            sum_halstead_time: 0.0,
        }
    }

    fn add(&mut self, f: &FunctionComplexity) {
        self.total_complexity += f.complexity;
        self.total_function_lines += f.lines;
        self.max_complexity = self.max_complexity.max(f.complexity);
        self.max_function_lines = self.max_function_lines.max(f.lines);
        self.max_nesting_depth = self.max_nesting_depth.max(f.nesting_depth);
        self.sum_nesting += f.nesting_depth as f64;
        self.max_halstead_volume = self.max_halstead_volume.max(f.halstead_volume);
        self.sum_halstead_volume += f.halstead_volume;
        self.max_halstead_difficulty = self.max_halstead_difficulty.max(f.halstead_difficulty);
        self.sum_halstead_difficulty += f.halstead_difficulty;
        self.max_halstead_effort = self.max_halstead_effort.max(f.halstead_effort);
        self.sum_halstead_effort += f.halstead_effort;
        self.max_halstead_time = self.max_halstead_time.max(f.halstead_time);
        self.sum_halstead_time += f.halstead_time;
    }
}
```

- [ ] **Step 2: Add `FileResult::from_accumulator` helper**

Insert the following private method inside `impl FileResult` (before `from_functions`):

```rust
    fn from_accumulator(
        path: &Path,
        total_lines: usize,
        functions: Vec<FunctionComplexity>,
        acc: &FileResultAccumulator,
        count: usize,
    ) -> Self {
        let n = count as f64;
        Self {
            path: path.to_path_buf(),
            total_complexity: acc.total_complexity,
            total_lines,
            function_count: count,
            error: None,
            max_nesting_depth: acc.max_nesting_depth,
            avg_nesting_depth: acc.sum_nesting / n,
            max_complexity: acc.max_complexity,
            max_function_lines: acc.max_function_lines,
            total_function_lines: acc.total_function_lines,
            max_halstead_volume: acc.max_halstead_volume,
            avg_halstead_volume: acc.sum_halstead_volume / n,
            max_halstead_difficulty: acc.max_halstead_difficulty,
            avg_halstead_difficulty: acc.sum_halstead_difficulty / n,
            max_halstead_effort: acc.max_halstead_effort,
            avg_halstead_effort: acc.sum_halstead_effort / n,
            max_halstead_time: acc.max_halstead_time,
            avg_halstead_time: acc.sum_halstead_time / n,
            functions,
        }
    }
```

- [ ] **Step 3: Replace `FileResult::from_functions` body**

Replace the existing `from_functions` implementation with:

```rust
    pub fn from_functions(path: &Path, total_lines: usize, functions: Vec<FunctionComplexity>) -> Self {
        let count = functions.len();
        if count == 0 {
            return Self {
                path: path.to_path_buf(),
                total_lines,
                ..Default::default()
            };
        }

        let mut acc = FileResultAccumulator::new();
        for f in &functions {
            acc.add(f);
        }

        Self::from_accumulator(path, total_lines, functions, &acc, count)
    }
```

- [ ] **Step 4: Compile to verify Task 1**

Run: `cargo check`
Expected: Compiles cleanly.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs
git commit -m "refactor: extract FileResultAccumulator from from_functions"
```

---

### Task 2: `SummaryAccumulator` struct and methods

**Files:**
- Modify: `src/lib.rs:210-241`

- [ ] **Step 1: Add `SummaryAccumulator` struct above `SummaryStatistics`**

Insert the following struct and its `impl` block immediately before `impl SummaryStatistics`:

```rust
struct SummaryAccumulator {
    total_functions: usize,
    total_lines: usize,
    total_complexity: u32,
    max_nesting_depth: u32,
    weighted_sum_nesting: f64,
    weighted_sum_halstead_volume: f64,
    weighted_sum_halstead_difficulty: f64,
    weighted_sum_halstead_effort: f64,
    weighted_sum_halstead_time: f64,
}

impl SummaryAccumulator {
    fn new() -> Self {
        Self {
            total_functions: 0,
            total_lines: 0,
            total_complexity: 0,
            max_nesting_depth: 0,
            weighted_sum_nesting: 0.0,
            weighted_sum_halstead_volume: 0.0,
            weighted_sum_halstead_difficulty: 0.0,
            weighted_sum_halstead_effort: 0.0,
            weighted_sum_halstead_time: 0.0,
        }
    }

    fn add_file(&mut self, file: &FileResult) {
        let n = file.function_count as f64;
        self.total_functions += file.function_count;
        self.total_lines += file.total_lines;
        self.total_complexity += file.total_complexity;
        self.max_nesting_depth = self.max_nesting_depth.max(file.max_nesting_depth);
        self.weighted_sum_nesting += file.avg_nesting_depth * n;
        self.weighted_sum_halstead_volume += file.avg_halstead_volume * n;
        self.weighted_sum_halstead_difficulty += file.avg_halstead_difficulty * n;
        self.weighted_sum_halstead_effort += file.avg_halstead_effort * n;
        self.weighted_sum_halstead_time += file.avg_halstead_time * n;
    }
}
```

- [ ] **Step 2: Add `SummaryStatistics::from_accumulator` helper**

Insert the following private method inside `impl SummaryStatistics` (before `from_results`):

```rust
    fn from_accumulator(acc: SummaryAccumulator) -> Self {
        let n = acc.total_functions as f64;
        Self {
            files_analyzed: 0, // populated by caller
            total_functions: acc.total_functions,
            total_lines: acc.total_lines,
            total_complexity: acc.total_complexity,
            avg_complexity_per_function: safe_div(acc.total_complexity as f64, n),
            max_nesting_depth: acc.max_nesting_depth,
            avg_nesting_depth: safe_div(acc.weighted_sum_nesting, n),
            avg_halstead_volume: safe_div(acc.weighted_sum_halstead_volume, n),
            avg_halstead_difficulty: safe_div(acc.weighted_sum_halstead_difficulty, n),
            avg_halstead_effort: safe_div(acc.weighted_sum_halstead_effort, n),
            avg_halstead_time: safe_div(acc.weighted_sum_halstead_time, n),
        }
    }
```

- [ ] **Step 3: Replace `SummaryStatistics::from_results` body**

Replace the existing `from_results` implementation with:

```rust
    pub fn from_results(results: &[FileResult]) -> Self {
        let successful: Vec<&FileResult> = results
            .iter()
            .filter(|f| f.error.is_none() && !f.functions.is_empty())
            .collect();

        if successful.is_empty() {
            return SummaryStatistics::default();
        }

        let total_files = successful.len();
        let mut acc = SummaryAccumulator::new();
        for file in &successful {
            acc.add_file(file);
        }

        let mut stats = SummaryStatistics::from_accumulator(acc);
        stats.files_analyzed = total_files;
        stats
    }
```

- [ ] **Step 4: Compile to verify Task 2**

Run: `cargo check`
Expected: Compiles cleanly.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs
git commit -m "refactor: extract SummaryAccumulator from from_results"
```

---

### Task 3: Validation

- [ ] **Step 1: Run all unit tests**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 2: Verify output fidelity against existing complexity.md**

Run: `cargo run -- src/ > /tmp/new-complexity.md`

Then compare: `diff complexity.md /tmp/new-complexity.md`

Expected: No diff (or only whitespace differences). If there are float rounding differences in the last decimal place, that is acceptable.

- [ ] **Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "test: verify accumulator refactor preserves output"
```

---

## Plan Self-Review

**Spec coverage:**
- `FileResultAccumulator` with `new` and `add` — Task 1, Steps 1-2 ✓
- `FileResult::from_accumulator` helper — Task 1, Step 2 ✓
- `FileResult::from_functions` refactored to delegate — Task 1, Step 3 ✓
- `SummaryAccumulator` with `new` and `add_file` — Task 2, Steps 1-2 ✓
- `SummaryStatistics::from_accumulator` helper — Task 2, Step 2 ✓
- `SummaryStatistics::from_results` refactored to delegate — Task 2, Step 3 ✓
- Public API signatures unchanged — verified, no signature changes in plan ✓
- `cargo test` passes — Task 3, Step 1 ✓
- Output fidelity check — Task 3, Step 2 ✓

**Placeholder scan:** No TBD, TODO, or vague steps. Every step contains exact code or exact commands.

**Type consistency:**
- `FileResultAccumulator` field names match `FileResult` field names ✓
- `SummaryAccumulator` uses `weighted_sum_*` prefix consistently ✓
- `safe_div` is reused from existing code ✓
- All numeric types (`u32`, `usize`, `f64`) match the spec and existing code ✓
