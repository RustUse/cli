use std::collections::BTreeMap;

use crate::rustuse::facade::codes::FacadeIssueCode;
use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::facade::manifest::FacadeManifestReport;
use crate::rustuse::report::destination::report_path;

pub(crate) fn write_manifest_health(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let report = &diagnostics.manifest;

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
    write_manifest_issues(markdown, diagnostics);
}

fn write_manifest_issue_summary(markdown: &mut String, report: &FacadeManifestReport) {
    let mut summary = BTreeMap::<(&'static str, FacadeIssueCode), usize>::new();

    for issue in report
        .manifests
        .iter()
        .flat_map(|manifest| manifest.issues.iter())
    {
        let key = (issue.severity.as_str(), issue.code);
        *summary.entry(key).or_default() += 1;
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

fn write_manifest_inventory(markdown: &mut String, report: &FacadeManifestReport) {
    markdown.push_str("### Manifest Inventory\n\n");
    markdown.push_str("| Kind | Package | Status | Issues | Manifest |\n");
    markdown.push_str("|---|---|---:|---:|---|\n");

    for manifest in &report.manifests {
        let package = manifest.package_name.as_deref().unwrap_or("<none>");
        let display_path = report_path(&manifest.path);

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

fn write_manifest_issues(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let report = &diagnostics.manifest;

    if report.issue_count() == 0 {
        if diagnostics.facade.crate_count() == 0 {
            markdown.push_str("- No child crate manifests were discovered.\n\n");
        }

        return;
    }

    markdown.push_str("### Manifest Issues\n\n");

    for manifest in &report.manifests {
        if manifest.issues.is_empty() {
            continue;
        }

        let display_path = report_path(&manifest.path);

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
                issue.code.as_str(),
                issue.message
            ));
        }

        markdown.push('\n');
    }

    if diagnostics.facade.crate_count() == 0 {
        markdown.push_str("- No child crate manifests were discovered.\n\n");
    }
}
