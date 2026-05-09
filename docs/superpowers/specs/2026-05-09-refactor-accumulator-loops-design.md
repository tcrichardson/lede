# Design: Refactor Accumulator Loops in `lib.rs`

**Date:** 2026-05-09
**Author:** Kilo
**Status:** Approved

## Problem Statement

`FileResult::from_functions` and `SummaryStatistics::from_results` in `src/lib.rs` suffer from a "13-accumulator loop" anti-pattern. Adding a new aggregated metric requires coordinated edits in three separate locations inside each function: a running-sum variable, a running-max variable, and a final field assignment in the struct constructor. This is brittle, error-prone, and inflates Halstead complexity.

## Goals

1. Eliminate the 13-accumulator loop in `FileResult::from_functions`.
2. Eliminate the parallel pattern in `SummaryStatistics::from_results`.
3. Preserve all existing public API signatures — external callers see no change.
4. Make adding a new metric require adding a field to one struct and one method.
5. Keep the solution simple and explicit — no macros or runtime reflection.

## Approach

### Architecture: Dedicated Accumulator Structs with `From` Implementations

For each aggregation target, introduce an explicit accumulator struct that owns all running state. The struct provides an `add(...)` method for the fold step and a `From` implementation that constructs the final value.

### `FileResultAccumulator`

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
```

**Methods:**
- `fn new() -> Self` — initializes all fields to zero.
- `fn add(&mut self, func: &FunctionComplexity)` — updates sums and maxes from one function.

**Conversion:**
- `impl FileResult { fn from_accumulator(path, total_lines, functions, acc: &FileResultAccumulator) -> Self }`
- The public `from_functions` delegates to this helper after running the fold.

### `SummaryAccumulator`

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
```

**Methods:**
- `fn new() -> Self` — initializes all fields to zero.
- `fn add_file(&mut self, file: &FileResult)` — adds weighted contributions per-file.

**Conversion:**
- `impl SummaryStatistics { fn from_accumulator(acc: SummaryAccumulator) -> Self }` — computes averages from weighted sums and function count.
- The public `from_results` delegates to this helper after running the fold.

## Interface Changes

| Item | Before | After |
|------|--------|-------|
| `FileResult::from_functions` | Inline 13 accumulators | Delegates to `FileResult::from_accumulator` |
| `SummaryStatistics::from_results` | Inline `sum_usize`, `sum_u32`, `max_u32_from_files`, `weighted_avg` | Delegates to `SummaryStatistics::from_accumulator` |
| `sum_usize` | Used by `from_results` | Retained (still used elsewhere if needed) |
| `sum_u32` | Used by `from_results` | Retained |
| `max_u32_from_files` | Used by `from_results` | Retained |
| `weighted_avg` | Used by `from_results` | Retained |
| New types | None | `FileResultAccumulator`, `SummaryAccumulator` |

All public signatures remain **unchanged**.

## Testing & Validation

1. **Unit tests:** `cargo test` must pass with zero modifications to existing test code.
2. **Output fidelity:** Running `cargo run -- src/` must produce identical output to the existing `complexity.md` (within floating-point rounding).
3. **No new error paths:** The accumulator structs are additive and cannot fail.

## Why Not the Alternatives?

- **Declarative macro (Approach B):** Would reduce lines further, but introduces macro-complexity disproportionate to a ~1,500-line project. Harder to read and debug.
- **Generic iterator helpers + `fold` (Approach C):** Does not solve the core problem because the interdependent nature of sum+max for the same metric still forces many fields.

## Acceptance Criteria

- [ ] `FileResult::from_functions` contains no inline accumulator variables.
- [ ] `SummaryStatistics::from_results` contains no inline accumulator variables.
- [ ] `cargo test` passes.
- [ ] Complexity output for `src/` matches existing `complexity.md`.
- [ ] All public API signatures unchanged.
