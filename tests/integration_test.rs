use std::process::{Command, Stdio};
use std::io::Write;

fn run_tablify_with_input(input: &str, args: &[&str]) -> String {
    let mut cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start tablify");

    if let Some(stdin) = cmd.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    }

    let output = cmd.wait_with_output().expect("Failed to read output");
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}

#[test]
fn test_basic_tab_separated() {
    let input = "apple\t100\norange\t200";
    let output = run_tablify_with_input(input, &[]);
    assert!(output.contains("| apple  | 100 |"));
    assert!(output.contains("| orange | 200 |"));
}

#[test]
fn test_custom_separator() {
    let input = "apple 100\norange 200";
    let output = run_tablify_with_input(input, &["-s", " "]);
    assert!(output.contains("| apple  | 100 |"));
    assert!(output.contains("| orange | 200 |"));
}

#[test]
fn test_header_option() {
    let input = "item\tprice\napple\t100\norange\t200";
    let output = run_tablify_with_input(input, &["--header"]);
    assert!(output.contains("| item   | price |"));
    assert!(output.contains("+--------+-------+"));
    assert!(output.contains("| apple  | 100   |"));
    assert!(output.contains("| orange | 200   |"));
}

#[test]
fn test_custom_columns() {
    let input = "apple\t100\norange\t200";
    let output = run_tablify_with_input(input, &["--columns", "fruit,price"]);
    assert!(output.contains("| fruit  | price |"));
    assert!(output.contains("+--------+-------+"));
    assert!(output.contains("| apple  | 100   |"));
    assert!(output.contains("| orange | 200   |"));
}

#[test]
fn test_regex_pattern() {
    let input = "apple   100\norange  200";
    let output = run_tablify_with_input(input, &["-p", r"\s+"]);
    assert!(output.contains("| apple  | 100 |"));
    assert!(output.contains("| orange | 200 |"));
}

#[test]
fn test_full_width_characters() {
    let input = "りんご\t100\nオレンジ\t200";
    let output = run_tablify_with_input(input, &[]);
    assert!(output.contains("| りんご   | 100 |"));
    assert!(output.contains("| オレンジ | 200 |"));
}

#[test]
fn test_column_formatting_basic() {
    let input = "apple\t100\norange\t200";
    let output = run_tablify_with_input(input, &["--format", "1:left,2:right"]);
    assert!(output.contains("| apple  | 100 |"));
    assert!(output.contains("| orange | 200 |"));
}

#[test]
fn test_column_formatting_center() {
    let input = "apple\t100\tgood\norange\t200\tbad";
    let output = run_tablify_with_input(input, &["--format", "1:left,2:right,3:center"]);
    assert!(output.contains("| apple  | 100 | good |"));
    assert!(output.contains("| orange | 200 | bad  |"));
}

#[test]
fn test_column_formatting_with_header() {
    let input = "item\tprice\trating\napple\t100\tgood\norange\t200\tbad";
    let output = run_tablify_with_input(input, &["--header", "--format", "1:left,2:right,3:center"]);
    assert!(output.contains("| item   | price | rating |"));
    assert!(output.contains("| apple  |   100 |  good  |"));
    assert!(output.contains("| orange |   200 |  bad   |"));
}

#[test]
fn test_column_formatting_partial_spec() {
    let input = "apple\t100\tgood\norange\t200\tbad";
    let output = run_tablify_with_input(input, &["--format", "2:right"]);
    assert!(output.contains("| apple  | 100 | good |"));
    assert!(output.contains("| orange | 200 | bad  |"));
}

#[test]
fn test_column_formatting_with_full_width() {
    let input = "りんご\t100\nオレンジ\t200";
    let output = run_tablify_with_input(input, &["--format", "1:center,2:right"]);
    assert!(output.contains("|  りんご  | 100 |"));
    assert!(output.contains("| オレンジ | 200 |"));
}