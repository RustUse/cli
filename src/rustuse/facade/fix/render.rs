//! Renders facade repair results.
//!
//! This module formats repair summaries for terminal and JSON output. It does
//! not inspect repositories, transform manifests, or write files.

use crate::output::Output;

use super::model::{FacadeFixPlan, FacadeFixSummary, FixMode};

pub(crate) fn write_summary(
    output: Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) {
    if output.is_json() {
        write_json_summary(output, mode, plan, summary);
    } else {
        write_text_summary(output, mode, plan, summary);
    }
}

fn write_json_summary(
    output: Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) {
    let status = summary_status(plan, summary);

    let message = format!(
        "root={}, mode={}, files_inspected={}, files_changed={}, files_written={}, \
         files_created={}, files_unchanged={}",
        plan.root.display(),
        mode.as_str(),
        summary.files_inspected,
        plan.files_changed(),
        summary.files_written(),
        plan.files_created(),
        summary.files_unchanged,
    );

    output.record("facade fix", status, &message);
}

fn write_text_summary(
    output: Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) {
    output.line(format!(
        "RustUse facade fix - root: {}",
        plan.root.display()
    ));

    output.line(format!("mode: {}", mode.as_str()));

    output.line(format!("status: {}", summary_status(plan, summary)));

    output.line(format!("files inspected: {}", summary.files_inspected));

    output.line(format!("files changed: {}", plan.files_changed()));

    output.line(format!("files written: {}", summary.files_written()));

    output.line(format!("files created: {}", plan.files_created()));

    output.line(format!("files unchanged: {}", summary.files_unchanged));

    if plan.is_empty() {
        return;
    }

    output.line("");
    output.line("changed files:");

    for change in &summary.changes {
        let action = if change.wrote { "wrote" } else { "would write" };

        let created = if change.created { " (created)" } else { "" };

        output.line(format!("- {action}: {}{created}", change.path.display()));
    }
}

fn summary_status<'a>(plan: &FacadeFixPlan, summary: &FacadeFixSummary) -> &'a str {
    if !summary.has_changes() {
        return "ok";
    }

    if summary.files_written() == plan.files_changed() {
        return "fixed";
    }

    "dry-run"
}
