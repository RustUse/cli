//! Markdown report generation for a local RustUse fleet.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::output::Output;
use crate::rustuse::facade::codes::{FacadeIssueBucket, FacadeIssueCode};
use crate::rustuse::facade::manifest::FacadeManifestReport;
use crate::rustuse::fleet::diagnostics::FleetDiagnostics;
use crate::rustuse::fleet::discover::{FleetFacadeEntry, display_version};
use crate::rustuse::report::markdown::{write_markdown_report, yes_no};

const FLEET_REPORT_FILE_NAME: &str = "rustuse-fleet-report.md";
const TOP_MANIFEST_OFFENDER_LIMIT: usize = 15;

pub(crate) fn generate_markdown_report(fleet: &Path, output: Output) -> Result<()> {
    let diagnostics = FleetDiagnostics::inspect(fleet)?;
    let report = build_report(&diagnostics);
    let output_path = default_fleet_report_path(&diagnostics.fleet);

    write_markdown_report(&output_path, &report)?;

    if output.is_json() {
        output.record(
            "fleet report",
            diagnostics.status(),
            &format!("wrote {}", output_path.display()),
        );
    } else {
        output.line(format!(
            "RustUse fleet report - fleet: {}",
            diagnostics.fleet.display()
        ));
        output.line(format!("wrote: {}", output_path.display()));
    }

    Ok(())
}

fn default_fleet_report_path(fleet: &Path) -> PathBuf {
    fleet.join(FLEET_REPORT_FILE_NAME)
}

fn build_report(diagnostics: &FleetDiagnostics) -> String {
    let mut markdown = String::new();

    write_summary(&mut markdown, diagnostics);
    write_contents(&mut markdown);
    write_fleet_repositories(&mut markdown, diagnostics);
    write_manifest_health(&mut markdown, diagnostics);
    write_facade_inventory(&mut markdown, diagnostics);
    write_notes(&mut markdown);

    markdown
}

fn write_summary(markdown: &mut String, diagnostics: &FleetDiagnostics) {
    markdown.push_str("# RustUse Fleet Report\n\n");
    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!("- Fleet: `{}`\n", diagnostics.fleet.display()));
    markdown.push_str(&format!("- Fleet name: `{}`\n", diagnostics.name));
    markdown.push_str(&format!(
        "- CLI repository present: {}\n",
        yes_no(diagnostics.has_cli())
    ));
    markdown.push_str(&format!(
        "- Documentation repository present: {}\n",
        yes_no(diagnostics.has_docs())
    ));
    markdown.push_str(&format!(
        "- Standard fleet name: {}\n",
        if diagnostics.name == "rustuse" {
            "yes"
        } else {
            "no, expected `rustuse`"
        }
    ));
    markdown.push_str(&format!(
        "- Fleet repositories missing: `{}`\n",
        diagnostics.missing_fleet_repo_count()
    ));
    markdown.push_str(&format!(
        "- `use-*` facades: `{}`\n",
        diagnostics.facade_count()
    ));
    markdown.push_str(&format!(
        "- Facades with `.git`: `{}`\n",
        diagnostics.facade_git_count()
    ));
    markdown.push_str(&format!(
        "- Facades missing `.git`: `{}`\n",
        diagnostics.missing_facade_git_count()
    ));
    markdown.push_str(&format!(
        "- Child crates detected: `{}`\n",
        diagnostics.child_crate_count()
    ));
    markdown.push_str(&format!(
        "- Cargo manifests inspected: `{}`\n",
        diagnostics.manifest_count()
    ));
    markdown.push_str(&format!(
        "- Cargo manifest issues: `{}` (`{}` error(s), `{}` warning(s))\n",
        diagnostics.manifest_issue_count(),
        diagnostics.manifest_error_count(),
        diagnostics.manifest_warning_count()
    ));
    markdown.push_str(&format!(
        "- Invalid crates.io category slugs: `{}`\n",
        diagnostics.invalid_category_count()
    ));
    markdown.push_str(&format!("- Status: **{}**\n\n", diagnostics.status()));

    let missing_git = diagnostics.missing_facade_git_names().collect::<Vec<_>>();

    if !missing_git.is_empty() {
        markdown.push_str("- Facades missing `.git`: ");
        markdown.push_str(
            &missing_git
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", "),
        );
        markdown.push_str("\n\n");
    }
}

fn write_contents(markdown: &mut String) {
    markdown.push_str("## Contents\n\n");
    markdown.push_str("- [Fleet Repositories](#fleet-repositories)\n");
    markdown.push_str("- [Cargo Manifest Health](#cargo-manifest-health)\n");
    markdown.push_str("  - [Manifest Issue Summary](#manifest-issue-summary)\n");
    markdown.push_str("  - [Manifest Shape Summary](#manifest-shape-summary)\n");
    markdown.push_str("  - [Manifest Summary by Facade](#manifest-summary-by-facade)\n");
    markdown.push_str("  - [Top Manifest Offenders](#top-manifest-offenders)\n");
    markdown.push_str("- [Facade Inventory](#facade-inventory)\n");
    markdown.push_str("- [Notes](#notes)\n\n");
}

fn write_fleet_repositories(markdown: &mut String, diagnostics: &FleetDiagnostics) {
    markdown.push_str("## Fleet Repositories\n\n");
    markdown.push_str("| Repository | Present | Git |\n");
    markdown.push_str("|---|---:|---:|\n");

    for repo in &diagnostics.repos {
        markdown.push_str(&format!(
            "| `{}` | {} | {} |\n",
            repo.name,
            yes_no(repo.present),
            yes_no(repo.has_git)
        ));
    }

    markdown.push('\n');
}

fn write_manifest_health(markdown: &mut String, diagnostics: &FleetDiagnostics) {
    markdown.push_str("## Cargo Manifest Health\n\n");
    markdown.push_str(&format!(
        "- Facades inspected: `{}`\n",
        diagnostics.manifests.len()
    ));
    markdown.push_str(&format!(
        "- Manifests inspected: `{}`\n",
        diagnostics.manifest_count()
    ));
    markdown.push_str(&format!(
        "- Issues: `{}` (`{}` error(s), `{}` warning(s))\n",
        diagnostics.manifest_issue_count(),
        diagnostics.manifest_error_count(),
        diagnostics.manifest_warning_count()
    ));
    markdown.push_str(&format!(
        "- Invalid crates.io category slugs: `{}`\n\n",
        diagnostics.invalid_category_count()
    ));

    write_manifest_issue_summary(markdown, &diagnostics.manifests);
    write_manifest_shape_summary(markdown, &diagnostics.manifests);
    write_manifest_summary_by_facade(markdown, &diagnostics.manifests);
    write_manifest_top_offenders(
        markdown,
        &diagnostics.manifests,
        TOP_MANIFEST_OFFENDER_LIMIT,
    );
}

fn write_manifest_issue_summary(markdown: &mut String, reports: &[FacadeManifestReport]) {
    let mut summary = BTreeMap::<(&'static str, FacadeIssueCode), usize>::new();

    for facade_report in reports {
        for manifest in &facade_report.manifests {
            for issue in &manifest.issues {
                let key = (issue.severity.as_str(), issue.code);
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
            .then_with(|| left.0.cmp(right.0))
            .then_with(|| left.1.cmp(&right.1))
    });

    markdown.push_str("### Manifest Issue Summary\n\n");
    markdown.push_str("| Severity | Code | Count |\n");
    markdown.push_str("|---|---|---:|\n");

    for (severity, code, count) in rows {
        markdown.push_str(&format!(
            "| `{severity}` | `{}` | {count} |\n",
            code.as_str()
        ));
    }

    markdown.push('\n');
}

fn write_manifest_shape_summary(markdown: &mut String, reports: &[FacadeManifestReport]) {
    let mut buckets = BTreeMap::<FacadeIssueBucket, usize>::new();

    for facade_report in reports {
        for manifest in &facade_report.manifests {
            for issue in &manifest.issues {
                *buckets.entry(issue.code.bucket()).or_default() += 1;
            }
        }
    }

    if buckets.is_empty() {
        return;
    }

    let mut rows = buckets.into_iter().collect::<Vec<_>>();

    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

    markdown.push_str("### Manifest Shape Summary\n\n");
    markdown.push_str("| Shape Area | Issues |\n");
    markdown.push_str("|---|---:|\n");

    for (bucket, count) in rows {
        markdown.push_str(&format!("| {} | {count} |\n", bucket.as_str()));
    }

    markdown.push('\n');
}

fn write_manifest_summary_by_facade(markdown: &mut String, reports: &[FacadeManifestReport]) {
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
        "Showing the top {} facade(s) by manifest issue count.\n\n",
        limit.min(reports_with_issues.len())
    ));
    markdown.push_str(
        "| Rank | Status | Facade | Manifests | Issues | Errors | Warnings | Invalid Categories |\n",
    );
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

fn write_facade_inventory(markdown: &mut String, diagnostics: &FleetDiagnostics) {
    markdown.push_str("## Facade Inventory\n\n");
    markdown
        .push_str("| Status | Facade | Version | Git | Cargo.toml | crates/ | Child crates |\n");
    markdown.push_str("|---|---|---:|---:|---:|---:|---:|\n");

    for facade in &diagnostics.facades {
        markdown.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} |\n",
            facade.status(),
            facade.name,
            display_markdown_version(facade),
            yes_no(facade.has_git),
            yes_no(facade.has_cargo_toml),
            yes_no(facade.has_crates_dir),
            facade.child_crate_count
        ));
    }

    markdown.push('\n');
}

fn display_markdown_version(facade: &FleetFacadeEntry) -> String {
    format!("`{}`", display_version(&facade.version))
}

fn write_notes(markdown: &mut String) {
    markdown.push_str("## Notes\n\n");
    markdown.push_str("- This report is generated from the local filesystem.\n");
    markdown.push_str("- `use-*` directories are treated as facade candidates in the Fleet.\n");
    markdown.push_str("- A facade repository is expected to contain its own `.git` directory.\n");
    markdown.push_str(
        "- `crates/` child counts only include direct child directories with `Cargo.toml`.\n",
    );
    markdown.push_str(
        "- Manifest errors are treated as publish blockers because crates.io rejects invalid category slugs and other invalid Cargo metadata.\n",
    );
}
