//! `scan` against temporary facade and root fixtures.

mod common;

use common::{CliBinary, TempProject, run_raw, run_raw_in};

#[test]
fn scan_detects_a_facade_repository() {
    let project = TempProject::facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["scan", "."]);

    assert!(output.status.success(), "scan facade should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("facade scan"),
        "missing facade summary:\n{stdout}"
    );
}

#[test]
fn scan_detects_a_development_root() {
    let project = TempProject::root_with_facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["scan", "."]);

    assert!(output.status.success(), "scan root should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("root scan"),
        "missing root summary:\n{stdout}"
    );
    assert!(
        stdout.contains("use-bar"),
        "missing discovered facade:\n{stdout}"
    );
}

#[test]
fn scan_catalog_lists_entries() {
    let bin = CliBinary::rustuse();

    let output = run_raw(&bin, &["scan", ".", "--kind", "catalog"]);

    assert!(output.status.success(), "scan catalog should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("catalog scan"),
        "missing catalog summary:\n{stdout}"
    );
    assert!(
        stdout.contains("use-geometry"),
        "missing catalog entry:\n{stdout}"
    );
}
