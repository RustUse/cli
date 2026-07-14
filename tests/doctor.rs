//! `doctor` inspects the current directory for project state.

mod common;

use common::{CliBinary, TempProject, run_raw_in};

#[test]
fn doctor_runs_in_a_plain_directory() {
    let project = TempProject::new();
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["doctor"]);

    assert!(output.status.success(), "doctor should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse doctor"),
        "missing doctor header:\n{stdout}"
    );
}

#[test]
fn doctor_json_reports_state() {
    let project = TempProject::new();
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["--json", "doctor"]);

    assert!(output.status.success(), "doctor --json should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"command\""),
        "missing json record:\n{stdout}"
    );
}
