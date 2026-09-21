use assert_cmd::Command;
use predicates::prelude::*;

fn recho() -> Command {
    Command::cargo_bin("recho").unwrap()
}

#[test]
fn prints_arguments_with_trailing_newline() {
    recho()
        .args(["hello", "world"])
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn single_argument() {
    recho()
        .arg("hello")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn no_arguments_prints_blank_line() {
    recho()
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn no_newline_flag_omits_trailing_newline() {
    recho()
        .args(["-n", "hello"])
        .assert()
        .success()
        .stdout("hello");
}

#[test]
fn help_flag_works() {
    recho()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Write arguments to the standard output"));
}

#[test]
fn version_flag_works() {
    recho()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("recho"));
}
