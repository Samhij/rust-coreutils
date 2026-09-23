use assert_cmd::Command;
use predicates::prelude::*;

fn renv() -> Command {
    Command::cargo_bin("renv").unwrap()
}

#[test]
fn help_flag_works() {
    renv()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Set environment and run command"));
}

#[test]
fn version_flag_works() {
    renv()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("renv"));
}

#[test]
fn prints_only_custom_vars_with_ignore_environment() {
    let mut lines = vec!["BAZ=qux".to_string(), "FOO=bar".to_string()];
    lines.sort();
    let expected = format!("{}\n", lines.join("\n"));

    renv()
        .args(["-i", "FOO=bar", "BAZ=qux"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn unset_omits_var_from_printed_environment() {
    renv()
        .env("RENV_COREUTILS_UNSET_MARKER", "gone")
        .args(["-u", "RENV_COREUTILS_UNSET_MARKER"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RENV_COREUTILS_UNSET_MARKER=").not());
}

#[test]
fn runs_command_with_added_environment() {
    renv()
        .args([
            "RENV_COREUTILS_TEST=hello",
            "--",
            "/bin/sh",
            "-c",
            "echo $RENV_COREUTILS_TEST",
        ])
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn runs_command_with_multiple_assignments() {
    renv()
        .args([
            "A=1",
            "B=2",
            "--",
            "/bin/sh",
            "-c",
            "echo $A-$B",
        ])
        .assert()
        .success()
        .stdout("1-2\n");
}

#[test]
fn ignore_environment_drops_inherited_vars_in_child() {
    renv()
        .env("RENV_COREUTILS_INHERITED", "from_parent")
        .args([
            "-i",
            "PATH=/usr/bin:/bin",
            "RENV_COREUTILS_INHERITED=only_custom",
            "--",
            "/bin/sh",
            "-c",
            "echo ${RENV_COREUTILS_INHERITED:-missing}",
        ])
        .assert()
        .success()
        .stdout("only_custom\n");
}

#[test]
fn unset_removes_var_in_child() {
    renv()
        .env("RENV_COREUTILS_CHILD_UNSET", "present")
        .args([
            "-u",
            "RENV_COREUTILS_CHILD_UNSET",
            "--",
            "/bin/sh",
            "-c",
            "[ -z \"${RENV_COREUTILS_CHILD_UNSET+set}\" ] && echo unset",
        ])
        .assert()
        .success()
        .stdout("unset\n");
}

#[test]
fn forwards_command_exit_status() {
    renv()
        .args(["--", "/usr/bin/false"])
        .assert()
        .code(1);
}

#[test]
fn missing_command_reports_error_and_exits_127() {
    renv()
        .args(["--", "renv_coreutils_nonexistent_command"])
        .assert()
        .failure()
        .code(127)
        .stderr(predicate::str::contains(
            "renv: renv_coreutils_nonexistent_command:",
        ));
}
