use crate::{FileResult, output::OutputFormatter};
use comfy_table::{Table, ContentArrangement};

pub struct PrettyFormatter;

impl OutputFormatter for PrettyFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        results.iter().map(format_file_entry).collect()
    }
}

fn format_file_entry(file: &FileResult) -> String {
    let mut out = String::new();
    if let Some(ref err) = file.error {
        out.push_str(&format!("{}: ERROR: {}\n", file.path.display(), err));
        return out;
    }
    if file.functions.is_empty() {
        return out;
    }

    out.push_str(&format!(
        "{} (total complexity: {}, total lines: {}, functions: {})\n",
        file.path.display(),
        file.total_complexity,
        file.total_lines,
        file.function_count
    ));

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Function", "Lines", "Line Range", "Complexity"]);
    for func in &file.functions {
        table.add_row(vec![
            &func.name,
            &func.lines.to_string(),
            &format!("{}-{}", func.line_start, func.line_end),
            &func.complexity.to_string(),
        ]);
    }
    out.push_str(&table.to_string());
    out.push('\n');
    out
}
