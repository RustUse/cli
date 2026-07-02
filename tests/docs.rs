//! `docs` subcommand surface.

mod common;

use common::{CliBinary, run_raw, run_success};

#[test]
fn docs_without_name_fails_with_required_name_error() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["docs"]);

    assert!(
        !output.status.success(),
        "docs without name should fail with clap usage error"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required arguments") && stderr.contains("<NAME>"),
        "docs error should mention missing name argument:\n{stderr}"
    );
}

#[test]
fn docs_with_name_keeps_named_surface() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["docs", "use-geometry"]);

    assert!(
        stdout.contains("name=use-geometry"),
        "docs output missing named detail:\n{stdout}"
    );
}

#[test]
fn docs_json_mode_returns_machine_record() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--json", "docs", "use-geometry"]);

    assert!(
        output.status.success(),
        "docs in json mode should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"command\":\"docs\""),
        "json output missing command field:\n{stdout}"
    );
}
