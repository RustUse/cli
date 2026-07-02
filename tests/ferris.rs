//! `ferris` command behavior.

mod common;

use common::{CliBinary, run_success};

#[test]
fn ferris_prints_greeting() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["ferris"]);

    assert!(
        stdout.contains("Hello from Ferris"),
        "ferris output missing greeting:\n{stdout}"
    );
}
