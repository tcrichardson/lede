use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub lines: usize,
    pub complexity: u32,
    pub nesting_depth: u32,
    pub halstead_volume: f64,
    pub halstead_difficulty: f64,
    pub cognitive_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub path: std::path::PathBuf,
    pub total_complexity: u32,
    pub total_lines: usize,
    pub function_count: usize,
    pub functions: Vec<FunctionComplexity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub avg_cognitive_load: f64,
    pub max_nesting_depth: u32,
    pub avg_halstead_volume: f64,
}

pub mod analyzer;
pub mod cognitive;
pub mod complexity;
pub mod language;
pub mod output;

pub use analyzer::analyze_path;
