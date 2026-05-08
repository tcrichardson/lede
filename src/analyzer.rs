use crate::{
    FileResult,
    language::{c::CAnalyzer, javascript::JavaScriptAnalyzer, python::PythonAnalyzer, rust::RustAnalyzer, LanguageAnalyzer},
};
use std::path::Path;
use walkdir::WalkDir;

static ANALYZERS: &[&dyn LanguageAnalyzer] = &[
    &RustAnalyzer,
    &PythonAnalyzer,
    &JavaScriptAnalyzer,
    &CAnalyzer,
];

pub fn analyze_path(path: &Path) -> Result<Vec<FileResult>, std::io::Error> {
    let mut results = Vec::new();

    if path.is_file() {
        results.push(analyze_file(path)?);
    } else if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                results.push(analyze_file(p)?);
            }
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} is not a file or directory", path.display()),
        ));
    }

    Ok(results)
}

fn analyze_file(path: &Path) -> Result<FileResult, std::io::Error> {
    let source = std::fs::read_to_string(path)?;

    for analyzer in ANALYZERS {
        if analyzer.can_analyze(path) {
            match analyzer.analyze(&source) {
                Ok(functions) => {
                    let total = functions.iter().map(|f| f.complexity).sum();
                    let total_lines = source.lines().count();
                    let function_count = functions.len();
                    let max_nesting_depth = functions.iter().map(|f| f.nesting_depth).max().unwrap_or(0);
                    let avg_nesting_depth = if function_count > 0 {
                        functions.iter().map(|f| f.nesting_depth as f64).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let avg_halstead_volume = if function_count > 0 {
                        functions.iter().map(|f| f.halstead_volume).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let avg_halstead_difficulty = if function_count > 0 {
                        functions.iter().map(|f| f.halstead_difficulty).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let avg_halstead_effort = if function_count > 0 {
                        functions.iter().map(|f| f.halstead_effort).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let avg_halstead_time = if function_count > 0 {
                        functions.iter().map(|f| f.halstead_time).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let max_complexity = functions.iter().map(|f| f.complexity).max().unwrap_or(0);
                    let max_function_lines = functions.iter().map(|f| f.lines).max().unwrap_or(0);
                    let total_function_lines: usize = functions.iter().map(|f| f.lines).sum();
                    let max_halstead_volume = functions.iter().map(|f| f.halstead_volume).fold(0.0_f64, f64::max);
                    let max_halstead_difficulty = functions.iter().map(|f| f.halstead_difficulty).fold(0.0_f64, f64::max);
                    let max_halstead_effort = functions.iter().map(|f| f.halstead_effort).fold(0.0_f64, f64::max);
                    let max_halstead_time = functions.iter().map(|f| f.halstead_time).fold(0.0_f64, f64::max);
                    return Ok(FileResult {
                        path: path.to_path_buf(),
                        total_complexity: total,
                        total_lines,
                        function_count,
                        functions,
                        error: None,
                        max_nesting_depth,
                        avg_nesting_depth,
                        avg_halstead_volume,
                        avg_halstead_difficulty,
                        avg_halstead_effort,
                        avg_halstead_time,
                        max_complexity,
                        max_function_lines,
                        total_function_lines,
                        max_halstead_volume,
                        max_halstead_difficulty,
                        max_halstead_effort,
                        max_halstead_time,
                    });
                }
                Err(e) => {
                    return Ok(FileResult {
                        path: path.to_path_buf(),
                        total_complexity: 0,
                        total_lines: source.lines().count(),
                        function_count: 0,
                        functions: Vec::new(),
                        error: Some(e),
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
                    });
                }
            }
        }
    }

    // Unsupported extension — skip silently
    Ok(FileResult {
        path: path.to_path_buf(),
        total_complexity: 0,
        total_lines: source.lines().count(),
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
    })
}
