use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rtail() -> Command {
    Command::cargo_bin("rtail").unwrap()
}

fn numbered_lines(count: usize) -> String {
    (1..=count)
        .map(|n| format!("line {n}\n"))
        .collect()
}

fn last_numbered_lines(total: usize, count: usize) -> String {
    let start = total.saturating_sub(count).saturating_add(1);
    (start..=total)
        .map(|n| format!("line {n}\n"))
        .collect()
}

#[test]
fn prints_last_ten_lines_by_default() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(15)).unwrap();

    rtail()
        .arg(&file)
        .assert()
        .success()
        .stdout(last_numbered_lines(15, 10));
}

#[test]
fn custom_line_count_with_short_flag() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(10)).unwrap();

    rtail()
        .args(["-n", "3"])
        .arg(&file)
        .assert()
        .success()
        .stdout(last_numbered_lines(10, 3));
}

#[test]
fn custom_line_count_with_long_flag() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lines.txt");
    fs::write(&file, numbered_lines(10)).unwrap();

    rtail()
        .args(["--lines", "2"])
        .arg(&file)
        .assert()
        .success()
        .stdout(last_numbered_lines(10, 2));
}

#[test]
fn file_with_fewer_lines_than_requested() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("short.txt");
    let content = "only one line\n";
    fs::write(&file, content).unwrap();

    rtail()
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

    let expected = format!(
        "==> {} <==\nalpha\n\n==> {} <==\nbeta\n",
        a.display(),
        b.display(),
    );

    rtail()
        .args(["-n", "1", &a.to_string_lossy(), &b.to_string_lossy()])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn reads_from_stdin_when_no_files_given() {
    rtail()
        .write_stdin("stdin line 1\nstdin line 2\n")
        .assert()
        .success()
        .stdout("stdin line 1\nstdin line 2\n");
}

#[test]
fn reads_from_stdin_when_dash_is_given() {
    rtail()
        .args(["-n", "1", "-"])
        .write_stdin("first\nsecond\n")
        .assert()
        .success()
        .stdout("second\n");
}

#[test]
fn empty_file_produces_no_output() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.txt");
    fs::write(&file, "").unwrap();

    rtail()
        .arg(&file)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn missing_file_exits_with_error() {
    rtail()
        .arg("missing.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("tail: missing.txt"));
}

#[test]
fn help_flag_works() {
    rtail()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Prints the last 10 lines"));
}

#[test]
fn version_flag_works() {
    rtail()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rtail"));
}
