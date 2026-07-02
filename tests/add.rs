//! `add` planning behavior.

mod common;

use common::{CliBinary, run_success};

#[test]
fn add_reports_plan_for_named_crate() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["add", "use-geometry", "--dry-run"]);

    assert!(
        stdout.contains("use-geometry"),
        "missing crate name:\n{stdout}"
    );
}
