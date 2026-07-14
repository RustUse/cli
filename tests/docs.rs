//! `docs` subcommand surface.

mod common;

use common::{CliBinary, run_raw, run_success};

#[test]
fn docs_without_name_returns_root_url() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["docs"]);
    assert!(
        stdout.contains("https://rustuse.org/"),
        "missing root docs URL:\n{stdout}"
    );
}

#[test]
fn docs_with_name_keeps_named_surface() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["docs", "use-geometry"]);

    assert!(
        stdout.contains("https://rustuse.org/use-geometry/"),
        "docs output missing named URL:\n{stdout}"
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
