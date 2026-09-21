use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rpwd() -> Command {
    Command::cargo_bin("rpwd").unwrap()
}

#[test]
fn prints_current_directory() {
    let dir = TempDir::new().unwrap();
    let expected = dir.path().canonicalize().unwrap();

    rpwd()
        .current_dir(dir.path())
        .env_remove("PWD")
        .assert()
        .success()
        .stdout(format!("{}\n", expected.display()));
}

#[test]
fn logical_mode_uses_pwd_env_when_valid() {
    let dir = TempDir::new().unwrap();
    let pwd = dir.path().to_string_lossy().to_string();

    rpwd()
        .current_dir(dir.path())
        .env("PWD", &pwd)
        .assert()
        .success()
        .stdout(format!("{pwd}\n"));
}

#[test]
fn logical_mode_ignores_stale_pwd_env() {
    let dir = TempDir::new().unwrap();
    let expected = dir.path().canonicalize().unwrap();

    rpwd()
        .current_dir(dir.path())
        .env("PWD", "/definitely/not/the/current/directory")
        .assert()
        .success()
        .stdout(format!("{}\n", expected.display()));
}

#[test]
fn physical_mode_prints_canonical_path() {
    let dir = TempDir::new().unwrap();

    rpwd()
        .current_dir(dir.path())
        .arg("-P")
        .env_remove("PWD")
        .assert()
        .success()
        .stdout(format!("{}\n", dir.path().canonicalize().unwrap().display()));
}

#[cfg(unix)]
#[test]
fn physical_mode_resolves_symlinks() {
    let root = TempDir::new().unwrap();
    let target = root.path().join("target");
    let link = root.path().join("link");
    fs::create_dir(&target).unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();

    rpwd()
        .current_dir(&link)
        .arg("--physical")
        .env_remove("PWD")
        .assert()
        .success()
        .stdout(format!("{}\n", target.canonicalize().unwrap().display()));
}

#[test]
fn help_flag_works() {
    rpwd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Output the current working directory"));
}

#[test]
fn version_flag_works() {
    rpwd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rpwd"));
}
