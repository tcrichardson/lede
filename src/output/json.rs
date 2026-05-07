use crate::output::OutputFormatter;
use crate::FileResult;

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, _results: &[FileResult]) -> String {
        todo!()
    }
}
