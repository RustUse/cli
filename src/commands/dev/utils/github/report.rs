#![allow(dead_code)]

//! Report rendering for `.github` consistency checks.

use std::fmt::Write;

use super::model::{GithubCheckReport, GithubFileCheck, GithubFileStatus};

/// Options used while rendering a `.github` report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GithubReportOptions {
    /// Include checks whose status is `ok`.
    pub(crate) include_ok_checks: bool,

    /// Include expected and actual content hashes.
    pub(crate) include_hashes: bool,
}

impl Default for GithubReportOptions {
    fn default() -> Self {
        Self {
            include_ok_checks: true,
            include_hashes: true,
        }
    }
}

/// Renders a GitHub-flavored Markdown `.github` report.
#[must_use]
pub(crate) fn render_markdown(report: &GithubCheckReport) -> String {
    render_markdown_with_options(report, &GithubReportOptions::default())
}

/// Renders a GitHub-flavored Markdown `.github` report with options.
#[must_use]
pub(crate) fn render_markdown_with_options(
    report: &GithubCheckReport,
    options: &GithubReportOptions,
) -> String {
    let mut output = String::new();

    writeln!(output, "## .github Consistency").expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");

    writeln!(output, "- Facade: `{}`", report.target.facade)
        .expect("writing to string should not fail");
    writeln!(output, "- Path: `{}`", report.facade_path.display())
        .expect("writing to string should not fail");
    writeln!(output, "- Status: **{}**", report_status(report))
        .expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");

    writeln!(output, "### Summary").expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");
    writeln!(output, "| Metric | Count |").expect("writing to string should not fail");
    writeln!(output, "|---|---:|").expect("writing to string should not fail");
    writeln!(output, "| Checks | {} |", report.checks.len())
        .expect("writing to string should not fail");
    writeln!(output, "| Ok | {} |", report.ok_check_count())
        .expect("writing to string should not fail");
    writeln!(output, "| Missing | {} |", report.missing_check_count())
        .expect("writing to string should not fail");
    writeln!(output, "| Stale | {} |", report.stale_check_count())
        .expect("writing to string should not fail");
    writeln!(
        output,
        "| Invalid kind | {} |",
        report.invalid_kind_check_count()
    )
    .expect("writing to string should not fail");
    writeln!(output, "| Errors | {} |", report.error_count())
        .expect("writing to string should not fail");
    writeln!(output, "| Warnings | {} |", report.warning_count())
        .expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");

    render_markdown_issues(report, &mut output);
    render_markdown_checks(report, options, &mut output);

    output
}

fn render_markdown_issues(report: &GithubCheckReport, output: &mut String) {
    writeln!(output, "### Issues").expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");

    if report.issues.is_empty() {
        writeln!(output, "No `.github` issues found.").expect("writing to string should not fail");
        writeln!(output).expect("writing to string should not fail");
        return;
    }

    writeln!(output, "| Severity | Path | Code | Message |")
        .expect("writing to string should not fail");
    writeln!(output, "|---|---|---|---|").expect("writing to string should not fail");

    for issue in &report.issues {
        writeln!(
            output,
            "| {} | `{}` | `{}` | {} |",
            issue.severity.as_str(),
            escape_markdown_table_cell(&issue.path),
            issue.code.as_str(),
            escape_markdown_table_cell(&issue.message)
        )
        .expect("writing to string should not fail");
    }

    writeln!(output).expect("writing to string should not fail");
}

fn render_markdown_checks(
    report: &GithubCheckReport,
    options: &GithubReportOptions,
    output: &mut String,
) {
    writeln!(output, "### File Checks").expect("writing to string should not fail");
    writeln!(output).expect("writing to string should not fail");

    let checks = visible_checks(report, options);

    if checks.is_empty() {
        writeln!(output, "No `.github` file checks to show.")
            .expect("writing to string should not fail");
        writeln!(output).expect("writing to string should not fail");
        return;
    }

    if options.include_hashes {
        writeln!(output, "| Status | Path | Expected hash | Actual hash |")
            .expect("writing to string should not fail");
        writeln!(output, "|---|---|---:|---:|").expect("writing to string should not fail");

        for check in checks {
            writeln!(
                output,
                "| {} | `{}` | `{}` | `{}` |",
                check.status.as_str(),
                escape_markdown_table_cell(check.relative_path),
                option_hash(check.expected_hash.as_deref()),
                option_hash(check.actual_hash.as_deref())
            )
            .expect("writing to string should not fail");
        }
    } else {
        writeln!(output, "| Status | Path |").expect("writing to string should not fail");
        writeln!(output, "|---|---|").expect("writing to string should not fail");

        for check in checks {
            writeln!(
                output,
                "| {} | `{}` |",
                check.status.as_str(),
                escape_markdown_table_cell(check.relative_path)
            )
            .expect("writing to string should not fail");
        }
    }

    writeln!(output).expect("writing to string should not fail");
}

fn visible_checks<'a>(
    report: &'a GithubCheckReport,
    options: &GithubReportOptions,
) -> Vec<&'a GithubFileCheck> {
    report
        .checks
        .iter()
        .filter(|check| options.include_ok_checks || check.status != GithubFileStatus::Ok)
        .collect()
}

fn report_status(report: &GithubCheckReport) -> &'static str {
    if report.has_errors() {
        "error"
    } else if report.warning_count() > 0 {
        "warning"
    } else {
        "ok"
    }
}

fn option_hash(hash: Option<&str>) -> &str {
    hash.unwrap_or("-")
}

fn escape_markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', "<br>")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::issue::{GithubIssue, GithubIssueCode, GithubIssueSeverity};
    use super::super::model::{GithubCheckReport, GithubFileCheck, GithubFileStatus, GithubTarget};
    use super::{GithubReportOptions, render_markdown, render_markdown_with_options};

    #[test]
    fn markdown_report_includes_summary_and_issues() {
        let report = sample_report();

        let rendered = render_markdown(&report);

        assert!(rendered.contains("## .github Consistency"));
        assert!(rendered.contains("| Warnings | 1 |"));
        assert!(rendered.contains("missing-github-workflow"));
    }

    #[test]
    fn markdown_report_can_hide_ok_checks() {
        let report = sample_report();

        let rendered = render_markdown_with_options(
            &report,
            &GithubReportOptions {
                include_ok_checks: false,
                include_hashes: false,
            },
        );

        assert!(!rendered.contains(".github/dependabot.yml"));
        assert!(rendered.contains(".github/workflows/ci.yml"));
    }

    fn sample_report() -> GithubCheckReport {
        GithubCheckReport {
            target: GithubTarget {
                facade: "use-example".to_owned(),
                path: PathBuf::from("use-example"),
            },
            facade_path: PathBuf::from("use-example"),
            checks: vec![
                GithubFileCheck {
                    relative_path: ".github/dependabot.yml",
                    status: GithubFileStatus::Ok,
                    expected_hash: Some("expected".to_owned()),
                    actual_hash: Some("expected".to_owned()),
                },
                GithubFileCheck {
                    relative_path: ".github/workflows/ci.yml",
                    status: GithubFileStatus::Missing,
                    expected_hash: Some("expected".to_owned()),
                    actual_hash: None,
                },
            ],
            issues: vec![GithubIssue::new(
                GithubIssueSeverity::Warning,
                GithubIssueCode::MissingGithubWorkflow,
                ".github/workflows/ci.yml",
                "Required `.github` file is missing.",
            )],
        }
    }
}
