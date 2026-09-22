use assert_cmd::Command;
use filetime::{FileTime, set_file_mtime};
use predicates::prelude::*;
use std::fs;
use std::time::SystemTime;
use tempfile::TempDir;

fn rtouch() -> Command {
    Command::cargo_bin("rtouch").unwrap()
}

#[test]
fn creates_new_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("new.txt");

    rtouch()
        .arg(&file)
        .assert()
        .success()
        .stdout("");

    assert!(file.is_file());
}

#[test]
fn updates_modification_time_on_existing_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("existing.txt");
    fs::write(&file, "data").unwrap();
    set_file_mtime(&file, FileTime::from_unix_time(1, 0)).unwrap();

    let before = SystemTime::now();
    rtouch()
        .arg(&file)
        .assert()
        .success();

    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    assert!(modified >= before);
}

#[test]
fn preserves_file_contents_when_touching() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("content.txt");
    fs::write(&file, "keep me").unwrap();

    rtouch()
        .arg(&file)
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "keep me");
}

#[test]
fn no_create_does_not_create_missing_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("absent.txt");

    rtouch()
        .args(["-c", &file.to_string_lossy()])
        .assert()
        .success()
        .stdout("");

    assert!(!file.exists());
}

#[test]
fn no_create_still_touches_existing_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("present.txt");
    fs::write(&file, "").unwrap();
    set_file_mtime(&file, FileTime::from_unix_time(1, 0)).unwrap();

    let before = SystemTime::now();
    rtouch()
        .args(["-c", &file.to_string_lossy()])
        .assert()
        .success();

    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    assert!(modified >= before);
}

#[test]
fn processes_multiple_files() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "a").unwrap();

    rtouch()
        .args([&a, &b])
        .assert()
        .success();

    assert!(a.is_file());
    assert!(b.is_file());
}

#[test]
fn continues_after_error_and_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    let ok = dir.path().join("ok.txt");

    rtouch()
        .args(["/no/such/directory/file.txt", &ok.to_string_lossy()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("rtouch: /no/such/directory/file.txt"));

    assert!(ok.is_file());
}

#[test]
fn help_flag_works() {
    rtouch()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Change file access and modification times"));
}

#[test]
fn version_flag_works() {
    rtouch()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rtouch"));
}
