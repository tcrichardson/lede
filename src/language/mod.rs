use crate::FunctionComplexity;
use std::path::Path;

pub trait LanguageAnalyzer: Send + Sync {
    fn can_analyze(&self, path: &Path) -> bool;
    fn analyze(&self, source: &str) -> Result<Vec<FunctionComplexity>, String>;
}

pub mod c;
pub mod javascript;
pub mod python;
pub mod rust;
