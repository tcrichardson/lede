use crate::FunctionComplexity;
use std::path::Path;
use tree_sitter::{Node, Parser};

pub struct LanguageConfig {
    pub function_kinds: &'static [&'static str],
    pub closure_kinds: &'static [&'static str],
    pub decision_kinds: &'static [&'static str],
    pub operator_kinds: &'static [&'static str],
    pub operand_kinds: &'static [&'static str],
    pub extract_name: fn(Node, &str) -> String,
    pub count_decisions_fn: fn(Node, &str, &[&str], &[&str]) -> u32,
    pub require_children: bool,
}

pub trait LanguageAnalyzer: Send + Sync {
    fn can_analyze(&self, path: &Path) -> bool;
    fn config(&self) -> LanguageConfig;
    fn parser(&self) -> Result<Parser, String>;
    fn language_name(&self) -> &'static str {
        "source"
    }
    fn analyze(&self, source: &str, include_closures: bool) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = self.parser()?;
        let config = self.config();
        let msg = format!("Failed to parse {} source", self.language_name());
        let tree = parser.parse(source, None).ok_or_else(|| msg.clone())?;
        if tree.root_node().has_error() {
            return Err(msg);
        }
        let mut functions = Vec::new();
        collect_functions(tree.root_node(), source, &mut functions, &config, include_closures);
        Ok(functions)
    }
}

/// Generic function collector. Each language analyzer calls this with its own configuration.
pub fn collect_functions(
    node: Node,
    source: &str,
    functions: &mut Vec<FunctionComplexity>,
    config: &LanguageConfig,
    include_closures: bool,
) {
    if config.function_kinds.contains(&node.kind())
        && (!config.require_children || node.child_count() > 0)
    {
        if include_closures || !config.closure_kinds.contains(&node.kind()) {
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
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions, config, include_closures);
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
