//! `cargo-rustuse` dispatch behavior.

mod common;

use common::{CliBinary, run_raw, run_success};

#[test]
fn cargo_rustuse_search_works() {
    let bin = CliBinary::cargo_rustuse();
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
fn cargo_dispatched_search_works() {
    // `cargo rustuse search geometry` invokes `cargo-rustuse rustuse search geometry`.
    let bin = CliBinary::cargo_rustuse();
    let stdout = run_success(&bin, &["rustuse", "search", "geometry"]);

    assert!(
        stdout.contains("use-geometry"),
        "missing use-geometry:\n{stdout}"
    );
}

#[test]
fn cargo_dispatched_help_works() {
    let bin = CliBinary::cargo_rustuse();
    let output = run_raw(&bin, &["rustuse", "--help"]);

    assert!(
        output.status.success(),
        "cargo-dispatched --help should succeed"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage:"),
        "help output missing usage:\n{stdout}"
    );
}
