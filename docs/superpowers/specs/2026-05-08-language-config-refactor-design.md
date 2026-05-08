# Language Config Refactor Design

**Date:** 2026-05-08  
**Status:** Approved

## Goals

- Primary: Improve maintainability — make adding a new language analyzer straightforward and the trait self-documenting as a complete contract
- Secondary: Improve structural clarity — the `LanguageAnalyzer` trait should be the single extension point; language config and behavior should live together
- Metric improvement is a byproduct: `collect_functions` Halstead effort is projected to drop from ~13,800 to under 3,000

## Scope

Focused. Only the top complexity hotspot is addressed: the 10-parameter `collect_functions` signature in `language/mod.rs` and the resulting boilerplate in each language file. No changes to `analyzer.rs`, `cognitive.rs`, `complexity.rs`, `lib.rs`, or the output formatters.

## Problem

`collect_functions` in `language/mod.rs` accepts 10 parameters, including two function pointers:

```rust
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
)
```

This drives a Halstead difficulty of 19.75 and effort of 13,883 — roughly 5× the next-highest function in the codebase. Each language file also contains a near-identical `analyze()` body (create parser, parse, check errors, collect), repeated across Rust, JS, and C with no shared abstraction.

## Architecture

### `LanguageConfig` struct

Introduced in `language/mod.rs`. Bundles all per-language configuration that was previously passed as individual arguments:

```rust
pub struct LanguageConfig {
    pub function_kinds: &'static [&'static str],
    pub decision_kinds: &'static [&'static str],
    pub operator_kinds: &'static [&'static str],
    pub operand_kinds: &'static [&'static str],
    pub extract_name: fn(Node, &str) -> String,
    pub count_decisions_fn: fn(Node, &str, &[&str], &[&str]) -> u32,
    pub require_children: bool,
}
```

### Updated `LanguageAnalyzer` trait

Two new required methods are added (`parser` and `config`). A default `analyze()` implementation is added that covers the common parse-and-collect path:

```rust
pub trait LanguageAnalyzer: Send + Sync {
    fn can_analyze(&self, path: &Path) -> bool;
    fn config(&self) -> LanguageConfig;         // new, required
    fn parser(&self) -> Parser;                 // new, required
    fn language_name(&self) -> &'static str { "source" }  // new, optional default
    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = self.parser();
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

### Updated `collect_functions`

Signature shrinks from 10 parameters to 4:

```rust
pub fn collect_functions(
    node: Node,
    source: &str,
    functions: &mut Vec<FunctionComplexity>,
    config: &LanguageConfig,
)
```

## Components

### `language/mod.rs`

- Add `LanguageConfig` struct
- Add `parser()` and `config()` to `LanguageAnalyzer` trait
- Add default `analyze()` to `LanguageAnalyzer` trait
- Update `collect_functions` to accept `&LanguageConfig` instead of 10 parameters
- `count_decisions` free function is unchanged — it is referenced as `config.count_decisions_fn` by three of the four languages

### `language/rust.rs`, `language/javascript.rs`, `language/c.rs`

- Remove `analyze()` implementation (covered by trait default)
- Add `parser()` method returning a configured `tree_sitter::Parser`
- Add `language_name()` method returning the language label (e.g. `"Rust"`, `"JS"`, `"C"`) to preserve specific error messages
- Add `config()` method returning `LanguageConfig` referencing the existing `const` arrays and `extract_name` function
- The existing `FUNCTION_KINDS`, `DECISION_KINDS`, `OPERATOR_KINDS`, `OPERAND_KINDS` const arrays remain in place
- The local `collect_functions` wrapper is removed (no longer needed)
- `extract_name` function is unchanged

### `language/python.rs`

- Adds `parser()` and `config()` methods like the other languages
- Retains its explicit `analyze()` override — `count_decisions_for_python` contains match-statement logic that cannot be expressed through the generic path
- `config.count_decisions_fn` points to `count_decisions_for_python`
- `count_decisions_for_python` and `extract_name` are unchanged

### Unchanged files

`analyzer.rs`, `cognitive.rs`, `complexity.rs`, `lib.rs`, and all output formatters require no changes. They interact with the trait's `analyze()` method and `FileResult`/`FunctionComplexity` types, which are unaffected.

## Data Flow

No behavioral change end-to-end:

```
analyze_path → analyze_file → analyzer.can_analyze()
             → analyzer.analyze()          ← now executes via trait default for Rust/JS/C
               → collect_functions(node, source, &mut functions, &config)
                 → FunctionComplexity pushed to vec
```

## Error Handling

No changes. The default `analyze()` propagates errors the same way the existing per-language bodies do:
- Parser setup errors return `Err(String)`
- Parse failures return `Err("Failed to parse ... source")`

## Testing

All existing unit tests in each language file remain valid and pass without modification — behavior is unchanged. The existing tests serve as the full regression suite.

The Python `analyze()` override is already covered by Python's unit tests, which exercise `count_decisions_for_python` through the full analyzer path.

No new tests are required for this refactor.

## Success Criteria

- All existing tests pass
- `collect_functions` parameter count drops from 10 to 4
- `collect_functions` Halstead effort drops from ~13,800 to under 3,000
- Each standard language file (Rust, JS, C) implements exactly 3 trait methods: `can_analyze`, `parser`, `config`
- No behavioral change in output
