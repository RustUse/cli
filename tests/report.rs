//! `dev report` against temporary root and facade fixtures.

mod common;

use common::{CliBinary, TempProject, run_raw_in};

#[test]
fn dev_report_fleet_writes_root_report() {
    let project = TempProject::root_with_facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["dev", "report", ".", "--fleet"]);

    assert!(output.status.success(), "fleet report should succeed");
    let report_path = project.path().join("rustuse-fleet-report.md");
    let report = std::fs::read_to_string(&report_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", report_path.display()));
    assert!(
        report.contains("RustUse Fleet Report"),
        "missing fleet report heading:\n{report}"
    );
}

#[test]
fn dev_report_facade_writes_facade_report() {
    let project = TempProject::facade("bar");
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["dev", "report", "."]);

    assert!(output.status.success(), "facade report should succeed");
    let report_path = project.path().join("rustuse-report.md");
    let report = std::fs::read_to_string(&report_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", report_path.display()));
    assert!(
        report.contains("RustUse Facade Report"),
        "missing facade report heading:\n{report}"
    );
}

#[test]
fn dev_report_facade_surfaces_nested_facade_package() {
    let project = TempProject::facade("quant");
    project.write(
        "crates/use-quant/Cargo.toml",
        "[package]\nname = \"use-quant\"\nversion = \"0.1.0\"\n",
    );
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["dev", "report", "."]);
    assert!(output.status.success(), "facade report should succeed");

    let report = std::fs::read_to_string(project.path().join("rustuse-report.md"))
        .expect("failed to read facade report");
    assert_eq!(
        report.matches("nested-facade-package").count(),
        2,
        "facade report should include the code in its summary and detail sections:\n{report}"
    );
    assert!(report.contains("crates/use-quant"));
}

#[test]
fn dev_report_fleet_surfaces_nested_facade_package() {
    let project = TempProject::root_with_facade("quant");
    project.write(
        "use-quant/crates/legacy-name/Cargo.toml",
        "[package]\nname = \"use-quant\"\nversion = \"0.1.0\"\n",
    );
    let bin = CliBinary::rustuse();

    let output = run_raw_in(&bin, project.path(), &["dev", "report", ".", "--fleet"]);
    assert!(output.status.success(), "fleet report should succeed");

    let report = std::fs::read_to_string(project.path().join("rustuse-fleet-report.md"))
        .expect("failed to read fleet report");
    assert!(report.contains("nested-facade-package"));
}
