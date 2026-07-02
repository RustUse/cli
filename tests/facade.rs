//! `facade` subcommand surface.

mod common;

use common::{CliBinary, run_help, run_raw};

#[test]
fn facade_help_lists_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["facade"]);

    for subcommand in ["inspect", "scan", "check", "report"] {
        assert!(
            stdout.contains(subcommand),
            "facade help missing `{subcommand}`:\n{stdout}"
        );
    }
}

#[test]
fn facade_without_subcommand_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["facade"]);

    assert!(
        !output.status.success(),
        "bare `facade` should require a subcommand"
    );
}

#[test]
fn facade_inspect_runs_real_backend() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("demo");

    let output = common::run_raw_in(&bin, temp.path(), &["facade", "inspect", "."]);

    assert!(
        output.status.success(),
        "expected success for `facade inspect .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse facade inspect"),
        "inspect output missing summary:\n{stdout}"
    );
}

#[test]
fn facade_check_deny_warnings_fails_on_incomplete_repo() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::new();

    let output = common::run_raw_in(
        &bin,
        temp.path(),
        &["facade", "check", "--deny-warnings", "."],
    );

    assert!(
        !output.status.success(),
        "expected `facade check --deny-warnings` to fail on incomplete repo"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deny-warnings") || stderr.contains("warning"),
        "stderr missing deny-warnings failure context:\n{stderr}"
    );
}

#[test]
fn facade_report_writes_default_markdown_report() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("report");

    let output = common::run_raw_in(&bin, temp.path(), &["facade", "report", "."]);

    assert!(
        output.status.success(),
        "expected success for `facade report .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_path = temp.path().join("rustuse-report.md");
    assert!(
        report_path.is_file(),
        "expected report file at {}",
        report_path.display()
    );
}
