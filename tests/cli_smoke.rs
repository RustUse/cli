//! Smoke tests for the compiled `rustuse` executable.

mod common;

use common::{CliBinary, run_raw};

#[test]
fn help_succeeds() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--help"]);

    assert!(
        output.status.success(),
        "expected `rustuse --help` to succeed"
    );
}

#[test]
fn non_interactive_without_command_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--non-interactive"]);

    assert!(
        !output.status.success(),
        "expected failure without an explicit command"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("a command is required"),
        "stderr missing helpful message:\n{stderr}"
    );
}

#[test]
fn yes_without_command_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--yes"]);

    assert!(
        !output.status.success(),
        "expected failure without an explicit command"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("a command is required"),
        "stderr missing helpful message:\n{stderr}"
    );
}
