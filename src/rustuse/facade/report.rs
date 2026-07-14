//! Markdown report generation for one RustUse facade repository.

use std::path::Path;

use anyhow::Result;

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

pub(crate) fn generate_markdown_report(root: &Path, output: Output) -> Result<()> {
    let diagnostics = FacadeDiagnostics::inspect(root)?;
    let report = build_report(&diagnostics);
    let output_path = default_report_path(&diagnostics.facade.root);

    write_markdown_report(&output_path, &report)?;

    if output.is_json() {
        output.record(
            "facade report",
            diagnostics.status(),
            &format!("wrote {}", output_path.display()),
        );
    } else {
        output.line(format!(
            "RustUse facade report - root: {}",
            diagnostics.facade.root.display()
        ));
        output.line(format!("wrote: {}", output_path.display()));
    }

    Ok(())
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
