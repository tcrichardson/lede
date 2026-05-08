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
        if function_kinds.contains(&child.kind()) {
            continue;
        }
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

    #[test]
    fn test_nesting_depth_simple() {
        let source = "fn f() { if true {} }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let func = find_function(root, "function_item").expect("function not found");
        let depth = max_nesting_depth(func, &["if_expression"], &["function_item"]);
        assert_eq!(depth, 1);
    }

    #[test]
    fn test_nesting_depth_nested() {
        let source = "fn f() { if true { for x in y {} } }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let func = find_function(root, "function_item").expect("function not found");
        let depth = max_nesting_depth(func, &["if_expression", "for_expression"], &["function_item"]);
        assert_eq!(depth, 2);
    }

    #[test]
    fn test_nesting_depth_skips_inner_function() {
        let source = "fn f() { if true { fn g() { if true {} } } }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let func = find_function(root, "function_item").expect("function not found");
        let depth = max_nesting_depth(func, &["if_expression"], &["function_item"]);
        assert_eq!(depth, 1);
    }

    #[test]
    fn test_halstead_basic() {
        let source = "fn f() { let x = 1 + 2; }";
        let tree = parse_rust(source);
        let root = tree.root_node();
        let func = find_function(root, "function_item").expect("function not found");
        let (volume, difficulty) = halstead_metrics(
            func,
            source,
            &["+", "let_declaration"],
            &["identifier", "integer_literal"],
            &["function_item"],
        );
        assert!(volume > 0.0);
        assert!(difficulty >= 0.0);
    }
}
