use crate::{FunctionComplexity, language::{LanguageAnalyzer, LanguageConfig}};
use tree_sitter::{Node, Parser};

pub struct PythonAnalyzer;

const FUNCTION_KINDS: &[&str] = &["function_definition", "lambda"];
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "elif_clause",
    "for_statement",
    "while_statement",
    "except_clause",
    "conditional_expression",
];
const OPERATOR_KINDS: &[&str] = &[
    "+", "-", "*", "/", "%", "//", "**",
    "==", "!=", "<", ">", "<=", ">=",
    "and", "or", "not", "in", "is",
    "=", "+=", "-=" , "*=", "/=", "%=", "//=", "**=",
    "&", "|", "^", "<<", ">>", "~",
    ".", ":", "->",
    "return_statement", "yield", "await",
];
const OPERAND_KINDS: &[&str] = &[
    "identifier", "integer", "float", "string", "true", "false", "none",
];

impl LanguageAnalyzer for PythonAnalyzer {
    fn can_analyze(&self, path: &std::path::Path) -> bool {
        path.extension().map_or(false, |e| e == "py")
    }

    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String> {
        let mut parser = Parser::new();
        let language: tree_sitter::Language = tree_sitter_python::LANGUAGE.into();
        parser
            .set_language(&language)
            .map_err(|e| format!("{e:?}"))?;
        let tree = parser.parse(source, None).ok_or("Failed to parse Python source")?;
        if tree.root_node().has_error() {
            return Err("Failed to parse Python source".to_string());
        }
        let mut functions = Vec::new();
        collect_functions(tree.root_node(), source, &mut functions);
        Ok(functions)
    }
}

fn collect_functions(node: Node, source: &str, functions: &mut Vec<FunctionComplexity>) {
    crate::language::collect_functions(
        node, source, functions,
        &crate::language::LanguageConfig {
            function_kinds: FUNCTION_KINDS,
            decision_kinds: DECISION_KINDS,
            operator_kinds: OPERATOR_KINDS,
            operand_kinds: OPERAND_KINDS,
            extract_name,
            count_decisions_fn: count_decisions_for_python,
            require_children: true,
        },
    );
}

fn count_decisions_for_python(
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
        if child.kind() == "match_statement" {
            count += crate::complexity::count_descendants_of_kind(child, &["case_clause"], function_kinds);
        }
        if crate::complexity::is_boolean_operator(child, source) {
            count += 1;
        }
        count += count_decisions_for_python(child, source, decision_kinds, function_kinds);
    }
    count
}

fn extract_name(node: Node, source: &str) -> String {
    if node.kind() == "lambda" {
        return format!("<lambda>@line {}", node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
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
        let source = "def foo():\n    if x:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[0].complexity, 2);
    }

    #[test]
    fn test_if_elif_else() {
        let source = "def bar():\n    if x:\n        pass\n    elif y:\n        pass\n    else:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + if 1 + elif 1
    }

    #[test]
    fn test_match() {
        let source = "def baz():\n    match x:\n        case 1:\n            pass\n        case 2:\n            pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + 2 cases
    }

    #[test]
    fn test_lambda() {
        let source = "f = lambda x: 1 if x > 0 else 0\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].name.starts_with("<lambda>"));
        assert_eq!(result[0].complexity, 2); // base 1 + ternary 1
    }

    #[test]
    fn test_try_except() {
        let source = "def err():\n    try:\n        pass\n    except A:\n        pass\n    except B:\n        pass\n";
        let analyzer = PythonAnalyzer;
        let result = analyzer.analyze(source).unwrap();
        assert_eq!(result[0].complexity, 3); // base 1 + except A 1 + except B 1
    }
}
