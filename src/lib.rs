use serde::{Deserialize, Serialize};
use std::path::Path;

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

struct FileResultAccumulator {
    total_complexity: u32,
    total_function_lines: usize,
    max_complexity: u32,
    max_function_lines: usize,
    max_nesting_depth: u32,
    sum_nesting: f64,
    max_halstead_volume: f64,
    sum_halstead_volume: f64,
    max_halstead_difficulty: f64,
    sum_halstead_difficulty: f64,
    max_halstead_effort: f64,
    sum_halstead_effort: f64,
    max_halstead_time: f64,
    sum_halstead_time: f64,
}

impl FileResultAccumulator {
    fn new() -> Self {
        Self {
            total_complexity: 0,
            total_function_lines: 0,
            max_complexity: 0,
            max_function_lines: 0,
            max_nesting_depth: 0,
            sum_nesting: 0.0,
            max_halstead_volume: 0.0,
            sum_halstead_volume: 0.0,
            max_halstead_difficulty: 0.0,
            sum_halstead_difficulty: 0.0,
            max_halstead_effort: 0.0,
            sum_halstead_effort: 0.0,
            max_halstead_time: 0.0,
            sum_halstead_time: 0.0,
        }
    }

    fn add(&mut self, f: &FunctionComplexity) {
        self.total_complexity += f.complexity;
        self.total_function_lines += f.lines;
        self.max_complexity = self.max_complexity.max(f.complexity);
        self.max_function_lines = self.max_function_lines.max(f.lines);
        self.max_nesting_depth = self.max_nesting_depth.max(f.nesting_depth);
        self.sum_nesting += f.nesting_depth as f64;
        self.max_halstead_volume = self.max_halstead_volume.max(f.halstead_volume);
        self.sum_halstead_volume += f.halstead_volume;
        self.max_halstead_difficulty = self.max_halstead_difficulty.max(f.halstead_difficulty);
        self.sum_halstead_difficulty += f.halstead_difficulty;
        self.max_halstead_effort = self.max_halstead_effort.max(f.halstead_effort);
        self.sum_halstead_effort += f.halstead_effort;
        self.max_halstead_time = self.max_halstead_time.max(f.halstead_time);
        self.sum_halstead_time += f.halstead_time;
    }
}

impl FileResult {
    fn from_accumulator(
        path: &Path,
        total_lines: usize,
        functions: Vec<FunctionComplexity>,
        acc: &FileResultAccumulator,
        count: usize,
    ) -> Self {
        let n = count as f64;
        Self {
            path: path.to_path_buf(),
            total_complexity: acc.total_complexity,
            total_lines,
            function_count: count,
            error: None,
            max_nesting_depth: acc.max_nesting_depth,
            avg_nesting_depth: acc.sum_nesting / n,
            max_complexity: acc.max_complexity,
            max_function_lines: acc.max_function_lines,
            total_function_lines: acc.total_function_lines,
            max_halstead_volume: acc.max_halstead_volume,
            avg_halstead_volume: acc.sum_halstead_volume / n,
            max_halstead_difficulty: acc.max_halstead_difficulty,
            avg_halstead_difficulty: acc.sum_halstead_difficulty / n,
            max_halstead_effort: acc.max_halstead_effort,
            avg_halstead_effort: acc.sum_halstead_effort / n,
            max_halstead_time: acc.max_halstead_time,
            avg_halstead_time: acc.sum_halstead_time / n,
            functions,
        }
    }

    pub fn from_functions(path: &Path, total_lines: usize, functions: Vec<FunctionComplexity>) -> Self {
        let count = functions.len();
        if count == 0 {
            return Self {
                path: path.to_path_buf(),
                total_lines,
                ..Default::default()
            };
        }

        let mut acc = FileResultAccumulator::new();
        for f in &functions {
            acc.add(f);
        }

        Self::from_accumulator(path, total_lines, functions, &acc, count)
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

fn sum_usize(files: &[&FileResult], extractor: fn(&FileResult) -> usize) -> usize {
    files.iter().map(|f| extractor(f)).sum()
}

fn sum_u32(files: &[&FileResult], extractor: fn(&FileResult) -> u32) -> u32 {
    files.iter().map(|f| extractor(f)).sum()
}

fn max_u32_from_files(files: &[&FileResult], extractor: fn(&FileResult) -> u32) -> u32 {
    files.iter().map(|f| extractor(f)).max().unwrap_or(0)
}

fn safe_div(numerator: f64, denominator: f64) -> f64 {
    if denominator > 0.0 {
        numerator / denominator
    } else {
        0.0
    }
}

fn weighted_avg(
    files: &[&FileResult],
    total_functions: usize,
    extractor: fn(&FileResult) -> f64,
) -> f64 {
    if total_functions == 0 {
        return 0.0;
    }
    files
        .iter()
        .map(|f| extractor(f) * f.function_count as f64)
        .sum::<f64>()
        / total_functions as f64
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
        let total_functions = sum_usize(&successful, |f| f.function_count);
        let total_lines = sum_usize(&successful, |f| f.total_lines);
        let total_complexity = sum_u32(&successful, |f| f.total_complexity);
        let avg_complexity = safe_div(total_complexity as f64, total_functions as f64);
        let max_nesting = max_u32_from_files(&successful, |f| f.max_nesting_depth);

        SummaryStatistics {
            files_analyzed: total_files,
            total_functions,
            total_lines,
            total_complexity,
            avg_complexity_per_function: avg_complexity,
            max_nesting_depth: max_nesting,
            avg_nesting_depth: weighted_avg(&successful, total_functions, |f| f.avg_nesting_depth),
            avg_halstead_volume: weighted_avg(&successful, total_functions, |f| f.avg_halstead_volume),
            avg_halstead_difficulty: weighted_avg(&successful, total_functions, |f| f.avg_halstead_difficulty),
            avg_halstead_effort: weighted_avg(&successful, total_functions, |f| f.avg_halstead_effort),
            avg_halstead_time: weighted_avg(&successful, total_functions, |f| f.avg_halstead_time),
        }
    }
}

pub mod analyzer;
pub mod cognitive;
pub mod complexity;
pub mod language;
pub mod output;

pub use analyzer::analyze_path;
