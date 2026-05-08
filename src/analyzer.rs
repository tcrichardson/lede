use crate::{
    FileResult, FunctionComplexity,
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
    let total_lines = source.lines().count();

    for analyzer in ANALYZERS {
        if analyzer.can_analyze(path) {
            match analyzer.analyze(&source) {
                Ok(functions) => return Ok(build_success_result(path, total_lines, functions)),
                Err(e) => return Ok(build_error_result(path, total_lines, e)),
            }
        }
    }

    Ok(build_empty_result(path, total_lines))
}

fn build_success_result(path: &Path, total_lines: usize, functions: Vec<FunctionComplexity>) -> FileResult {
    let function_count = functions.len();
    let total_complexity: u32 = functions.iter().map(|f| f.complexity).sum();
    let total_function_lines: usize = functions.iter().map(|f| f.lines).sum();
    let max_complexity = functions.iter().map(|f| f.complexity).max().unwrap_or(0);
    let max_function_lines = functions.iter().map(|f| f.lines).max().unwrap_or(0);
    let max_nesting_depth = functions.iter().map(|f| f.nesting_depth).max().unwrap_or(0);

    let max_halstead_volume = functions.iter().map(|f| f.halstead_volume).fold(0.0_f64, f64::max);
    let max_halstead_difficulty = functions.iter().map(|f| f.halstead_difficulty).fold(0.0_f64, f64::max);
    let max_halstead_effort = functions.iter().map(|f| f.halstead_effort).fold(0.0_f64, f64::max);
    let max_halstead_time = functions.iter().map(|f| f.halstead_time).fold(0.0_f64, f64::max);

    let avg = |extractor: fn(&FunctionComplexity) -> f64| -> f64 {
        if function_count == 0 {
            0.0
        } else {
            functions.iter().map(extractor).sum::<f64>() / function_count as f64
        }
    };

    let avg_nesting_depth = avg(|f| f.nesting_depth as f64);
    let avg_halstead_volume = avg(|f| f.halstead_volume);
    let avg_halstead_difficulty = avg(|f| f.halstead_difficulty);
    let avg_halstead_effort = avg(|f| f.halstead_effort);
    let avg_halstead_time = avg(|f| f.halstead_time);

    FileResult {
        path: path.to_path_buf(),
        total_complexity,
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
    }
}

fn build_error_result(path: &Path, total_lines: usize, error: String) -> FileResult {
    FileResult {
        path: path.to_path_buf(),
        total_lines,
        error: Some(error),
        ..Default::default()
    }
}

fn build_empty_result(path: &Path, total_lines: usize) -> FileResult {
    FileResult {
        path: path.to_path_buf(),
        total_lines,
        ..Default::default()
    }
}
