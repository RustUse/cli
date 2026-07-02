//! `ci` is the stable CI automation entrypoint.

mod common;

use common::{CliBinary, run_help, run_raw, run_success};

#[test]
fn ci_help_lists_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["ci"]);

    for subcommand in ["check", "github-actions", "trusted-publishing", "report"] {
        assert!(
            stdout.contains(subcommand),
            "ci help missing `{subcommand}`:\n{stdout}"
        );
    }
}

#[test]
fn ci_without_subcommand_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["ci"]);

    assert!(
        !output.status.success(),
        "bare `ci` should require a subcommand"
    );
}

#[test]
fn ci_check_marks_staged_behavior() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["ci", "check"]);

    assert!(
        stdout.contains("staged=true"),
        "ci check output should mark staged behavior:\n{stdout}"
    );
}
