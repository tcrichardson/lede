use crate::output::OutputFormatter;
use crate::FileResult;

pub struct PrettyFormatter;

impl OutputFormatter for PrettyFormatter {
    fn format(&self, _results: &[FileResult]) -> String {
        todo!()
    }
}
