//! Catalog-based `search` behavior (must work outside a RustUse root).

mod common;

use common::{CliBinary, TempProject, run_raw, run_raw_in, run_success};

#[test]
fn search_geometry_reports_single_entry() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["search", "geometry"]);

    assert!(
        stdout.contains("use-geometry"),
        "missing use-geometry:\n{stdout}"
    );
    assert!(
        stdout.contains("Found 1 RustUse entry"),
        "missing count:\n{stdout}"
    );
}

#[test]
fn search_works_outside_a_rustuse_root() {
    // An empty temp dir has no `use-*` repositories; catalog search must still work.
    let project = TempProject::new();
    let bin = CliBinary::rustuse();
    let output = run_raw_in(&bin, project.path(), &["search", "geometry"]);

    assert!(
        output.status.success(),
        "search should not require a RustUse root"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("use-geometry"),
        "missing use-geometry:\n{stdout}"
    );
}

#[test]
fn search_without_matches_is_reported() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["search", "definitely-not-a-crate"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No RustUse entries matched"),
        "missing empty message:\n{stdout}"
    );
}
