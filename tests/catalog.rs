//! catalog subcommand surface.

mod common;

use common::{CliBinary, run_success};

#[test]
fn catalog_info_returns_entry_details() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["catalog", "info", "use-geometry"]);

    assert!(
        stdout.contains("use-geometry"),
        "catalog info output missing entry name:\n{stdout}"
    );
    assert!(
        stdout.contains("docs:"),
        "catalog info output missing docs field:\n{stdout}"
    );
}

#[test]
fn catalog_search_returns_matches() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["catalog", "search", "geometry", "--limit", "1"]);

    assert!(
        stdout.contains("Found 1 RustUse entry"),
        "catalog search output missing summary:\n{stdout}"
    );
}

#[test]
fn catalog_discover_is_explicitly_staged() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["catalog", "discover"]);

    assert!(
        stdout.contains("staged=true"),
        "catalog discover output should mark staged behavior:\n{stdout}"
    );
}
