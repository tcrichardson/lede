use crate::{FileResult, SummaryStatistics, output::OutputFormatter};

pub struct MarkdownFormatter;

impl OutputFormatter for MarkdownFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        let mut out = String::new();

        let summary = SummaryStatistics::from_results(results);

        if summary.files_analyzed > 0 {
            out.push_str("## Summary Statistics\n\n");
            out.push_str("| Metric | Value |\n");
            out.push_str("|--------|-------|\n");
            out.push_str(&format!("| Files Analyzed | {} |\n", summary.files_analyzed));
            out.push_str(&format!("| Total Functions | {} |\n", summary.total_functions));
            out.push_str(&format!("| Total Lines | {} |\n", summary.total_lines));
            out.push_str(&format!("| Total Complexity | {} |\n", summary.total_complexity));
            out.push_str(&format!("| Avg Complexity / Function | {:.2} |\n", summary.avg_complexity_per_function));
            out.push_str(&format!("| Max Nesting Depth | {} |\n", summary.max_nesting_depth));
            out.push_str(&format!("| Avg Nesting Depth | {:.2} |\n", summary.avg_nesting_depth));
            out.push_str(&format!("| Avg Halstead Volume | {:.2} |\n", summary.avg_halstead_volume));
            out.push_str(&format!("| Avg Halstead Difficulty | {:.2} |\n", summary.avg_halstead_difficulty));
            out.push_str(&format!("| Avg Halstead Effort | {:.2} |\n", summary.avg_halstead_effort));
            out.push_str(&format!("| Avg Halstead Time | {:.2} |\n", summary.avg_halstead_time));
            out.push('\n');
        }

        for file in results {
            if let Some(ref err) = file.error {
                out.push_str(&format!("**{}**: ERROR: {}\n\n", file.path.display(), err));
                continue;
            }
            if file.functions.is_empty() {
                continue;
            }

            let fc = file.function_count;
            let avg_complexity = if fc > 0 { file.total_complexity as f64 / fc as f64 } else { 0.0 };

            out.push_str(&format!("### {}\n\n", file.path.display()));
            out.push_str("#### File Summary\n\n");
            out.push_str("| Metric | Value |\n");
            out.push_str("|--------|-------|\n");
            out.push_str(&format!("| Total Functions | {} |\n", fc));
            out.push_str(&format!("| Total Lines | {} |\n", file.total_lines));
            out.push_str(&format!("| Total Function Lines | {} |\n", file.total_function_lines));
            out.push_str(&format!("| Total Complexity | {} |\n", file.total_complexity));
            out.push_str(&format!("| Avg Complexity / Function | {:.2} |\n", avg_complexity));
            out.push_str(&format!("| Max Complexity | {} |\n", file.max_complexity));
            out.push_str(&format!("| Max Nesting Depth | {} |\n", file.max_nesting_depth));
            out.push_str(&format!("| Avg Nesting Depth | {:.2} |\n", file.avg_nesting_depth));
            out.push_str(&format!("| Max Function Lines | {} |\n", file.max_function_lines));
            out.push_str(&format!("| Avg Halstead Volume | {:.2} |\n", file.avg_halstead_volume));
            out.push_str(&format!("| Max Halstead Volume | {:.2} |\n", file.max_halstead_volume));
            out.push_str(&format!("| Avg Halstead Difficulty | {:.2} |\n", file.avg_halstead_difficulty));
            out.push_str(&format!("| Max Halstead Difficulty | {:.2} |\n", file.max_halstead_difficulty));
            out.push_str(&format!("| Avg Halstead Effort | {:.2} |\n", file.avg_halstead_effort));
            out.push_str(&format!("| Max Halstead Effort | {:.2} |\n", file.max_halstead_effort));
            out.push_str(&format!("| Avg Halstead Time | {:.2} |\n", file.avg_halstead_time));
            out.push_str(&format!("| Max Halstead Time | {:.2} |\n", file.max_halstead_time));
            out.push('\n');

            out.push_str("| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |\n");
            out.push_str("|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|\n");
            for func in &file.functions {
                out.push_str(&format!(
                    "| {} | {} | {}-{} | {} | {} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
                    func.name,
                    func.lines,
                    func.line_start,
                    func.line_end,
                    func.complexity,
                    func.nesting_depth,
                    func.halstead_volume,
                    func.halstead_difficulty,
                    func.halstead_effort,
                    func.halstead_time
                ));
            }
            out.push('\n');
        }
        out
    }
}
