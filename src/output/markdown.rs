use crate::{FileResult, output::OutputFormatter};

pub struct MarkdownFormatter;

impl OutputFormatter for MarkdownFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        let mut out = String::new();
        for file in results {
            if let Some(ref err) = file.error {
                out.push_str(&format!("**{}**: ERROR: {}\n\n", file.path.display(), err));
                continue;
            }
            if file.functions.is_empty() {
                continue;
            }
            out.push_str(&format!(
                "### {} (total complexity: {})\n\n",
                file.path.display(),
                file.total_complexity
            ));
            out.push_str("| Function | Lines | Complexity |\n");
            out.push_str("|----------|-------|------------|\n");
            for func in &file.functions {
                out.push_str(&format!(
                    "| {} | {}-{} | {} |\n",
                    func.name,
                    func.line_start,
                    func.line_end,
                    func.complexity
                ));
            }
            out.push('\n');
        }
        out
    }
}
