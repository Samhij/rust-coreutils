use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rcat() -> Command {
    Command::cargo_bin("rcat").unwrap()
}

#[test]
fn prints_file_contents() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("hello.txt");
    fs::write(&file, "hello world\n").unwrap();

    rcat()
        .arg(&file)
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenates_multiple_files() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "aaa\n").unwrap();
    fs::write(&b, "bbb\n").unwrap();

    rcat()
        .args([&a, &b])
        .assert()
        .success()
        .stdout("aaa\nbbb\n");
}

#[test]
fn reads_from_stdin_when_no_files_given() {
    rcat()
        .write_stdin("piped input\n")
        .assert()
        .success()
        .stdout("piped input\n");
}

#[test]
fn reads_from_stdin_when_dash_is_given() {
    rcat()
        .arg("-")
        .write_stdin("explicit stdin\n")
        .assert()
        .success()
        .stdout("explicit stdin\n");
}

#[test]
fn empty_file_produces_no_output() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.txt");
    fs::write(&file, "").unwrap();

    rcat()
        .arg(&file)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn missing_file_exits_with_error() {
    rcat()
        .arg("nonexistent.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cat: nonexistent.txt"));
}

#[test]
fn continues_after_missing_file_but_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("exists.txt");
    fs::write(&file, "ok\n").unwrap();

    rcat()
        .args(["missing.txt", &file.to_string_lossy()])
        .assert()
        .failure()
        .stdout("ok\n")
        .stderr(predicate::str::contains("cat: missing.txt"));
}

#[test]
fn help_flag_works() {
    rcat()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Concatenate FILE(s)"));
}

#[test]
fn version_flag_works() {
    rcat()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rcat"));
}
