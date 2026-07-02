//! `list` subcommand surface.

mod common;

use common::{CliBinary, run_success};

#[test]
fn list_defaults_to_non_all_mode() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["list"]);

    assert!(
        stdout.contains("rustuse list: placeholder"),
        "list output missing placeholder marker:\n{stdout}"
    );
    assert!(
        stdout.contains("all=false"),
        "list output missing default all=false detail:\n{stdout}"
    );
}

#[test]
fn list_all_sets_all_true() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["list", "--all"]);

    assert!(
        stdout.contains("all=true"),
        "list output missing all=true detail:\n{stdout}"
    );
}
