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
    if !path.is_file() && !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} is not a file or directory", path.display()),
        ));
    }

    let mut results = Vec::new();

    if path.is_file() {
        results.push(analyze_file(path)?);
        return Ok(results);
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            results.push(analyze_file(p)?);
        }
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

fn max_u32(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> u32) -> u32 {
    functions.iter().map(extractor).max().unwrap_or(0)
}

fn max_usize(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> usize) -> usize {
    functions.iter().map(extractor).max().unwrap_or(0)
}

fn max_f64(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> f64) -> f64 {
    functions.iter().map(extractor).fold(0.0_f64, f64::max)
}

fn avg_f64(functions: &[FunctionComplexity], extractor: fn(&FunctionComplexity) -> f64) -> f64 {
    if functions.is_empty() {
        0.0
    } else {
        functions.iter().map(extractor).sum::<f64>() / functions.len() as f64
    }
}

fn build_success_result(path: &Path, total_lines: usize, functions: Vec<FunctionComplexity>) -> FileResult {
    let function_count = functions.len();
    let total_complexity: u32 = functions.iter().map(|f| f.complexity).sum();
    let total_function_lines: usize = functions.iter().map(|f| f.lines).sum();

    FileResult {
        path: path.to_path_buf(),
        total_complexity,
        total_lines,
        function_count,
        error: None,
        max_nesting_depth: max_u32(&functions, |f| f.nesting_depth),
        avg_nesting_depth: avg_f64(&functions, |f| f.nesting_depth as f64),
        avg_halstead_volume: avg_f64(&functions, |f| f.halstead_volume),
        avg_halstead_difficulty: avg_f64(&functions, |f| f.halstead_difficulty),
        avg_halstead_effort: avg_f64(&functions, |f| f.halstead_effort),
        avg_halstead_time: avg_f64(&functions, |f| f.halstead_time),
        max_complexity: max_u32(&functions, |f| f.complexity),
        max_function_lines: max_usize(&functions, |f| f.lines),
        total_function_lines,
        max_halstead_volume: max_f64(&functions, |f| f.halstead_volume),
        max_halstead_difficulty: max_f64(&functions, |f| f.halstead_difficulty),
        max_halstead_effort: max_f64(&functions, |f| f.halstead_effort),
        max_halstead_time: max_f64(&functions, |f| f.halstead_time),
        functions,
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
