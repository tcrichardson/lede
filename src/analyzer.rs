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
                    return Ok(FileResult {
                        path: path.to_path_buf(),
                        total_complexity: total,
                        total_lines,
                        function_count,
                        functions,
                        error: None,
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
    })
}
