use crate::{FileResult, output::OutputFormatter};
use comfy_table::{Table, ContentArrangement};

pub struct PrettyFormatter;

impl OutputFormatter for PrettyFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        let mut out = String::new();
        for file in results {
            if let Some(ref err) = file.error {
                out.push_str(&format!("{}: ERROR: {}\n", file.path.display(), err));
                continue;
            }
            if file.functions.is_empty() {
                continue;
            }
            out.push_str(&format!("{} (total: {})\n", file.path.display(), file.total_complexity));
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(vec!["Function", "Lines", "Complexity"]);
            for func in &file.functions {
                table.add_row(vec![
                    &func.name,
                    &format!("{}-{}", func.line_start, func.line_end),
                    &func.complexity.to_string(),
                ]);
            }
            out.push_str(&table.to_string());
            out.push('\n');
        }
        out
    }
}
