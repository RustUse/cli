use crate::rustuse::facade::diagnostics::FacadeDiagnostics;

pub(crate) fn write_non_standard_paths(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let report = &diagnostics.non_standard_paths;

    markdown.push_str("## Non-standard Paths\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", report.status()));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        report.present_count(),
        report.total_count()
    ));

    let present_checks = report.present_checks();

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
