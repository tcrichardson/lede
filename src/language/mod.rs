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
    count_decisions_fn: fn(Node, &str, &[&str], &[&str]) -> u32,
) {
    if function_kinds.contains(&node.kind()) && node.child_count() > 0 {
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
            extract_name, count_decisions_fn,
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
