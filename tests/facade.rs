//! `facade` subcommand surface.

mod common;

use common::{CliBinary, run_help, run_raw};

#[test]
fn facade_help_lists_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["facade"]);

    for subcommand in ["inspect", "scan", "check", "report"] {
        assert!(
            stdout.contains(subcommand),
            "facade help missing `{subcommand}`:\n{stdout}"
        );
    }
}

#[test]
fn facade_without_subcommand_fails() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["facade"]);

    assert!(
        !output.status.success(),
        "bare `facade` should require a subcommand"
    );
}
