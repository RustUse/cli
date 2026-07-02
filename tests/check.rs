//! `check` validates local RustUse/Cargo/repository state.

mod common;

use common::{CliBinary, TempProject, run_help, run_raw_in};

#[test]
fn check_runs_against_a_directory() {
    let project = TempProject::new();
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["check", "."]);

    assert!(output.status.success(), "check should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check"), "missing check output:\n{stdout}");
}

#[test]
fn check_help_lists_kinds() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["check"]);

    assert!(
        stdout.contains("--kind"),
        "check help missing --kind:\n{stdout}"
    );
}
