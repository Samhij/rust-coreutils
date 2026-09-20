use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rhead() -> Command {
    Command::cargo_bin("rhead").unwrap()
}

fn numbered_lines(count: usize) -> String {
    (1..=count)
        .map(|n| format!("line {n}\n"))
        .collect()
}

#[test]
fn prints_first_ten_lines_by_default() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(15)).unwrap();

    rhead()
        .arg(&file)
        .assert()
        .success()
        .stdout(numbered_lines(10));
}

#[test]
fn custom_line_count_with_short_flag() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(10)).unwrap();

    rhead()
        .args(["-n", "3"])
        .arg(&file)
        .assert()
        .success()
        .stdout(numbered_lines(3));
}

#[test]
fn custom_line_count_with_long_flag() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(10)).unwrap();

    rhead()
        .args(["--lines", "2"])
        .arg(&file)
        .assert()
        .success()
        .stdout(numbered_lines(2));
}

#[test]
fn file_with_fewer_lines_than_requested() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("short.txt");
    let content = "only one line\n";
    fs::write(&file, content).unwrap();

    rhead()
        .arg(&file)
        .assert()
        .success()
        .stdout(content);
}

#[test]
fn processes_multiple_files_sequentially() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "alpha\n").unwrap();
    fs::write(&b, "beta\n").unwrap();

    rhead()
        .args(["-n", "1", &a.to_string_lossy(), &b.to_string_lossy()])
        .assert()
        .success()
        .stdout("alpha\nbeta\n");
}

#[test]
fn reads_from_stdin_when_no_files_given() {
    rhead()
        .write_stdin("stdin line 1\nstdin line 2\n")
        .assert()
        .success()
        .stdout("stdin line 1\nstdin line 2\n");
}

#[test]
fn reads_from_stdin_when_dash_is_given() {
    rhead()
        .args(["-n", "1", "-"])
        .write_stdin("first\nsecond\n")
        .assert()
        .success()
        .stdout("first\n");
}

#[test]
fn empty_file_produces_no_output() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.txt");
    fs::write(&file, "").unwrap();

    rhead()
        .arg(&file)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn missing_file_exits_with_error() {
    rhead()
        .arg("missing.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("head: missing.txt"));
}

#[test]
fn help_flag_works() {
    rhead()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Prints the first 10 lines"));
}

#[test]
fn version_flag_works() {
    rhead()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rhead"));
}
