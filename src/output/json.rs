use crate::{AnalysisOutput, FileResult, SummaryStatistics, output::OutputFormatter};

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &[FileResult]) -> String {
        let summary = SummaryStatistics::from_results(results);

        let output = AnalysisOutput {
            summary,
            files: results.to_vec(),
        };

        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
    }
}
