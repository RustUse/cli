//! Root `--help` and non-interactive behavior.

mod common;

use common::{CliBinary, run_help, run_raw};

#[test]
fn help_shows_about_line() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &[]);

    assert!(
        stdout.contains("RustUse command-line adoption and maintenance helper."),
        "help output missing about line:\n{stdout}"
    );
}

#[test]
fn help_lists_core_commands() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &[]);

    for command in [
        "search", "scan", "report", "facade", "ci", "check", "ferris",
    ] {
        assert!(stdout.contains(command), "help output missing `{command}`");
    }
}

#[test]
fn help_has_no_dev_command() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &[]);

    assert!(!stdout.contains("\n  dev "), "dev command should not exist");
}

#[test]
fn non_interactive_without_command_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--non-interactive"]);

    assert!(
        !output.status.success(),
        "expected failure when no command is given with --non-interactive"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("a command is required"),
        "stderr missing helpful message:\n{stderr}"
    );
}

#[test]
fn yes_without_command_runs_safe_default_workflow() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--yes"]);

    assert!(
        output.status.success(),
        "expected success when no command is given with --yes"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse doctor"),
        "stdout missing doctor output:\n{stdout}"
    );
}
