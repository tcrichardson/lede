use crate::{FileResult, output::OutputFormatter};

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string())
    }
}
