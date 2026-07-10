//! `list` subcommand surface.

mod common;

use common::{CliBinary, TempProject, run_raw_in, run_success};

#[test]
fn list_defaults_to_non_all_mode() {
    let project = TempProject::new();
    project.write(
        "rustuse.toml",
        "version = 1\n[project]\nname = \"demo\"\nkind = \"project\"\ndefault_adoption = \"cargo\"\ncopy_root = \"src\"\ntest_root = \"tests\"\nlicense = \"MIT OR Apache-2.0\"\n",
    );
    let bin = CliBinary::rustuse();
    let stdout =
        String::from_utf8_lossy(&run_raw_in(&bin, project.path(), &["list"]).stdout).into_owned();

    assert!(
        stdout.contains("No tracked RustUse primitives."),
        "unexpected list output:\n{stdout}"
    );
}

#[test]
fn list_all_sets_all_true() {
    let bin = CliBinary::rustuse();
    let stdout = run_success(&bin, &["list", "--all"]);

    assert!(
        stdout.contains("use-geometry"),
        "list --all missing catalog entry:\n{stdout}"
    );
}
