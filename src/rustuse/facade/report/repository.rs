use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::facade::inspect::{FacadeRepositoryReport, RepositorySurfaceCheck};
use crate::rustuse::report::markdown::yes_no;

pub(crate) fn write_repository_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let report = &diagnostics.repository;

    markdown.push_str("## Repository Surface\n\n");
    markdown.push_str(&format!("- Status: **{}**\n", report.status().as_str()));
    markdown.push_str(&format!(
        "- Required files: `{}/{}`\n",
        present_required_file_count(report),
        required_file_count(report)
    ));
    markdown.push_str(&format!(
        "- Optional files: `{}/{}`\n",
        present_optional_file_count(report),
        optional_file_count(report)
    ));
    markdown.push_str(&format!(
        "- Required directories: `{}/{}`\n",
        present_required_directory_count(report),
        required_directory_count(report)
    ));
    markdown.push_str(&format!(
        "- Optional directories: `{}/{}`\n\n",
        present_optional_directory_count(report),
        optional_directory_count(report)
    ));
}

pub(crate) fn write_standard_file_consistency(
    markdown: &mut String,
    diagnostics: &FacadeDiagnostics,
) {
    markdown.push_str("## Standard File Consistency\n\n");

    markdown.push_str("### Files\n\n");
    write_repository_checks(markdown, &diagnostics.repository.files);

    markdown.push_str("### Directories\n\n");
    write_repository_checks(markdown, &diagnostics.repository.directories);
}

fn write_repository_checks(markdown: &mut String, checks: &[RepositorySurfaceCheck]) {
    markdown.push_str("| Status | Required | Present | Path | Purpose |\n");
    markdown.push_str("|---|---:|---:|---|---|\n");

    for check in checks {
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

fn required_file_count(report: &FacadeRepositoryReport) -> usize {
    report.files.iter().filter(|check| check.required).count()
}

fn present_required_file_count(report: &FacadeRepositoryReport) -> usize {
    report
        .files
        .iter()
        .filter(|check| check.required && check.present)
        .count()
}

fn optional_file_count(report: &FacadeRepositoryReport) -> usize {
    report.files.iter().filter(|check| !check.required).count()
}

fn present_optional_file_count(report: &FacadeRepositoryReport) -> usize {
    report
        .files
        .iter()
        .filter(|check| !check.required && check.present)
        .count()
}

fn required_directory_count(report: &FacadeRepositoryReport) -> usize {
    report
        .directories
        .iter()
        .filter(|check| check.required)
        .count()
}

fn present_required_directory_count(report: &FacadeRepositoryReport) -> usize {
    report
        .directories
        .iter()
        .filter(|check| check.required && check.present)
        .count()
}

fn optional_directory_count(report: &FacadeRepositoryReport) -> usize {
    report
        .directories
        .iter()
        .filter(|check| !check.required)
        .count()
}

fn present_optional_directory_count(report: &FacadeRepositoryReport) -> usize {
    report
        .directories
        .iter()
        .filter(|check| !check.required && check.present)
        .count()
}
