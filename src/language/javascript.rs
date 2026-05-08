use crate::language::{LanguageAnalyzer, LanguageConfig};
use tree_sitter::{Node, Parser};

pub struct JavaScriptAnalyzer;

const FUNCTION_KINDS: &[&str] = &[
    "function_declaration",
    "function_expression",
    "arrow_function",
    "method_definition",
];
const CLOSURE_KINDS: &[&str] = &[
    "function_expression",
    "arrow_function",
];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "while_statement",
    "do_statement",
    "catch_clause",
    "ternary_expression",
    "switch_case",
    "switch_default",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%", "**",
    "==", "!=", "===", "!==", "<", ">", "<=", ">=",
    "&&", "||", "!", "??", "?.",
    "=", "+=", "-=" , "*=", "/=", "%=", "**=",
    "&", "|", "^", "<<", ">>", ">>>", "~",
    "++", "--",
    ".", ":", "=>",
    "return_statement", "yield", "await",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "number", "string", "true", "false", "null", "undefined",
];

impl LanguageAnalyzer for JavaScriptAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "js" || e == "jsx")
    }

    fn language_name(&self) -> &'static str {
        "JS"
    }

    fn parser(&self) -> Result<Parser, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_javascript::LANGUAGE.into();
        parser.set_language(&language).map_err(|e| format!("{e:?}"))?;
        Ok(parser)
    }

    fn config(&self) -> LanguageConfig {
        LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            closure_kinds: CLOSURE_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            match_case_kinds: &[],
            skip_childless_nodes: false,
        }
    }
}

fn extract_name(node: Node, source: &str) -> String {
    if node.kind() == "arrow_function" || node.kind() == "function_expression" {
        return format!("<closure>@line {}", node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "property_identifier" {
            return source[child.start_byte()..child.end_byte()].to_string();
        }
    }
    format!("<anon>@line {}", node.start_position().row + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let source = "function foo() { if (x) {} }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_else() {
        let source = "function bar() { if (x) {} else if (y) {} else {} }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + else-if 1
    }

    #[test]
    fn test_switch() {
        let source = "function baz() { switch(x) { case 1: break; case 2: break; default: break; } }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 4); // base 1 + 3 cases
    }

    #[test]
    fn test_arrow_function_included() {
        let source = "const f = (x) => x > 0 ? 1 : 0;";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, true).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].name.starts_with("<closure>"));
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_arrow_function_excluded_by_default() {
        let source = "const f = (x) => x > 0 ? 1 : 0;";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_try_catch() {
        let source = "function err() { try {} catch (a) { if (b) {} } }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + catch 1 + if 1
    }

    #[test]
    fn test_boolean_ops() {
        let source = "function b() { return a && b || c; }";
        let analyzer = JavaScriptAnalyzer;
        let result = analyzer.analyze(source, false).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + && 1 + || 1
    }
}
