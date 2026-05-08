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
    pub halstead_effort: f64,
    pub halstead_time: f64,
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
    pub max_nesting_depth: u32,
    pub avg_nesting_depth: f64,
    pub avg_halstead_volume: f64,
    pub avg_halstead_difficulty: f64,
    pub avg_halstead_effort: f64,
    pub avg_halstead_time: f64,
    pub max_complexity: u32,
    pub max_function_lines: usize,
    pub total_function_lines: usize,
    pub max_halstead_volume: f64,
    pub max_halstead_difficulty: f64,
    pub max_halstead_effort: f64,
    pub max_halstead_time: f64,
}

impl Default for FileResult {
    fn default() -> Self {
        Self {
            path: std::path::PathBuf::new(),
            total_complexity: 0,
            total_lines: 0,
            function_count: 0,
            functions: Vec::new(),
            error: None,
            max_nesting_depth: 0,
            avg_nesting_depth: 0.0,
            avg_halstead_volume: 0.0,
            avg_halstead_difficulty: 0.0,
            avg_halstead_effort: 0.0,
            avg_halstead_time: 0.0,
            max_complexity: 0,
            max_function_lines: 0,
            total_function_lines: 0,
            max_halstead_volume: 0.0,
            max_halstead_difficulty: 0.0,
            max_halstead_effort: 0.0,
            max_halstead_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStatistics {
    pub files_analyzed: usize,
    pub total_functions: usize,
    pub total_lines: usize,
    pub total_complexity: u32,
    pub avg_complexity_per_function: f64,
    pub max_nesting_depth: u32,
    pub avg_nesting_depth: f64,
    pub avg_halstead_volume: f64,
    pub avg_halstead_difficulty: f64,
    pub avg_halstead_effort: f64,
    pub avg_halstead_time: f64,
}

impl Default for SummaryStatistics {
    fn default() -> Self {
        Self {
            files_analyzed: 0,
            total_functions: 0,
            total_lines: 0,
            total_complexity: 0,
            avg_complexity_per_function: 0.0,
            max_nesting_depth: 0,
            avg_nesting_depth: 0.0,
            avg_halstead_volume: 0.0,
            avg_halstead_difficulty: 0.0,
            avg_halstead_effort: 0.0,
            avg_halstead_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub summary: SummaryStatistics,
    pub files: Vec<FileResult>,
}

impl SummaryStatistics {
    pub fn from_results(results: &[FileResult]) -> Self {
        let successful: Vec<&FileResult> = results
            .iter()
            .filter(|f| f.error.is_none() && !f.functions.is_empty())
            .collect();

        if successful.is_empty() {
            return SummaryStatistics::default();
        }

        let total_files = successful.len();
        let total_functions: usize = successful.iter().map(|f| f.function_count).sum();
        let total_lines: usize = successful.iter().map(|f| f.total_lines).sum();
        let total_complexity: u32 = successful.iter().map(|f| f.total_complexity).sum();
        let avg_complexity = if total_functions > 0 {
            total_complexity as f64 / total_functions as f64
        } else {
            0.0
        };
        let max_nesting = successful
            .iter()
            .map(|f| f.max_nesting_depth)
            .max()
            .unwrap_or(0);

        let weighted_avg = |extractor: fn(&FileResult) -> f64| -> f64 {
            if total_functions == 0 {
                return 0.0;
            }
            successful
                .iter()
                .map(|f| extractor(f) * f.function_count as f64)
                .sum::<f64>()
                / total_functions as f64
        };

        SummaryStatistics {
            files_analyzed: total_files,
            total_functions,
            total_lines,
            total_complexity,
            avg_complexity_per_function: avg_complexity,
            max_nesting_depth: max_nesting,
            avg_nesting_depth: weighted_avg(|f| f.avg_nesting_depth),
            avg_halstead_volume: weighted_avg(|f| f.avg_halstead_volume),
            avg_halstead_difficulty: weighted_avg(|f| f.avg_halstead_difficulty),
            avg_halstead_effort: weighted_avg(|f| f.avg_halstead_effort),
            avg_halstead_time: weighted_avg(|f| f.avg_halstead_time),
        }
    }
}

pub mod analyzer;
pub mod cognitive;
pub mod complexity;
pub mod language;
pub mod output;

pub use analyzer::analyze_path;
