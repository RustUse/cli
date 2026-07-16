//! Integration tests for upgrading the installed `RustUse` CLI.

mod common;

use common::{CliBinary, run_raw};

#[test]
fn upgrade_dry_run_reports_cargo_install_command() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["upgrade", "--dry-run"]);

    assert!(
        output.status.success(),
        "expected `rustuse upgrade --dry-run` to succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("cargo install rustuse-cli --force --locked"),
        "stdout missing planned Cargo command:\n{stdout}"
    );
}

#[test]
fn upgrade_help_lists_dry_run_option() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["upgrade", "--help"]);

    assert!(
        output.status.success(),
        "expected `rustuse upgrade --help` to succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("--dry-run"),
        "upgrade help missing `--dry-run` option:\n{stdout}"
    );

    assert!(
        stdout.contains("Show the planned Cargo command without running it"),
        "upgrade help missing dry-run description:\n{stdout}"
    );
}
