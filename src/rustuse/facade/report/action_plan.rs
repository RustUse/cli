use std::collections::BTreeMap;

use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::destination::report_path;

pub(crate) fn write_action_plan(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("## Action Plan\n\n");

    if diagnostics.issues.is_empty() {
        markdown.push_str("- No required action.\n\n");
        return;
    }

    markdown.push_str(&format!(
        "- Address {} issue(s): {} error(s), {} warning(s).\n",
        diagnostics.issue_count(),
        diagnostics.error_count(),
        diagnostics.warning_count()
    ));

    let fixable_count = diagnostics.fixable_count();
    let manual_count = diagnostics.issue_count().saturating_sub(fixable_count);

    markdown.push_str(&format!(
        "- Fixable by RustUse: `{fixable_count}` issue(s).\n"
    ));
    markdown.push_str(&format!(
        "- Manual review required: `{manual_count}` issue(s).\n"
    ));

    if diagnostics.error_count() > 0 {
        markdown.push_str("- Fix errors before addressing warnings.\n");
    }

    markdown.push('\n');

    write_issue_group_summary(markdown, diagnostics);
    write_fixable_issue_summary(markdown, diagnostics);
    write_priority_issues(markdown, diagnostics);
}

fn write_issue_group_summary(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let mut buckets = BTreeMap::<&str, usize>::new();

    for issue in &diagnostics.issues {
        *buckets.entry(issue.bucket).or_default() += 1;
    }

    if buckets.is_empty() {
        return;
    }

    markdown.push_str("### Issue Groups\n\n");
    markdown.push_str("| Group | Issues |\n");
    markdown.push_str("|---|---:|\n");

    for (bucket, count) in buckets {
        markdown.push_str(&format!("| {bucket} | {count} |\n"));
    }

    markdown.push('\n');
}

fn write_fixable_issue_summary(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let mut fixable_by_code = BTreeMap::<&str, usize>::new();

    for issue in diagnostics.fixable_issues() {
        *fixable_by_code.entry(issue.code).or_default() += 1;
    }

    if fixable_by_code.is_empty() {
        return;
    }

    markdown.push_str("### Fixable Issue Types\n\n");
    markdown.push_str("| Code | Count |\n");
    markdown.push_str("|---|---:|\n");

    for (code, count) in fixable_by_code {
        markdown.push_str(&format!("| `{code}` | {count} |\n"));
    }

    markdown.push('\n');
}

fn write_priority_issues(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let mut issues = diagnostics.issues.iter().collect::<Vec<_>>();

    issues.sort_by(|left, right| {
        left.severity
            .sort_rank()
            .cmp(&right.severity.sort_rank())
            .then_with(|| left.bucket.cmp(right.bucket))
            .then_with(|| left.code.cmp(right.code))
            .then_with(|| left.message.cmp(&right.message))
    });

    markdown.push_str("### Priority Issues\n\n");
    markdown.push_str("| Severity | Code | Path | Message |\n");
    markdown.push_str("|---|---|---|---|\n");

    for issue in issues.iter().take(20) {
        let path = issue
            .path
            .as_deref()
            .map_or_else(|| "-".to_owned(), report_path);

        markdown.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            issue.severity.as_str(),
            issue.code,
            path,
            issue.message
        ));
    }

    if issues.len() > 20 {
        markdown.push_str(&format!(
            "\nShowing 20 of {} issue(s). See later sections for full details.\n",
            issues.len()
        ));
    }

    markdown.push('\n');
}
