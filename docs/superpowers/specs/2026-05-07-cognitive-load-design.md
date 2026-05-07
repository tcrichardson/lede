# Cognitive Load Scoring for Rubik

## Overview
Add cognitive load scoring for Rust, Python, and JavaScript. Each code file and each function/method within a code file will receive a cognitive load score, exposed in both Markdown and JSON output.

## Goals
- Quantify how hard a function/file is to understand beyond raw cyclomatic complexity.
- Provide actionable metrics: nesting depth and Halstead volume/difficulty.
- Keep scoring deterministic, fast, and language-specific where needed.

## Metrics

### 1. Nesting Depth
The maximum depth of control-flow block nesting within a function.
- Count `if`/`for`/`while`/`match`/`try`/etc. bodies.
- Example: `if` inside `for` inside `while` = depth 3.

### 2. Halstead Metrics
Per-function, count distinct operators and operands to compute:
- **Vocabulary** η = η₁ + η₂ (distinct operators + distinct operands)
- **Length** N = N₁ + N₂ (total operators + total operands)
- **Volume** V = N × log₂(η)
- **Difficulty** D = (η₁ / 2) × (N₂ / η₂)

Operator/operand token definitions are language-specific.

### 3. Cognitive Load Score
Composite formula per function:
```
cognitive_load = (halstead_volume / 100)
               + (max_nesting_depth × 5)
               + (halstead_difficulty / 10)
```

## Data Model Changes

### `FunctionComplexity`
Add fields:
- `nesting_depth: u32`
- `halstead_volume: f64`
- `halstead_difficulty: f64`
- `cognitive_load: f64`

### `FileResult`
Add fields:
- `avg_cognitive_load: f64`
- `max_nesting_depth: u32`
- `avg_halstead_volume: f64`

## Output Changes

### Markdown
Extend the function table with columns:
- Nesting Depth
- Halstead Volume
- Difficulty
- Cognitive Load

Add a file-level summary line with avg/max cognitive load and avg Halstead volume.

### JSON
Serialize all new `FunctionComplexity` and `FileResult` fields automatically via serde.

## Architecture
- **`src/cognitive.rs`** — new module containing:
  - `max_nesting_depth(node, kinds)` — recursive walker skipping nested functions
  - `halstead_metrics(node, source, operators, operands)` — token-based counter returning volume and difficulty
- **Language analyzers** (`rust.rs`, `python.rs`, `javascript.rs`) — updated to:
  - Define language-specific operator/operand token kinds
  - Call `max_nesting_depth` and `halstead_metrics` during function collection
  - Populate new `FunctionComplexity` fields
- **`analyzer.rs`** — compute file-level aggregates (`avg_cognitive_load`, `max_nesting_depth`, `avg_halstead_volume`) after all functions are analyzed
- **`markdown.rs`** — add new columns and summary
- **`json.rs`** — no changes needed (serde handles new fields)

## Testing
- Unit tests per language for nesting depth and Halstead metrics on sample snippets
- Integration test verifying Markdown/JSON output contains new fields

## Performance
- Halstead counting is O(tokens in function) — negligible overhead on top of existing tree traversal.
