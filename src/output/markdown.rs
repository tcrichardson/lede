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
                "### {} (total complexity: {}, total lines: {}, functions: {}, avg cognitive load: {:.2}, max nesting: {}, avg Halstead volume: {:.2})\n\n",
                file.path.display(),
                file.total_complexity,
                file.total_lines,
                file.function_count,
                file.avg_cognitive_load,
                file.max_nesting_depth,
                file.avg_halstead_volume
            ));
            out.push_str("| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Cognitive Load |\n");
            out.push_str("|----------|-------|------------|------------|---------|--------------|------------|----------------|\n");
            for func in &file.functions {
                out.push_str(&format!(
                    "| {} | {} | {}-{} | {} | {} | {:.2} | {:.2} | {:.2} |\n",
                    func.name,
                    func.lines,
                    func.line_start,
                    func.line_end,
                    func.complexity,
                    func.nesting_depth,
                    func.halstead_volume,
                    func.halstead_difficulty,
                    func.cognitive_load
                ));
            }
            out.push('\n');
        }
        out
    }
}
