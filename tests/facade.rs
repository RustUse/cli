//! Facade maintainer workflows exposed through `dev` and `ci`.

mod common;

use common::{CliBinary, run_help, run_raw};

#[test]
fn dev_help_lists_facade_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["dev"]);

    for subcommand in ["inspect", "report"] {
        assert!(
            stdout.contains(subcommand),
            "dev help missing `{subcommand}`:\n{stdout}"
        );
    }
}

#[test]
fn dev_without_subcommand_fails_non_interactively() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--non-interactive", "dev"]);

    assert!(
        !output.status.success(),
        "bare `dev` should require a subcommand when non-interactive"
    );
}

#[test]
fn dev_inspect_runs_real_backend() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("demo");

    let output = common::run_raw_in(&bin, temp.path(), &["dev", "inspect", "."]);

    assert!(
        output.status.success(),
        "expected success for `dev inspect .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse facade inspect"),
        "inspect output missing summary:\n{stdout}"
    );
}

#[test]
fn ci_check_deny_warnings_fails_on_incomplete_repo() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::new();

    let output = common::run_raw_in(&bin, temp.path(), &["ci", "check", "--deny-warnings", "."]);

    assert!(
        !output.status.success(),
        "expected `ci check --deny-warnings` to fail on incomplete repo"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deny-warnings") || stderr.contains("warning"),
        "stderr missing deny-warnings failure context:\n{stderr}"
    );
}

#[test]
fn dev_report_writes_default_markdown_report() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("report");

    let output = common::run_raw_in(&bin, temp.path(), &["dev", "report", "."]);

    assert!(
        output.status.success(),
        "expected success for `dev report .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_path = temp.path().join("rustuse-report.md");
    assert!(
        report_path.is_file(),
        "expected report file at {}",
        report_path.display()
    );
}
