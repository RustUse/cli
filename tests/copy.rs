//! `copy` planning behavior.

mod common;

use common::{CliBinary, TempProject, run_raw_in};

#[test]
fn copy_reports_plan_for_named_crate() {
    let project = TempProject::new();
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["copy", "use-geometry"]);

    assert!(output.status.success(), "copy should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("use-geometry"),
        "missing crate name:\n{stdout}"
    );
}
