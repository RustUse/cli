//! Markdown report generation for one RustUse facade repository.

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::output::Output;
use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::destination::default_report_path;
use crate::rustuse::report::markdown::write_markdown_report;

mod action_plan;
mod child_crates;
mod ci_cd;
mod contents;
mod documentation;
mod expected_structure;
mod manifest;
mod non_standard;
mod release;
mod repository;
mod shape;
mod summary;
mod tooling;

#[derive(Debug, Serialize)]
struct FacadeReportResponse {
    command: &'static str,
    status: String,
    root: PathBuf,
    output: PathBuf,
    errors: usize,
    warnings: usize,
}

pub(crate) fn generate_markdown_report(root: &Path, output: Output) -> Result<()> {
    let diagnostics = FacadeDiagnostics::inspect(root)?;
    let report = build_report(&diagnostics);
    let output_path = default_report_path(&diagnostics.facade.root);

    write_markdown_report(&output_path, &report)?;

    let response = FacadeReportResponse {
        command: "facade report",
        status: diagnostics.status().to_owned(),
        root: diagnostics.facade.root.clone(),
        output: output_path,
        errors: diagnostics.error_count(),
        warnings: diagnostics.warning_count(),
    };

    render(&output, &response)
}

fn render(output: &Output, response: &FacadeReportResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    output.line(format!(
        "RustUse facade report - root: {}",
        response.root.display()
    ))?;
    output.line(format!("status: {}", response.status))?;
    output.line(format!("errors: {}", response.errors))?;
    output.line(format!("warnings: {}", response.warnings))?;
    output.line(format!("wrote: {}", response.output.display()))
}

fn build_report(diagnostics: &FacadeDiagnostics) -> String {
    let mut markdown = String::new();

    summary::write_summary(&mut markdown, diagnostics);
    contents::write_contents(&mut markdown);
    action_plan::write_action_plan(&mut markdown, diagnostics);
    expected_structure::write_expected_facade_structure(&mut markdown);
    shape::write_detected_facade_shape(&mut markdown, diagnostics);
    repository::write_repository_surface(&mut markdown, diagnostics);
    manifest::write_manifest_health(&mut markdown, diagnostics);
    child_crates::write_child_crates(&mut markdown, diagnostics);
    documentation::write_crate_documentation_consistency(&mut markdown, diagnostics);
    repository::write_standard_file_consistency(&mut markdown, diagnostics);
    non_standard::write_non_standard_paths(&mut markdown, diagnostics);
    tooling::write_tooling_configuration(&mut markdown, diagnostics);
    ci_cd::write_ci_cd_surface(&mut markdown, diagnostics);
    documentation::write_documentation_surface(&mut markdown, diagnostics);
    release::write_release_surface(&mut markdown, diagnostics);
    summary::write_notes(&mut markdown);

    markdown
}
