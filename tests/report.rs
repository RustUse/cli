//! `report --stdout` against temporary root and facade fixtures.

mod common;

use common::{CliBinary, TempProject, run_raw_in};

#[test]
fn report_root_to_stdout() {
    let project = TempProject::root_with_facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(
        &bin,
        project.path(),
        &["report", ".", "--kind", "root", "--stdout"],
    );

    assert!(output.status.success(), "root report should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse Development Root Report"),
        "missing root report heading:\n{stdout}"
    );
    assert!(
        !project.path().join("rustuse-root-report.md").exists(),
        "--stdout must not write a report file"
    );
}

#[test]
fn report_facade_to_stdout() {
    let project = TempProject::facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(
        &bin,
        project.path(),
        &["report", ".", "--kind", "facade", "--stdout"],
    );

    assert!(output.status.success(), "facade report should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse Facade Report"),
        "missing facade report heading:\n{stdout}"
    );
    assert!(
        !project.path().join("rustuse-report.md").exists(),
        "--stdout must not write a report file"
    );
}
