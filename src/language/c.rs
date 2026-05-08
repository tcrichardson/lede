use crate::{FunctionComplexity, language::LanguageAnalyzer};
use tree_sitter::{Node, Parser};

pub struct CAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_definition"];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "while_statement",
    "do_statement",
    "case_statement",
    "conditional_expression",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%",
    "==", "!=", "<", ">", "<=", ">=",
    "&&", "||", "!",
    "=", "+=", "-=", "*=", "/=", "%=",
    "&", "|", "^", "<<", ">>", "~",
    ".", "->", ":",
    "return_statement", "break_statement", "continue_statement", "goto_statement",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "number_literal", "string_literal", "char_literal",
    "true", "false", "null",
];

impl LanguageAnalyzer for CAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "c" || e == "h")
    }

    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
        parser
            .set_language(&language)
            .map_err(|e| format!("{e:?}"))?;
        let tree = parser.parse(source, None).ok_or("Failed to parse C source")?;
        if tree.root_node().has_error() {
            return Err("Failed to parse C source".to_string());
        }
        let mut functions = Vec::new();
        collect_functions(tree.root_node(), source, &mut functions);
        Ok(functions)
    }
}

fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    if FUNCTION_KINDS.contains(&node.kind()) {
        let name = extract_name(node, source);
        let complexity = 1 + count_decisions(node, source);
        let nesting_depth = crate::cognitive::max_nesting_depth(node, DECISION_KINDS, FUNCTION_KINDS);
        let (halstead_volume, halstead_difficulty) = crate::cognitive::halstead_metrics(
            node, source, OPERATOR_KINDS, OPERAND_KINDS, FUNCTION_KINDS,
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
        collect_functions(child, source, functions);
    }
}

fn count_decisions(node: Node, source: &str) -> u32 {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if FUNCTION_KINDS.contains(&child.kind()) {
            continue;
        }
        if DECISION_KINDS.contains(&child.kind()) {
            count += 1;
        }
        if crate::complexity::is_boolean_operator(child, source) {
            count += 1;
        }
        count += count_decisions(child, source);
    }
    count
}

fn extract_name(node: Node, source: &str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return source[child.start_byte()..child.end_byte()].to_string();
        }
        // The declarator may be wrapped in pointer_declarator or function_declarator
        let name = find_identifier_in_declarator(child, source);
        if !name.is_empty() {
            return name;
        }
    }
    format!("<anon>@line {}", node.start_position().row + 1)
}

fn find_identifier_in_declarator(node: Node, source: &str) -> String {
    if node.kind() == "identifier" {
        return source[node.start_byte()..node.end_byte()].to_string();
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let name = find_identifier_in_declarator(child, source);
        if !name.is_empty() {
            return name;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let source = "int foo() { if (1) {} }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else() {
        let source = "int bar() { if (x) {} else {} }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 2); // base 1 + if 1
    }

    #[test]
    fn test_switch() {
        let source = r#"
int baz() {
    switch (x) {
        case 1: break;
        case 2: break;
        default: break;
    }
}
"#;
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 cases
    }

    #[test]
    fn test_for_while_do() {
        let source = r#"
int loop() {
    for (;;) {}
    while (1) {}
    do {} while (1);
}
"#;
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + for 1 + while 1 + do 1
    }

    #[test]
    fn test_ternary() {
        let source = "int t() { return x > 0 ? 1 : 0; }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_boolean_ops() {
        let source = "int b() { return a && b || c; }";
        let analyzer = CAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }
}
