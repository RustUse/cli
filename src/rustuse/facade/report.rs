//! Markdown report generation for one RustUse facade repository.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use super::discover::{FacadeInfo, discover_facade};
use crate::output::Output;
use crate::rustuse::adapter::github::workflows::{GitHubWorkflowReport, inspect_github_workflows};
use crate::rustuse::adapter::gitlab::{GitLabReport, inspect_gitlab};
use crate::rustuse::facade::documentation::{
    DocumentationSurfaceReport, inspect_documentation_surface,
};
use crate::rustuse::facade::inspect::{FacadeRepositoryReport, inspect_facade_repository};
use crate::rustuse::facade::manifest::{FacadeManifestReport, analyze_facade_repository_manifests};
use crate::rustuse::facade::nonstandard::{
    NonStandardPathKind, NonStandardPathReport, NonStandardPathRule, inspect_non_standard_paths,
};
use crate::rustuse::facade::release::{ReleaseSurfaceReport, inspect_release_surface};
use crate::rustuse::report::destination::ReportDestination;
use crate::rustuse::report::markdown::{
    emit_markdown_to_stdout, markdown_path, resolve_report_path, write_markdown_report,
    write_presence_table, yes_no,
};
use crate::rustuse::utils::artifacts::{GeneratedArtifactReport, inspect_generated_artifacts};
use crate::rustuse::utils::tooling::{ToolingSurfaceReport, inspect_tooling_surface};

const FACADE_NON_STANDARD_PATHS: &[NonStandardPathRule] = &[NonStandardPathRule {
    path: "docs/",
    kind: NonStandardPathKind::Directory,
    recommendation: "Move facade documentation to the central docs repository.",
}];

#[allow(clippy::too_many_arguments)]
fn build_report(
    facade: &FacadeInfo,
    manifest_report: &FacadeManifestReport,
    repository_report: &FacadeRepositoryReport,
    github_workflow_report: &GitHubWorkflowReport,
    gitlab_report: &GitLabReport,
    release_report: &ReleaseSurfaceReport,
    documentation_report: &DocumentationSurfaceReport,
    // development_report: &DevelopmentSurfaceReport,
    tooling_report: &ToolingSurfaceReport,
    non_standard_path_report: &NonStandardPathReport,
    artifact_report: &GeneratedArtifactReport,
) -> String {
    let mut markdown = String::new();

    markdown.push_str("# RustUse Facade Report\n\n");
    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!("- Root: `{}`\n", facade.root.display()));
    markdown.push_str(&format!("- Facade: `{}`\n", facade.name));
    markdown.push_str(&format!("- Git: `{}`\n", yes_no(facade.has_git())));
    markdown.push_str(&format!(
        "- Cargo.toml: `{}`\n",
        yes_no(facade.has_manifest())
    ));
    markdown.push_str(&format!(
        "- crates/: `{}`\n",
        yes_no(facade.has_crates_dir())
    ));
    markdown.push_str(&format!(
        "- Child crate manifests: `{}`\n",
        facade.crate_count()
    ));
    markdown.push_str(&format!(
        "- Status: **{}**\n\n",
        overall_status(
            facade,
            manifest_report,
            repository_report,
            github_workflow_report,
            release_report,
            documentation_report,
        )
    ));

    write_contents(&mut markdown);
    write_action_plan(
        &mut markdown,
        facade,
        manifest_report,
        repository_report,
        github_workflow_report,
        release_report,
        // documentation_report,
    );
    write_facade_shape(&mut markdown, facade);
    write_repository_surface(&mut markdown, repository_report);
    write_manifest_health(&mut markdown, facade, manifest_report);
    write_child_crates(&mut markdown, facade, manifest_report);
    write_crate_documentation_consistency(&mut markdown, facade);
    write_standard_file_consistency(&mut markdown, repository_report);
    write_non_standard_paths(&mut markdown, non_standard_path_report);
    write_tooling_configuration(&mut markdown, tooling_report);
    // write_development_environment(&mut markdown, development_report);
    write_ci_cd_surface(
        &mut markdown,
        github_workflow_report,
        gitlab_report,
        release_report,
    );
    write_documentation_surface(&mut markdown, documentation_report);
    write_release_surface(&mut markdown, release_report);
    write_generated_artifacts(&mut markdown, artifact_report);
    write_notes(&mut markdown);

    markdown
}

fn write_contents(markdown: &mut String) {
    markdown.push_str("## Contents\n\n");
    markdown.push_str("- [Action Plan](#action-plan)\n");
    markdown.push_str("- [Facade Shape](#facade-shape)\n");
    markdown.push_str("- [Repository Surface](#repository-surface)\n");
    markdown.push_str("- [Cargo Manifest Health](#cargo-manifest-health)\n");
    markdown.push_str("- [Child Crates](#child-crates)\n");
    markdown.push_str("- [Crate Documentation Consistency](#crate-documentation-consistency)\n");
    markdown.push_str("- [Standard File Consistency](#standard-file-consistency)\n");
    markdown.push_str("- [Non-standard Paths](#non-standard-paths)\n");
    markdown.push_str("- [Tooling Configuration](#tooling-configuration)\n");
    markdown.push_str("- [Development Environment](#development-environment)\n");
    markdown.push_str("- [CI/CD Surface](#cicd-surface)\n");
    markdown.push_str("- [Documentation Surface](#documentation-surface)\n");
    markdown.push_str("- [Release Surface](#release-surface)\n");
    markdown.push_str("- [Generated / Local Artifacts](#generated--local-artifacts)\n");
    markdown.push_str("- [Notes](#notes)\n\n");
}

fn write_action_plan(
    markdown: &mut String,
    facade: &FacadeInfo,
    manifest_report: &FacadeManifestReport,
    repository_report: &FacadeRepositoryReport,
    github_workflow_report: &GitHubWorkflowReport,
    release_report: &ReleaseSurfaceReport,
    // documentation_report: &DocumentationSurfaceReport,
) {
    markdown.push_str("## Action Plan\n\n");

    let missing_files = repository_report.missing_required_files();
    let missing_directories = repository_report.missing_required_directories();
    let crate_documentation_warnings = crate_documentation_warning_count(facade);
    let missing_ci_cd_paths = github_workflow_report.missing_required_paths();
    let has_release_surface_warnings = release_report.status() != "ok";
    let has_documentation_surface_warnings = false; // documentation_report.status() != "ok";

    let has_action = !facade.has_git()
        || !facade.has_manifest()
        || !facade.has_crates_dir()
        || facade.crate_count() == 0
        || manifest_report.error_count() > 0
        || manifest_report.warning_count() > 0
        || crate_documentation_warnings > 0
        || !missing_ci_cd_paths.is_empty()
        || has_release_surface_warnings
        || !missing_files.is_empty()
        || !missing_directories.is_empty();

    if !has_action {
        markdown.push_str("- No required action.\n");

        let missing_optional_directories = missing_optional_directories(repository_report);

        if !missing_optional_directories.is_empty() {
            markdown.push_str("- Optional gaps:\n");

            for path in missing_optional_directories {
                markdown.push_str(&format!("  - `{path}`\n"));
            }
        }

        markdown.push('\n');
        return;
    }

    if !missing_ci_cd_paths.is_empty() {
        markdown.push_str(&format!(
            "- Restore {} missing required GitHub CI/CD file(s).\n",
            missing_ci_cd_paths.len()
        ));

        for path in missing_ci_cd_paths.iter().take(12) {
            markdown.push_str(&format!("  - Missing `{path}`\n"));
        }
    }

    if has_release_surface_warnings {
        markdown.push_str("- Restore missing release surface files.\n");
    }

    if has_documentation_surface_warnings {
        markdown.push_str("- Address documentation surface warnings.\n");
    }

    if crate_documentation_warnings > 0 {
        markdown.push_str(&format!(
        "- Fix crate documentation consistency warnings. There are {crate_documentation_warnings} crate documentation warning(s).\n",
    ));
    }

    if manifest_report.error_count() > 0 {
        markdown.push_str(&format!(
            "- Fix manifest errors first. There are {} manifest error(s).\n",
            manifest_report.error_count()
        ));
    }

    if manifest_report.warning_count() > 0 {
        markdown.push_str(&format!(
            "- Clean up manifest warnings. There are {} manifest warning(s).\n",
            manifest_report.warning_count()
        ));
    }

    if !missing_files.is_empty() {
        markdown.push_str(&format!(
            "- Restore {} missing required standard file(s).\n",
            missing_files.len()
        ));

        for check in missing_files.iter().take(8) {
            markdown.push_str(&format!("  - Missing `{}` ({})\n", check.path, check.label));
        }
    }

    if !missing_directories.is_empty() {
        markdown.push_str(&format!(
            "- Restore {} missing required standard directory/directories.\n",
            missing_directories.len()
        ));

        for check in missing_directories.iter().take(8) {
            markdown.push_str(&format!("  - Missing `{}` ({})\n", check.path, check.label));
        }
    }

    if !facade.has_git() {
        markdown.push_str("- Initialize or restore the facade `.git` repository.\n");
    }

    if !facade.has_manifest() {
        markdown.push_str("- Add the facade root `Cargo.toml`.\n");
    }

    if !facade.has_crates_dir() {
        markdown.push_str("- Add the facade `crates/` directory.\n");
    }

    if facade.has_crates_dir() && facade.crate_count() == 0 {
        markdown.push_str("- Add child crates under `crates/`, each with its own `Cargo.toml`.\n");
    }

    markdown.push('\n');
}

fn write_facade_shape(markdown: &mut String, facade: &FacadeInfo) {
    markdown.push_str("## Facade Shape\n\n");
    markdown.push_str("| Check | Status |\n");
    markdown.push_str("|---|---:|\n");
    markdown.push_str(&format!("| `.git` | {} |\n", yes_no(facade.has_git())));
    markdown.push_str(&format!(
        "| `Cargo.toml` | {} |\n",
        yes_no(facade.has_manifest())
    ));
    markdown.push_str(&format!(
        "| `crates/` | {} |\n",
        yes_no(facade.has_crates_dir())
    ));
    markdown.push_str(&format!(
        "| Child crate manifests | {} |\n",
        facade.crate_count()
    ));
    markdown.push('\n');
}

fn write_repository_surface(markdown: &mut String, report: &FacadeRepositoryReport) {
    markdown.push_str("## Repository Surface\n\n");

    let required_files = report.files.iter().filter(|check| check.required).count();
    let present_required_files = report
        .files
        .iter()
        .filter(|check| check.required && check.present)
        .count();

    let optional_files = report.files.iter().filter(|check| !check.required).count();
    let present_optional_files = report
        .files
        .iter()
        .filter(|check| !check.required && check.present)
        .count();

    let required_directories = report
        .directories
        .iter()
        .filter(|check| check.required)
        .count();
    let present_required_directories = report
        .directories
        .iter()
        .filter(|check| check.required && check.present)
        .count();

    let optional_directories = report
        .directories
        .iter()
        .filter(|check| !check.required)
        .count();
    let present_optional_directories = report
        .directories
        .iter()
        .filter(|check| !check.required && check.present)
        .count();

    markdown.push_str(&format!("- Status: **{}**\n", report.status().as_str()));
    markdown.push_str(&format!(
        "- Required files: `{present_required_files}/{required_files}`\n"
    ));
    markdown.push_str(&format!(
        "- Optional files: `{present_optional_files}/{optional_files}`\n"
    ));
    markdown.push_str(&format!(
        "- Required directories: `{present_required_directories}/{required_directories}`\n"
    ));
    markdown.push_str(&format!(
        "- Optional directories: `{present_optional_directories}/{optional_directories}`\n\n"
    ));
}

fn write_manifest_health(
    markdown: &mut String,
    facade: &FacadeInfo,
    report: &FacadeManifestReport,
) {
    markdown.push_str("## Cargo Manifest Health\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", report.status()));
    markdown.push_str(&format!(
        "- Manifests inspected: `{}`\n",
        report.manifest_count()
    ));
    markdown.push_str(&format!("- Issues: `{}`\n", report.issue_count()));
    markdown.push_str(&format!("- Errors: `{}`\n", report.error_count()));
    markdown.push_str(&format!("- Warnings: `{}`\n", report.warning_count()));
    markdown.push_str(&format!(
        "- Invalid crates.io category slugs: `{}`\n\n",
        report.invalid_category_count()
    ));

    write_manifest_issue_summary(markdown, report);
    write_manifest_inventory(markdown, report);

    if report.issue_count() == 0 {
        return;
    }

    markdown.push_str("### Manifest Issues\n\n");

    for manifest in &report.manifests {
        if manifest.issues.is_empty() {
            continue;
        }

        let display_path = markdown_path(&manifest.path);

        markdown.push_str(&format!("#### `{display_path}`\n\n"));
        markdown.push_str(&format!("- Kind: `{}`\n", manifest.kind.as_str()));
        markdown.push_str(&format!("- Status: **{}**\n", manifest.status()));

        if let Some(package_name) = &manifest.package_name {
            markdown.push_str(&format!("- Package: `{package_name}`\n"));
        }

        markdown.push_str("- Issues:\n");

        for issue in &manifest.issues {
            markdown.push_str(&format!(
                "  - **{}** `{}`: {}\n",
                issue.severity.as_str(),
                issue.code,
                issue.message
            ));
        }

        markdown.push('\n');
    }

    if facade.crate_count() == 0 {
        markdown.push_str("- No child crate manifests were discovered.\n\n");
    }
}

fn write_manifest_issue_summary(markdown: &mut String, report: &FacadeManifestReport) {
    let mut rows = report
        .manifests
        .iter()
        .flat_map(|manifest| manifest.issues.iter())
        .fold(
            BTreeMap::<(&'static str, &'static str), usize>::new(),
            |mut summary, issue| {
                let key = (issue.severity.as_str(), issue.code);
                *summary.entry(key).or_default() += 1;
                summary
            },
        )
        .into_iter()
        .map(|((severity, code), count)| (severity, code, count))
        .collect::<Vec<_>>();

    if rows.is_empty() {
        return;
    }

    rows.sort_by(|left, right| {
        right
            .2
            .cmp(&left.2)
            .then_with(|| left.0.cmp(right.0))
            .then_with(|| left.1.cmp(right.1))
    });

    markdown.push_str("### Manifest Issue Summary\n\n");
    markdown.push_str("| Severity | Code | Count |\n");
    markdown.push_str("|---|---|---:|\n");

    for (severity, code, count) in rows {
        markdown.push_str(&format!("| `{severity}` | `{code}` | {count} |\n"));
    }

    markdown.push('\n');
}

fn write_manifest_inventory(markdown: &mut String, report: &FacadeManifestReport) {
    markdown.push_str("### Manifest Inventory\n\n");
    markdown.push_str("| Kind | Package | Status | Issues | Manifest |\n");
    markdown.push_str("|---|---|---:|---:|---|\n");

    for manifest in &report.manifests {
        let package = manifest.package_name.as_deref().unwrap_or("<none>");
        let display_path = markdown_path(&manifest.path);

        markdown.push_str(&format!(
            "| `{}` | `{}` | {} | {} | `{}` |\n",
            manifest.kind.as_str(),
            package,
            manifest.status(),
            manifest.issue_count(),
            display_path
        ));
    }

    markdown.push('\n');
}

fn write_child_crates(
    markdown: &mut String,
    facade: &FacadeInfo,
    manifest_report: &FacadeManifestReport,
) {
    markdown.push_str("## Child Crates\n\n");

    if facade.crate_manifest_paths.is_empty() {
        markdown.push_str("- No child crate manifests found.\n\n");
        return;
    }

    markdown
        .push_str("| Kind | Crate | Manifest | README | lib.rs | prelude.rs | Status | Issues |\n");
    markdown.push_str("|---|---|---|---:|---:|---:|---:|---:|\n");

    for manifest_path in &facade.crate_manifest_paths {
        let crate_dir = manifest_path.parent().unwrap_or(&facade.root);

        let crate_name = crate_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");

        let kind = if crate_name == facade.name {
            "facade"
        } else {
            "child"
        };

        let display_path = manifest_path
            .strip_prefix(&facade.root)
            .unwrap_or(manifest_path);

        let display_path = markdown_path(display_path);

        let manifest_report_item = manifest_report
            .manifests
            .iter()
            .find(|manifest| markdown_path(&manifest.path) == display_path);

        let manifest_status = manifest_report_item
            .map(|manifest| manifest.status())
            .unwrap_or("unknown");

        let issue_count = manifest_report_item
            .map(|manifest| manifest.issue_count())
            .unwrap_or(0);

        markdown.push_str(&format!(
            "| `{kind}` | `{crate_name}` | `{display_path}` | {} | {} | {} | {} | {} |\n",
            yes_no(crate_dir.join("README.md").is_file()),
            yes_no(crate_dir.join("src/lib.rs").is_file()),
            yes_no(crate_dir.join("src/prelude.rs").is_file()),
            manifest_status,
            issue_count
        ));
    }

    markdown.push('\n');
}

fn write_standard_file_consistency(markdown: &mut String, report: &FacadeRepositoryReport) {
    markdown.push_str("## Standard File Consistency\n\n");

    markdown.push_str("| Status | Required | Present | Path | Purpose |\n");
    markdown.push_str("|---|---:|---:|---|---|\n");

    for check in &report.files {
        markdown.push_str(&format!(
            "| {} | {} | {} | `{}` | {} |\n",
            check.status.as_str(),
            yes_no(check.required),
            yes_no(check.present),
            check.path,
            check.label
        ));
    }

    markdown.push('\n');
}

fn write_tooling_configuration(markdown: &mut String, tooling_report: &ToolingSurfaceReport) {
    markdown.push_str("## Tooling Configuration\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", tooling_report.status()));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        tooling_report.present_count(),
        tooling_report.total_count()
    ));

    write_presence_table(markdown, &tooling_report.surface);
}

fn write_ci_cd_surface(
    markdown: &mut String,
    github_report: &GitHubWorkflowReport,
    gitlab_report: &GitLabReport,
    release_report: &ReleaseSurfaceReport,
) {
    markdown.push_str("## CI/CD Surface\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", github_report.status()));
    markdown.push_str(&format!(
        "- Required GitHub CI/CD surface: `{}/{}`\n",
        github_report.present_required_count(),
        github_report.required_count()
    ));
    markdown.push_str(&format!(
        "- Required GitHub workflows: `{}/{}`\n",
        github_report.present_workflow_count(),
        github_report.workflow_count()
    ));
    markdown.push_str(&format!(
        "- GitLab CI surface: `{}/{}`\n",
        gitlab_report.present_count(),
        gitlab_report.total_count()
    ));
    markdown.push_str(&format!(
        "- Release CI/CD surface: `{}/{}`\n\n",
        release_report.ci_present_count(),
        release_report.ci_total_count()
    ));

    let missing = github_report.missing_required_paths();

    if !missing.is_empty() {
        markdown.push_str("- Missing required CI/CD files:\n");

        for path in &missing {
            markdown.push_str(&format!("  - `{path}`\n"));
        }

        markdown.push('\n');
    }

    markdown.push_str("### GitHub CI/CD Surface\n\n");
    markdown.push_str("| Required | Surface | Present |\n");
    markdown.push_str("|---:|---|---:|\n");

    for check in &github_report.required_surface {
        markdown.push_str(&format!(
            "| yes | `{}` | {} |\n",
            check.path,
            yes_no(check.present)
        ));
    }

    markdown.push('\n');

    markdown.push_str("### Required GitHub Workflows\n\n");
    markdown.push_str("| Workflow | Present |\n");
    markdown.push_str("|---|---:|\n");

    for check in &github_report.required_workflows {
        markdown.push_str(&format!(
            "| `{}` | {} |\n",
            check.path,
            yes_no(check.present)
        ));
    }

    markdown.push('\n');

    markdown.push_str("### GitLab CI Surface\n\n");
    write_presence_table(markdown, &gitlab_report.surface);

    markdown.push_str("### Release CI/CD Surface\n\n");
    write_presence_table(markdown, &release_report.ci_surface);
}

fn write_documentation_surface(
    markdown: &mut String,
    documentation_report: &DocumentationSurfaceReport,
) {
    markdown.push_str("## Documentation Surface\n\n");

    markdown.push_str(&format!(
        "- Status: **{}**\n",
        documentation_report.status()
    ));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        documentation_report.present_count(),
        documentation_report.total_count()
    ));

    write_presence_table(markdown, &documentation_report.surface);
}

fn write_release_surface(markdown: &mut String, release_report: &ReleaseSurfaceReport) {
    markdown.push_str("## Release Surface\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", release_report.status()));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        release_report.present_count(),
        release_report.total_count()
    ));

    write_presence_table(markdown, &release_report.surface);
}

fn write_generated_artifacts(markdown: &mut String, artifact_report: &GeneratedArtifactReport) {
    markdown.push_str("## Generated / Local Artifacts\n\n");

    if artifact_report.is_empty() {
        markdown.push_str("- No generated/local artifacts detected.\n\n");
        return;
    }

    markdown.push_str("| Path | Meaning |\n");
    markdown.push_str("|---|---|\n");

    for artifact in &artifact_report.artifacts {
        markdown.push_str(&format!(
            "| `{}` | {} |\n",
            markdown_path(&artifact.path),
            artifact.label
        ));
    }

    markdown.push('\n');
}

fn write_notes(markdown: &mut String) {
    markdown.push_str("## Notes\n\n");
    markdown.push_str("- This report is generated from the local filesystem.\n");
    markdown.push_str(
        "- A facade repository is expected to contain `.git`, `Cargo.toml`, and `crates/`.\n",
    );
    markdown.push_str(
        "- Child crates are direct directories under `crates/` with their own `Cargo.toml`.\n",
    );
    markdown.push_str(
        "- Generated and local artifacts are reported separately from required standard files.\n",
    );
}

fn overall_status(
    facade: &FacadeInfo,
    manifest_report: &FacadeManifestReport,
    repository_report: &FacadeRepositoryReport,
    github_workflow_report: &GitHubWorkflowReport,
    release_report: &ReleaseSurfaceReport,
    documentation_report: &DocumentationSurfaceReport,
) -> &'static str {
    if manifest_report.error_count() > 0 {
        return "error";
    }

    if manifest_report.warning_count() > 0 {
        return "warning";
    }

    if crate_documentation_warning_count(facade) > 0 {
        return "warning";
    }

    if github_workflow_report.status() != "ok" {
        return "warning";
    }

    if release_report.status() != "ok" {
        return "warning";
    }

    if documentation_report.status() != "ok" {
        return "warning";
    }

    if repository_report.status().as_str() != "ok" {
        return repository_report.status().as_str();
    }

    facade.status()
}

fn missing_optional_directories(report: &FacadeRepositoryReport) -> Vec<&str> {
    report
        .directories
        .iter()
        .filter(|check| !check.required && !check.present)
        .map(|check| check.path)
        .collect()
}

fn write_non_standard_paths(
    markdown: &mut String,
    non_standard_path_report: &NonStandardPathReport,
) {
    markdown.push_str("## Non-standard Paths\n\n");

    markdown.push_str(&format!(
        "- Status: **{}**\n",
        non_standard_path_report.status()
    ));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        non_standard_path_report.present_count(),
        non_standard_path_report.total_count()
    ));

    let present_checks = non_standard_path_report.present_checks();

    if present_checks.is_empty() {
        markdown.push_str("- No non-standard paths detected.\n\n");
        return;
    }

    markdown.push_str("| Path | Recommendation |\n");
    markdown.push_str("|---|---|\n");

    for check in present_checks {
        markdown.push_str(&format!(
            "| `{}` | {} |\n",
            check.path, check.recommendation
        ));
    }

    markdown.push('\n');
}

fn write_crate_documentation_consistency(markdown: &mut String, facade: &FacadeInfo) {
    markdown.push_str("## Crate Documentation Consistency\n\n");

    if facade.crate_manifest_paths.is_empty() {
        markdown.push_str("- No child crate manifests found.\n\n");
        return;
    }

    markdown.push_str("| Status | Kind | Crate | README | lib.rs | prelude.rs | Notes |\n");
    markdown.push_str("|---|---|---|---:|---:|---:|---|\n");

    for manifest_path in &facade.crate_manifest_paths {
        let crate_dir = manifest_path.parent().unwrap_or(&facade.root);

        let crate_name = crate_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");

        let is_facade_package = crate_name == facade.name;

        let readme_present = crate_dir.join("README.md").is_file();
        let lib_present = crate_dir.join("src/lib.rs").is_file();
        let prelude_present = crate_dir.join("src/prelude.rs").is_file();

        let status = if readme_present && lib_present && (!is_facade_package || prelude_present) {
            "ok"
        } else {
            "warning"
        };

        let kind = if is_facade_package { "facade" } else { "child" };

        let notes = crate_documentation_notes(
            is_facade_package,
            readme_present,
            lib_present,
            prelude_present,
        );

        markdown.push_str(&format!(
            "| {status} | `{kind}` | `{crate_name}` | {} | {} | {} | {} |\n",
            yes_no(readme_present),
            yes_no(lib_present),
            yes_no(prelude_present),
            notes
        ));
    }

    markdown.push('\n');
}

fn crate_documentation_notes(
    is_facade_package: bool,
    readme_present: bool,
    lib_present: bool,
    prelude_present: bool,
) -> String {
    let mut notes = Vec::new();

    if !readme_present {
        notes.push("missing README.md");
    }

    if !lib_present {
        notes.push("missing src/lib.rs");
    }

    if is_facade_package && !prelude_present {
        notes.push("missing facade src/prelude.rs");
    }

    if notes.is_empty() {
        "ok".to_string()
    } else {
        notes.join("; ")
    }
}

fn crate_documentation_warning_count(facade: &FacadeInfo) -> usize {
    facade
        .crate_manifest_paths
        .iter()
        .filter(|manifest_path| {
            let crate_dir = manifest_path.parent().unwrap_or(&facade.root);

            let crate_name = crate_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<unknown>");

            let is_facade_package = crate_name == facade.name;

            let readme_present = crate_dir.join("README.md").is_file();
            let lib_present = crate_dir.join("src/lib.rs").is_file();
            let prelude_present = crate_dir.join("src/prelude.rs").is_file();

            !readme_present || !lib_present || (is_facade_package && !prelude_present)
        })
        .count()
}

pub(crate) fn generate_markdown_report(
    root: &Path,
    output: Output,
    destination: ReportDestination,
) -> Result<()> {
    let facade = discover_facade(root)?;
    let manifest_report = analyze_facade_repository_manifests(&facade.root, &facade.name)?;
    let repository_report = inspect_facade_repository(&facade);
    let github_workflow_report = inspect_github_workflows(&facade.root);
    let gitlab_report = inspect_gitlab(&facade.root);
    let release_report = inspect_release_surface(&facade.root);
    let documentation_report = inspect_documentation_surface(&facade.root);
    // let development_report = inspect_development_surface(&facade.root);
    let tooling_report = inspect_tooling_surface(&facade.root);
    let non_standard_path_report =
        inspect_non_standard_paths(&facade.root, FACADE_NON_STANDARD_PATHS);
    let artifact_report = inspect_generated_artifacts(&facade.root);

    let report = build_report(
        &facade,
        &manifest_report,
        &repository_report,
        &github_workflow_report,
        &gitlab_report,
        &release_report,
        &documentation_report,
        // &development_report,
        &tooling_report,
        &non_standard_path_report,
        &artifact_report,
    );

    match destination {
        ReportDestination::Stdout => {
            emit_markdown_to_stdout(&report);
        },
        ReportDestination::File(path) => {
            let output_path = resolve_report_path(&facade.root, path);

            write_markdown_report(&output_path, &report)?;

            if output.is_json() {
                output.record(
                    "facade report",
                    "ok",
                    &format!("wrote {}", output_path.display()),
                );
            } else {
                output.line(format!(
                    "RustUse facade report - root: {}",
                    facade.root.display()
                ));
                output.line(format!("wrote: {}", output_path.display()));
            }
        },
    }

    Ok(())
}
