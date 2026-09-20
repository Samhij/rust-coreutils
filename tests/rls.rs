use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rls() -> Command {
    Command::cargo_bin("rls").unwrap()
}

#[test]
fn lists_directory_contents() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("visible.txt"), "").unwrap();

    rls()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("visible.txt"));
}

#[test]
fn hides_hidden_files_by_default() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".hidden"), "").unwrap();
    fs::write(dir.path().join("visible.txt"), "").unwrap();

    let assert = rls().arg(dir.path()).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(!stdout.contains(".hidden"));
    assert!(stdout.contains("visible.txt"));
}

#[test]
fn show_all_includes_hidden_files() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".hidden"), "").unwrap();
    fs::write(dir.path().join("visible.txt"), "").unwrap();

    rls()
        .args(["-a", &dir.path().to_string_lossy()])
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"))
        .stdout(predicate::str::contains("visible.txt"));
}

#[test]
fn long_format_includes_permissions_and_filename() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("sample.txt"), "data").unwrap();

    rls()
        .args(["-l", &dir.path().to_string_lossy()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[-dl][rwx-]{9}[@ ]").unwrap())
        .stdout(predicate::str::contains("sample.txt"));
}

#[test]
fn long_format_all_flag_combined() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".dotfile"), "").unwrap();

    rls()
        .args(["-la", &dir.path().to_string_lossy()])
        .assert()
        .success()
        .stdout(predicate::str::contains(".dotfile"));
}

#[test]
fn defaults_to_current_directory() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("here.txt"), "").unwrap();

    rls()
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("here.txt"));
}

#[test]
fn lists_multiple_paths() {
    let root = TempDir::new().unwrap();
    let first = root.path().join("first");
    let second = root.path().join("second");
    fs::create_dir(&first).unwrap();
    fs::create_dir(&second).unwrap();
    fs::write(first.join("a.txt"), "").unwrap();
    fs::write(second.join("b.txt"), "").unwrap();

    rls()
        .arg(&first)
        .arg(&second)
        .assert()
        .success()
        .stdout(predicate::str::contains("a.txt"))
        .stdout(predicate::str::contains("b.txt"));
}

#[test]
fn missing_directory_exits_with_error() {
    rls()
        .arg("no_such_directory")
        .assert()
        .failure()
        .stderr(predicate::str::contains("rls: no_such_directory"));
}

#[test]
fn help_flag_works() {
    rls()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List directory contents"));
}

#[test]
fn version_flag_works() {
    rls()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rls"));
}
