use crate::{
    FileResult,
    language::{javascript::JavaScriptAnalyzer, python::PythonAnalyzer, rust::RustAnalyzer, LanguageAnalyzer},
};
use std::path::Path;
use walkdir::WalkDir;

static ANALYZERS: &[&dyn LanguageAnalyzer] = &[
    &RustAnalyzer,
    &PythonAnalyzer,
    &JavaScriptAnalyzer,
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
                    let avg_cognitive_load = if function_count > 0 {
                        functions.iter().map(|f| f.cognitive_load).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    let max_nesting_depth = functions.iter().map(|f| f.nesting_depth).max().unwrap_or(0);
                    let avg_halstead_volume = if function_count > 0 {
                        functions.iter().map(|f| f.halstead_volume).sum::<f64>() / function_count as f64
                    } else { 0.0 };
                    return Ok(FileResult {
                        path: path.to_path_buf(),
                        total_complexity: total,
                        total_lines,
                        function_count,
                        functions,
                        error: None,
                        avg_cognitive_load,
                        max_nesting_depth,
                        avg_halstead_volume,
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
                        avg_cognitive_load: 0.0,
                        max_nesting_depth: 0,
                        avg_halstead_volume: 0.0,
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
        avg_cognitive_load: 0.0,
        max_nesting_depth: 0,
        avg_halstead_volume: 0.0,
    })
}
