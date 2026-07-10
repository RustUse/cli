//! `ci` is the stable CI automation entrypoint.

mod common;

use common::{CliBinary, TempProject, run_help, run_raw, run_raw_in};

#[test]
fn ci_help_lists_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["ci"]);

    assert!(
        stdout.contains("check"),
        "ci help missing `check`:\n{stdout}"
    );
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
fn ci_check_deny_warnings_fails_on_incomplete_facade() {
    let bin = CliBinary::rustuse();
    let project = TempProject::new();
    let output = run_raw_in(
        &bin,
        project.path(),
        &["ci", "check", "--deny-warnings", "."],
    );

    assert!(
        !output.status.success(),
        "ci check should fail for an incomplete facade when warnings are denied"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--deny-warnings"),
        "stderr missing denial context:\n{stderr}"
    );
}
