use crate::FileResult;

pub trait OutputFormatter {
    fn format(&self, results: &[FileResult]) -> String;
}

pub mod json;
pub mod pretty;

pub fn get_formatter(format: &str) -> Box<dyn OutputFormatter> {
    match format {
        "json" => Box::new(json::JsonFormatter),
        _ => Box::new(pretty::PrettyFormatter),
    }
}
