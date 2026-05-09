use crate::duplicates::DuplicateCluster;
use crate::FileResult;

pub trait OutputFormatter {
    fn format(&self, results: &[FileResult], clusters: &[DuplicateCluster]) -> String;
}

pub mod json;
pub mod markdown;
pub mod pretty;

pub fn get_formatter(format: &str) -> Box<dyn OutputFormatter> {
    match format {
        "json" => Box::new(json::JsonFormatter),
        "pretty" => Box::new(pretty::PrettyFormatter),
        _ => Box::new(markdown::MarkdownFormatter),
    }
}
