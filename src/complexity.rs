use tree_sitter::Node;

/// Count descendants whose kind is in `kinds`, skipping subtrees rooted at
/// nodes whose kind is in `skip_kinds`.
pub fn count_descendants_of_kind(node: Node, kinds: &[&str], skip_kinds: &[&str]) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if kinds.contains(&child.kind()) {
            count += 1;
        }
        if !skip_kinds.contains(&child.kind()) {
            count += count_descendants_of_kind(child, kinds, skip_kinds);
        }
    }
    count
}

/// Count immediate children whose kind is in `kinds`.
pub fn count_children_of_kind(node: Node, kinds: &[&str]) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if kinds.contains(&child.kind()) {
            count += 1;
        }
    }
    count
}

/// Check whether a binary_expression node uses `&&` or `||`.
pub fn is_boolean_operator(node: Node, source: &str) -> bool {
    if node.kind() != "binary_expression" {
        return false;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let text = &source[child.start_byte()..child.end_byte()];
        if text == "&&" || text == "||" {
            return true;
        }
    }
    false
}
