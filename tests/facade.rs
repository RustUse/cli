//! Facade maintainer workflows exposed through `dev` and `ci`.

mod common;

use common::{CliBinary, run_help, run_raw};

#[test]
fn dev_help_lists_facade_workflows() {
    let bin = CliBinary::rustuse();
    let stdout = run_help(&bin, &["dev"]);

    for subcommand in ["inspect", "report"] {
        assert!(
            stdout.contains(subcommand),
            "dev help missing `{subcommand}`:\n{stdout}"
        );
    }
}

#[test]
fn dev_without_subcommand_fails_non_interactively() {
    let bin = CliBinary::rustuse();
    let output = run_raw(&bin, &["--non-interactive", "dev"]);

    assert!(
        !output.status.success(),
        "bare `dev` should require a subcommand when non-interactive"
    );
}

#[test]
fn dev_inspect_runs_real_backend() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("demo");

    let output = common::run_raw_in(&bin, temp.path(), &["dev", "inspect", "."]);

    assert!(
        output.status.success(),
        "expected success for `dev inspect .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("RustUse facade inspect"),
        "inspect output missing summary:\n{stdout}"
    );
}

#[test]
fn ci_check_reports_nested_facade_package_once_by_package_name() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("quant");
    temp.write(
        "crates/legacy-name/Cargo.toml",
        "[package]\nname = \"use-quant\"\nversion = \"0.1.0\"\n",
    );

    let output = common::run_raw_in(&bin, temp.path(), &["ci", "check", "."]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.matches("nested-facade-package").count(),
        1,
        "nested facade diagnostic should appear once:\n{stdout}"
    );
    assert!(
        stdout.contains("legacy-name"),
        "diagnostic should include the matching child directory:\n{stdout}"
    );
}

#[test]
fn ci_check_uses_package_name_not_directory_name() {
    let bin = CliBinary::rustuse();

    let matching_directory = common::TempProject::facade("quant");
    matching_directory.write(
        "crates/use-quant/Cargo.toml",
        "[package]\nname = \"different-package\"\nversion = \"0.1.0\"\n",
    );
    let output = common::run_raw_in(&bin, matching_directory.path(), &["ci", "check", "."]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("nested-facade-package"),
        "directory name alone must not trigger the rule:\n{stdout}"
    );

    let matching_package = common::TempProject::facade("quant");
    matching_package.write(
        "crates/other-name/Cargo.toml",
        "[package]\nname = \"use-quant\"\nversion = \"0.1.0\"\n",
    );
    let output = common::run_raw_in(&bin, matching_package.path(), &["ci", "check", "."]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("nested-facade-package"),
        "package name must trigger the rule even with a different directory:\n{stdout}"
    );
}

#[test]
fn ci_check_reports_docs_rs_all_features_policy() {
    let bin = CliBinary::rustuse();

    let missing = common::TempProject::facade("docs-missing");
    let output = common::run_raw_in(&bin, missing.path(), &["ci", "check", "."]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("missing-docs-rs-all-features"),
        "missing docs.rs all-features policy should be reported:\n{stdout}"
    );

    let invalid = common::TempProject::facade("docs-invalid");
    invalid.write(
        "Cargo.toml",
        "[package]\nname = \"use-docs-invalid\"\nversion = \"0.1.0\"\n\n[package.metadata.docs.rs]\nall-features = false\n",
    );
    let output = common::run_raw_in(&bin, invalid.path(), &["ci", "check", "."]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("invalid-docs-rs-all-features"),
        "invalid docs.rs all-features policy should be reported:\n{stdout}"
    );
}

#[test]
fn ci_check_reports_child_package_directory_mismatch() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("directory");
    temp.write(
        "crates/actual-directory/Cargo.toml",
        "[package]\nname = \"use-different-name\"\nversion = \"0.1.0\"\n",
    );

    let output = common::run_raw_in(&bin, temp.path(), &["ci", "check", "."]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("package-name-directory-mismatch"),
        "child package directory mismatch should be reported:\n{stdout}"
    );
}

#[test]
fn ci_check_deny_warnings_fails_on_incomplete_repo() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::new();

    let output = common::run_raw_in(&bin, temp.path(), &["ci", "check", "--deny-warnings", "."]);

    assert!(
        !output.status.success(),
        "expected `ci check --deny-warnings` to fail on incomplete repo"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deny-warnings") || stderr.contains("warning"),
        "stderr missing deny-warnings failure context:\n{stderr}"
    );
}

#[test]
fn dev_report_writes_default_markdown_report() {
    let bin = CliBinary::rustuse();
    let temp = common::TempProject::facade("report");

    let output = common::run_raw_in(&bin, temp.path(), &["dev", "report", "."]);

    assert!(
        output.status.success(),
        "expected success for `dev report .`, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_path = temp.path().join("rustuse-report.md");
    assert!(
        report_path.is_file(),
        "expected report file at {}",
        report_path.display()
    );
}
