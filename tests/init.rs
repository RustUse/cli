//! `init --dry-run` must not create files.

mod common;

use common::{CliBinary, TempProject, run_raw_in};

#[test]
fn init_dry_run_creates_no_files() {
    let project = TempProject::new();
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["init", "--dry-run"]);

    assert!(output.status.success(), "init --dry-run should succeed");
    assert!(
        !project.path().join("rustuse.toml").exists(),
        "init --dry-run must not write rustuse.toml"
    );
    assert!(
        !project.path().join(".rustuse").exists(),
        "init --dry-run must not create tracking directories"
    );
}
