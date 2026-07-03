//! Markdown report generation for a local RustUse development root.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};

use crate::output::Output;
use crate::rustuse::facade::discover::{FacadeEntry, discover_facades, discover_root_repos};
use crate::rustuse::facade::flags::manifest_shape_bucket;
use crate::rustuse::facade::manifest::{FacadeManifestReport, analyze_manifests};
use crate::rustuse::facade::standards::{StandardFileReport, analyze_exact_standard_files};
use crate::rustuse::report::destination::ReportDestination;
use crate::rustuse::report::markdown::{emit_markdown_to_stdout, yes_no};

/* #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportSubject {
    Facade,
    Root,
} */

const TOP_MANIFEST_OFFENDER_LIMIT: usize = 15;

fn write_report(path: &Path, report: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create `{}`", parent.display()))?;
    }

    let file =
        File::create(path).with_context(|| format!("failed to create `{}`", path.display()))?;
    let mut writer = BufWriter::new(file);

    writer
        .write_all(report.as_bytes())
        .with_context(|| format!("failed to write `{}`", path.display()))?;

    writer
        .flush()
        .with_context(|| format!("failed to flush `{}`", path.display()))?;

    Ok(())
}

fn build_report(root: &Path) -> Result<String> {
    let root_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>");

    let has_cli = root.join("cli").is_dir();
    let has_docs = root.join("docs").is_dir();
    let facades = discover_facades(root)?;
    let root_repos = discover_root_repos(root);

    let use_dir_count = facades.len();
    let git_count = facades.iter().filter(|facade| facade.has_git).count();
    let missing_git_count = use_dir_count.saturating_sub(git_count);
    let missing_git = facades
        .iter()
        .filter(|facade| !facade.has_git)
        .collect::<Vec<_>>();
    let child_crate_count: usize = facades.iter().map(|facade| facade.child_crate_count).sum();

    let missing_root_repos = root_repos
        .iter()
        .filter(|repo| !repo.present)
        .map(|repo| repo.name)
        .collect::<Vec<_>>();
    let missing_root_repo_count = missing_root_repos.len();

    let standard_file_reports = analyze_exact_standard_files(root, &facades)?;

    let has_standard_file_drift = standard_file_reports
        .iter()
        .any(|report| !report.is_consistent(use_dir_count));

    let manifest_reports = analyze_manifests(root, &facades)?;
    let manifest_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::manifest_count)
        .sum::<usize>();
    let manifest_issue_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::issue_count)
        .sum::<usize>();
    let manifest_error_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::error_count)
        .sum::<usize>();
    let manifest_warning_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::warning_count)
        .sum::<usize>();
    let invalid_category_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::invalid_category_count)
        .sum::<usize>();

    let has_warning = !has_cli
        || !has_docs
        || missing_root_repo_count > 0
        || use_dir_count == 0
        || missing_git_count > 0
        || has_standard_file_drift
        || manifest_warning_count > 0;

    let status = if manifest_error_count > 0 {
        "error"
    } else if has_warning {
        "warning"
    } else {
        "ok"
    };

    let mut markdown = String::new();

    markdown.push_str("# RustUse Development Root Report\n\n");
    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!("- Root: `{}`\n", root.display()));
    markdown.push_str(&format!("- Root name: `{root_name}`\n"));
    markdown.push_str(&format!(
        "- Standard root name: {}\n",
        if root_name == "rustuse" {
            "yes"
        } else {
            "no, expected `rustuse`"
        }
    ));

    markdown.push('\n');
    markdown.push_str("### Root Repositories\n\n");
    markdown.push_str("| Repository | Present | Git |\n");
    markdown.push_str("|---|---:|---:|\n");

    for repo in &root_repos {
        markdown.push_str(&format!(
            "| `{}` | {} | {} |\n",
            repo.name,
            yes_no(repo.present),
            yes_no(repo.has_git)
        ));
    }

    markdown.push('\n');
    markdown.push_str(&format!("- `use-*` directories: {use_dir_count}\n"));
    markdown.push_str(&format!("- Facade repos with `.git`: {git_count}\n"));
    markdown.push_str(&format!(
        "- `use-*` directories missing `.git`: {missing_git_count}\n"
    ));
    markdown.push_str(&format!("- Child crates detected: {child_crate_count}\n"));
    markdown.push_str(&format!("- Cargo manifests inspected: {manifest_count}\n"));
    markdown.push_str(&format!(
        "- Cargo manifest issues: {manifest_issue_count} ({manifest_error_count} error(s), {manifest_warning_count} warning(s))\n"
    ));
    markdown.push_str(&format!(
        "- Invalid crates.io category slugs: {invalid_category_count}\n"
    ));
    markdown.push_str(&format!("- Status: **{status}**\n\n"));

    write_contents(&mut markdown);

    write_action_plan(
        &mut markdown,
        root_name,
        &missing_root_repos,
        &missing_git,
        &standard_file_reports,
        &manifest_reports,
        use_dir_count,
    );

    write_manifest_summary(
        &mut markdown,
        &manifest_reports,
        manifest_count,
        manifest_issue_count,
        manifest_error_count,
        manifest_warning_count,
        invalid_category_count,
    );

    markdown.push_str("## Standard File Consistency\n\n");
    markdown.push_str("| File | Present | Missing | Variants | Consistent |\n");
    markdown.push_str("|---|---:|---:|---:|---:|\n");

    for report in &standard_file_reports {
        markdown.push_str(&format!(
            "| `{}` | {}/{} | {} | {} | {} |\n",
            report.file_name,
            report.present_count,
            use_dir_count,
            report.missing.len(),
            report.variants.len(),
            yes_no(report.is_consistent(use_dir_count))
        ));
    }

    markdown.push('\n');

    for report in &standard_file_reports {
        write_standard_file_report(&mut markdown, report, use_dir_count);
    }

    markdown.push_str("## Facade Inventory\n\n");
    markdown
        .push_str("| Status | Facade | Version | Git | Cargo.toml | crates/ | Child crates |\n");
    markdown.push_str("|---|---|---:|---:|---:|---:|---:|\n");

    for facade in &facades {
        markdown.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} |\n",
            facade.status(),
            facade.name,
            display_markdown_version(&facade.version),
            yes_no(facade.has_git),
            yes_no(facade.has_cargo_toml),
            yes_no(facade.has_crates_dir),
            facade.child_crate_count
        ));
    }

    if !missing_git.is_empty() {
        markdown.push_str("\n## `use-*` Directories Missing `.git`\n\n");

        for facade in missing_git {
            markdown.push_str(&format!("- `{}`\n", facade.name));
        }
    }

    markdown.push_str("\n## Notes\n\n");
    markdown.push_str("- This report is generated from the local filesystem.\n");
    markdown.push_str("- `use-*` directories are treated as facade candidates.\n");
    markdown.push_str("- A facade repo is expected to contain its own `.git` directory.\n");
    markdown.push_str(
        "- `crates/` child counts only include direct child directories with `Cargo.toml`.\n",
    );
    markdown.push_str(
        "- Manifest errors are treated as publish blockers because crates.io rejects invalid category slugs and other invalid Cargo metadata.\n",
    );
    markdown.push_str(
        "- Manifest warnings are treated as RustUse standardization debt, not necessarily publish blockers.\n",
    );

    Ok(markdown)
}

/* fn write_contents(markdown: &mut String) {
    markdown.push_str("## Contents\n\n");
    markdown.push_str("- [Action Plan](#action-plan)\n");
    markdown.push_str("- [Cargo Manifest Health](#cargo-manifest-health)\n");
    markdown.push_str("  - [Manifest Issue Summary](#manifest-issue-summary)\n");
    markdown.push_str("  - [Manifest Shape Summary](#manifest-shape-summary)\n");
    markdown.push_str("  - [Manifest Summary by Facade](#manifest-summary-by-facade)\n");
    markdown.push_str("  - [Manifest Issues](#manifest-issues)\n");
    markdown.push_str("- [Standard File Consistency](#standard-file-consistency)\n");
    markdown.push_str("- [Facade Inventory](#facade-inventory)\n");
    markdown.push_str("- [Notes](#notes)\n\n");
} */

fn write_contents(markdown: &mut String) {
    markdown.push_str("## Contents\n\n");
    markdown.push_str("- [Action Plan](#action-plan)\n");
    markdown.push_str("- [Cargo Manifest Health](#cargo-manifest-health)\n");
    markdown.push_str("  - [Manifest Issue Summary](#manifest-issue-summary)\n");
    markdown.push_str("  - [Manifest Shape Summary](#manifest-shape-summary)\n");
    markdown.push_str("  - [Manifest Summary by Facade](#manifest-summary-by-facade)\n");
    markdown.push_str("  - [Top Manifest Offenders](#top-manifest-offenders)\n");
    markdown.push_str("- [Standard File Consistency](#standard-file-consistency)\n");
    markdown.push_str("- [Facade Inventory](#facade-inventory)\n");
    markdown.push_str("- [Notes](#notes)\n\n");
}

fn write_action_plan(
    markdown: &mut String,
    root_name: &str,
    missing_root_repos: &[&str],
    missing_git: &[&FacadeEntry],
    standard_file_reports: &[StandardFileReport],
    manifest_reports: &[FacadeManifestReport],
    expected_count: usize,
) {
    let drifting_files = standard_file_reports
        .iter()
        .filter(|report| !report.is_consistent(expected_count))
        .collect::<Vec<_>>();

    let manifest_error_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::error_count)
        .sum::<usize>();
    let manifest_warning_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::warning_count)
        .sum::<usize>();
    let invalid_category_count = manifest_reports
        .iter()
        .map(FacadeManifestReport::invalid_category_count)
        .sum::<usize>();

    let manifest_error_facades = manifest_reports
        .iter()
        .filter(|report| report.error_count() > 0)
        .collect::<Vec<_>>();

    let has_action = root_name != "rustuse"
        || !missing_root_repos.is_empty()
        || !missing_git.is_empty()
        || !drifting_files.is_empty()
        || manifest_error_count > 0
        || manifest_warning_count > 0;

    markdown.push_str("## Action Plan\n\n");

    if !has_action {
        markdown.push_str("- No action required.\n\n");
        return;
    }

    if manifest_error_count > 0 {
        markdown.push_str(&format!(
            "- **Fix Cargo.toml errors first.** `scan` can be clean while manifests are still not publish-clean. There are {manifest_error_count} manifest error(s), including {invalid_category_count} invalid crates.io category slug(s).\n"
        ));
        markdown.push_str("- Prioritize manifest errors before standard-file drift. These can block publishing or break RustUse workspace consistency.\n");
    }

    if manifest_warning_count > 0 {
        markdown.push_str(&format!(
            "- Clean up Cargo.toml warnings next. There are {manifest_warning_count} warning(s). Focus on workspace shape, facade dependency/feature wiring, package metadata, README files, docs.rs metadata, lints inheritance, and inherited categories.\n"
        ));
    }

    if !manifest_error_facades.is_empty() {
        markdown.push_str("- Manifest error facades to fix first:\n");

        for report in manifest_error_facades {
            markdown.push_str(&format!(
                "  - `{}`: {} error(s), {} invalid category slug(s)\n",
                report.facade_name,
                report.error_count(),
                report.invalid_category_count()
            ));
        }
    }

    if root_name != "rustuse" {
        markdown.push_str(&format!(
            "- Rename the local development root from `{root_name}` to `rustuse` when convenient. This is lower priority than manifest publish blockers.\n"
        ));
    }

    if !missing_root_repos.is_empty() {
        markdown.push_str("- Restore missing root repositories:\n");

        for repo in missing_root_repos {
            markdown.push_str(&format!("  - `{repo}`\n"));
        }
    }

    if !missing_git.is_empty() {
        markdown.push_str("- Fix `use-*` directories missing `.git`:\n");

        for facade in missing_git {
            markdown.push_str(&format!("  - `{}`\n", facade.name));
        }
    }

    if !drifting_files.is_empty() {
        markdown
            .push_str("- Normalize standard files with drift after manifest errors are fixed:\n");

        for report in drifting_files {
            markdown.push_str(&format!(
                "  - `{}`: {} variant(s), {} missing\n",
                report.file_name,
                report.variants.len(),
                report.missing.len()
            ));
        }
    }

    markdown.push('\n');
}

fn write_manifest_summary(
    markdown: &mut String,
    reports: &[FacadeManifestReport],
    manifest_count: usize,
    issue_count: usize,
    error_count: usize,
    warning_count: usize,
    invalid_category_count: usize,
) {
    markdown.push_str("## Cargo Manifest Health\n\n");
    markdown.push_str(&format!("- Facades inspected: {}\n", reports.len()));
    markdown.push_str(&format!("- Manifests inspected: {manifest_count}\n"));
    markdown.push_str(&format!(
        "- Issues: {issue_count} ({error_count} error(s), {warning_count} warning(s))\n"
    ));
    markdown.push_str(&format!(
        "- Invalid crates.io category slugs: {invalid_category_count}\n"
    ));

    if error_count > 0 {
        markdown.push_str(
            "- Candor: this is the most important section of the report right now. Standard file drift is annoying; invalid manifest metadata blocks publishing or breaks RustUse shape.\n",
        );
    } else if warning_count > 0 {
        markdown.push_str(
            "- Candor: the manifest layer is publishable, but not standardized. The warnings are maintainability debt.\n",
        );
    } else {
        markdown.push_str("- Manifest layer is clean.\n");
    }

    markdown.push('\n');

    write_manifest_issue_summary(markdown, reports);
    write_manifest_shape_summary(markdown, reports);

    markdown.push_str("### Manifest Summary by Facade\n\n");
    markdown.push_str("| Status | Facade | Manifests | Errors | Warnings | Invalid Categories |\n");
    markdown.push_str("|---|---|---:|---:|---:|---:|\n");

    for report in reports {
        markdown.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} |\n",
            report.status(),
            report.facade_name,
            report.manifest_count(),
            report.error_count(),
            report.warning_count(),
            report.invalid_category_count()
        ));
    }

    markdown.push('\n');

    write_manifest_top_offenders(markdown, reports, TOP_MANIFEST_OFFENDER_LIMIT);
}

fn write_manifest_issue_summary(markdown: &mut String, reports: &[FacadeManifestReport]) {
    let mut summary: BTreeMap<(String, String), usize> = BTreeMap::new();

    for facade_report in reports {
        for manifest in &facade_report.manifests {
            for issue in &manifest.issues {
                let key = (issue.severity.as_str().to_string(), issue.code.to_string());
                *summary.entry(key).or_default() += 1;
            }
        }
    }

    if summary.is_empty() {
        return;
    }

    let mut rows = summary
        .into_iter()
        .map(|((severity, code), count)| (severity, code, count))
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        right
            .2
            .cmp(&left.2)
            .then_with(|| left.0.cmp(&right.0))
            .then_with(|| left.1.cmp(&right.1))
    });

    markdown.push_str("### Manifest Issue Summary\n\n");
    markdown.push_str("| Severity | Code | Count |\n");
    markdown.push_str("|---|---|---:|\n");

    for (severity, code, count) in rows {
        markdown.push_str(&format!("| `{severity}` | `{code}` | {count} |\n"));
    }

    markdown.push('\n');
}

fn write_manifest_shape_summary(markdown: &mut String, reports: &[FacadeManifestReport]) {
    let mut buckets: BTreeMap<&'static str, usize> = BTreeMap::new();

    for facade_report in reports {
        for manifest in &facade_report.manifests {
            for issue in &manifest.issues {
                let bucket = manifest_shape_bucket(issue.code);
                *buckets.entry(bucket).or_default() += 1;
            }
        }
    }

    if buckets.is_empty() {
        return;
    }

    let mut rows = buckets.into_iter().collect::<Vec<_>>();

    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));

    markdown.push_str("### Manifest Shape Summary\n\n");
    markdown.push_str("| Shape Area | Issues |\n");
    markdown.push_str("|---|---:|\n");

    for (bucket, count) in rows {
        markdown.push_str(&format!("| {bucket} | {count} |\n"));
    }

    markdown.push('\n');
}

fn write_manifest_top_offenders(
    markdown: &mut String,
    reports: &[FacadeManifestReport],
    limit: usize,
) {
    let mut reports_with_issues = reports
        .iter()
        .filter(|report| report.issue_count() > 0)
        .collect::<Vec<_>>();

    if reports_with_issues.is_empty() {
        return;
    }

    reports_with_issues.sort_by(|left, right| {
        right
            .issue_count()
            .cmp(&left.issue_count())
            .then_with(|| right.error_count().cmp(&left.error_count()))
            .then_with(|| right.warning_count().cmp(&left.warning_count()))
            .then_with(|| left.facade_name.cmp(&right.facade_name))
    });

    markdown.push_str("### Top Manifest Offenders\n\n");
    markdown.push_str(&format!(
        "Showing the top {} facade(s) by manifest issue count. Full per-manifest details should be generated through a verbose report mode later.\n\n",
        limit.min(reports_with_issues.len())
    ));

    markdown.push_str("| Rank | Status | Facade | Manifests | Issues | Errors | Warnings | Invalid Categories |\n");
    markdown.push_str("|---:|---|---|---:|---:|---:|---:|---:|\n");

    for (index, report) in reports_with_issues.iter().take(limit).enumerate() {
        markdown.push_str(&format!(
            "| {} | {} | `{}` | {} | {} | {} | {} | {} |\n",
            index + 1,
            report.status(),
            report.facade_name,
            report.manifest_count(),
            report.issue_count(),
            report.error_count(),
            report.warning_count(),
            report.invalid_category_count()
        ));
    }

    markdown.push('\n');
}

fn write_standard_file_report(
    markdown: &mut String,
    report: &StandardFileReport,
    expected_count: usize,
) {
    markdown.push_str(&format!("## `{}` Consistency\n\n", report.file_name));
    markdown.push_str(&format!(
        "- Present: {}/{}\n",
        report.present_count, expected_count
    ));
    markdown.push_str(&format!("- Missing: {}\n", report.missing.len()));
    markdown.push_str(&format!("- Content variants: {}\n", report.variants.len()));
    markdown.push_str(&format!(
        "- Consistent: {}\n",
        yes_no(report.is_consistent(expected_count))
    ));

    if let Some(majority) = report.variants.first() {
        markdown.push_str(&format!(
            "- Majority variant: `{}` used by {} facade(s)\n",
            majority.hash,
            majority.facades.len()
        ));
    }

    markdown.push('\n');

    if !report.missing.is_empty() {
        markdown.push_str(&format!("### Missing `{}`\n\n", report.file_name));

        for facade in &report.missing {
            markdown.push_str(&format!("- `{facade}`\n"));
        }

        markdown.push('\n');
    }

    if !report.variants.is_empty() {
        markdown.push_str(&format!("### `{}` Variants\n\n", report.file_name));
        markdown.push_str("| Variant | Facades | Lines | Bytes | Examples |\n");
        markdown.push_str("|---|---:|---:|---:|---|\n");

        for variant in &report.variants {
            markdown.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                variant.hash,
                variant.facades.len(),
                variant.line_count,
                variant.byte_len,
                sample_facades(&variant.facades)
            ));
        }

        markdown.push('\n');
    }
}

fn sample_facades(facades: &[String]) -> String {
    let sample_count = 8;
    let mut names = facades
        .iter()
        .take(sample_count)
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>();

    if facades.len() > sample_count {
        names.push(format!("... +{} more", facades.len() - sample_count));
    }

    names.join(", ")
}

fn display_markdown_version(version: &Option<String>) -> String {
    match version {
        Some(version) => format!("`{version}`"),
        None => "`<missing>`".to_string(),
    }
}

pub(crate) fn generate_markdown_report(
    root: &Path,
    output: Output,
    destination: ReportDestination,
) -> Result<()> {
    let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    let report = build_report(&root)?;

    match destination {
        ReportDestination::Stdout => {
            emit_markdown_to_stdout(&report);
            return Ok(());
        },
        ReportDestination::File(path) => {
            let output_path = path.unwrap_or_else(|| root.join("rustuse-root-report.md"));

            if output.is_json() {
                write_report(&output_path, &report)?;

                output.record(
                    "report",
                    "ok",
                    &format!(
                        "generated RustUse root report for {}; wrote {}",
                        root.display(),
                        output_path.display()
                    ),
                );

                return Ok(());
            }

            output.line(format!(
                "RustUse development root report - root: {}",
                root.display()
            ));

            write_report(&output_path, &report)?;

            output.line(format!("wrote: {}", output_path.display()));
        },
    }

    Ok(())
}
