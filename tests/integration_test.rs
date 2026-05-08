use std::process::Command;

fn rubik() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--");
    cmd
}

#[test]
fn test_rust_fixture_pretty() {
    let output = rubik()
        .arg("tests/fixtures/rust_sample.rs")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("simple"));
    assert!(stdout.contains("with_if"));
    assert!(stdout.contains("with_match"));
    assert!(stdout.contains("nested"));
    assert!(!stdout.contains("<closure>"));
    assert!(stdout.contains("Halstead Vol"));
    assert!(stdout.contains("Nesting"));
}

#[test]
fn test_rust_fixture_include_closures() {
    let output = rubik()
        .arg("tests/fixtures/rust_sample.rs")
        .arg("--include-closures")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("simple"));
    assert!(stdout.contains("nested"));
    assert!(stdout.contains("<closure>"));
}

#[test]
fn test_python_fixture_json() {
    let output = rubik()
        .arg("tests/fixtures/python_sample.py")
        .arg("-f")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.summary.files_analyzed, 1);
    let file = &parsed.files[0];
    assert!(file.path.to_string_lossy().contains("python_sample.py"));
    let names: Vec<&str> = file.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"simple"));
    assert!(names.contains(&"with_if"));
    assert!(names.contains(&"with_match"));
    assert!(names.contains(&"nested"));
    assert!(file.functions.iter().any(|f| f.halstead_effort > 0.0));
    assert!(file.avg_halstead_effort > 0.0);
    assert!(file.avg_halstead_volume >= 0.0);
    assert!(parsed.summary.total_functions > 0);
    assert!(parsed.summary.total_lines > 0);
}

#[test]
fn test_js_fixture_json() {
    let output = rubik()
        .arg("tests/fixtures/js_sample.js")
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.summary.files_analyzed, 1);
    let file = &parsed.files[0];
    assert!(file.path.to_string_lossy().contains("js_sample.js"));
    let names: Vec<&str> = file.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"simple"));
    assert!(names.contains(&"withIf"));
    assert!(file.functions.iter().any(|f| f.halstead_effort > 0.0));
    assert!(file.avg_halstead_effort > 0.0);
    assert!(file.avg_halstead_volume >= 0.0);
    assert!(parsed.summary.total_functions > 0);
    assert!(parsed.summary.total_lines > 0);
}

#[test]
fn test_c_fixture_json() {
    let output = rubik()
        .arg("tests/fixtures/c_sample.c")
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.summary.files_analyzed, 1);
    let file = &parsed.files[0];
    assert!(file.path.to_string_lossy().contains("c_sample.c"));
    let names: Vec<&str> = file.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"simple"));
    assert!(names.contains(&"withIf"));
    assert!(names.contains(&"withSwitch"));
    assert!(names.contains(&"nested"));
    assert!(file.functions.iter().any(|f| f.halstead_effort > 0.0));
    assert!(file.avg_halstead_effort > 0.0);
    assert!(file.avg_halstead_volume >= 0.0);
    assert!(parsed.summary.total_functions > 0);
    assert!(parsed.summary.total_lines > 0);
}

#[test]
fn test_invalid_file_skips() {
    let output = rubik()
        .arg("tests/fixtures/invalid.py")
        .output()
        .expect("failed to run rubik");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error parsing") || stderr.contains("Failed to parse"));
    assert!(output.status.success());
}

#[test]
fn test_directory_scan() {
    let output = rubik()
        .arg("tests/fixtures")
        .arg("-f")
        .arg("json")
        .output()
        .expect("failed to run rubik");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: rubik::AnalysisOutput = serde_json::from_str(&stdout).expect("invalid JSON");
    let paths: Vec<String> = parsed.files.iter().map(|r| r.path.to_string_lossy().to_string()).collect();
    assert!(paths.iter().any(|p| p.contains("rust_sample.rs")));
    assert!(paths.iter().any(|p| p.contains("python_sample.py")));
    assert!(paths.iter().any(|p| p.contains("js_sample.js")));
    assert!(paths.iter().any(|p| p.contains("c_sample.c")));
    assert!(parsed.summary.files_analyzed >= 4);
}
