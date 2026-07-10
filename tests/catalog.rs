//! catalog subcommand surface.

mod common;

use common::{CliBinary, TempProject, run_success};

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
fn catalog_discover_lists_local_use_packages() {
    let project = TempProject::new();
    project.write(
        "use-local/Cargo.toml",
        "[package]\nname = \"use-local\"\nversion = \"0.1.0\"\n",
    );
    let bin = CliBinary::rustuse();
    let stdout = run_success(
        &bin,
        &["catalog", "discover", project.path().to_str().unwrap()],
    );

    assert!(
        stdout.contains("use-local"),
        "catalog discover output missing local package:\n{stdout}"
    );
}
