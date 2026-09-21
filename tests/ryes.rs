use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Read;
use std::process::Stdio;
use std::time::Duration;

fn ryes() -> Command {
    Command::cargo_bin("ryes").unwrap()
}

fn collect_repeated_output(args: &[&str]) -> String {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_ryes"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    std::thread::sleep(Duration::from_millis(100));

    let mut buffer = vec![0u8; 4096];
    let len = child
        .stdout
        .as_mut()
        .unwrap()
        .read(&mut buffer)
        .unwrap_or(0);
    child.kill().unwrap();
    child.wait().unwrap();

    String::from_utf8_lossy(&buffer[..len]).into_owned()
}

#[test]
fn prints_y_by_default() {
    let stdout = collect_repeated_output(&[]);
    assert!(stdout.starts_with("y\n"));
    assert!(stdout[2..].contains("y\n"));
}

#[test]
fn prints_custom_string() {
    let stdout = collect_repeated_output(&["no"]);
    assert!(stdout.starts_with("no\n"));
    assert!(stdout[3..].contains("no\n"));
}

#[test]
fn joins_multiple_arguments() {
    let stdout = collect_repeated_output(&["hello", "world"]);
    assert!(stdout.starts_with("hello world\n"));
    assert!(stdout[12..].contains("hello world\n"));
}

#[test]
fn help_flag_works() {
    ryes()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Be repetitively affirmative"));
}

#[test]
fn version_flag_works() {
    ryes()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ryes"));
}
