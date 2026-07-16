//! Renders facade repair results.
//!
//! This module formats repair summaries for terminal and JSON output. It does
//! not inspect repositories, transform manifests, or write files.

use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;

use crate::output::Output;

use super::model::{FacadeFixPlan, FacadeFixSummary, FixMode};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum FacadeFixStatus {
    Ok,
    Fixed,
    DryRun,
}

impl FacadeFixStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Fixed => "fixed",
            Self::DryRun => "dry-run",
        }
    }
}

#[derive(Debug, Serialize)]
struct FacadeFixResponse {
    command: &'static str,
    status: FacadeFixStatus,
    root: PathBuf,
    mode: &'static str,
    files_inspected: usize,
    files_changed: usize,
    files_written: usize,
    files_created: usize,
    files_unchanged: usize,
    changes: Vec<FacadeFixChangeResponse>,
}

#[derive(Debug, Serialize)]
struct FacadeFixChangeResponse {
    path: PathBuf,
    wrote: bool,
    created: bool,
}

pub(crate) fn write_summary(
    output: &Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) -> Result<()> {
    if output.is_json() {
        write_json_summary(output, mode, plan, summary)
    } else {
        write_text_summary(output, mode, plan, summary)
    }
}

fn write_json_summary(
    output: &Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) -> Result<()> {
    let response = FacadeFixResponse {
        command: "facade fix",
        status: summary_status(plan, summary),
        root: plan.root.clone(),
        mode: mode.as_str(),
        files_inspected: summary.files_inspected,
        files_changed: plan.files_changed(),
        files_written: summary.files_written(),
        files_created: plan.files_created(),
        files_unchanged: summary.files_unchanged,
        changes: summary
            .changes
            .iter()
            .map(|change| FacadeFixChangeResponse {
                path: change.path.clone(),
                wrote: change.wrote,
                created: change.created,
            })
            .collect(),
    };

    output.json(&response)
}

fn write_text_summary(
    output: &Output,
    mode: FixMode,
    plan: &FacadeFixPlan,
    summary: &FacadeFixSummary,
) -> Result<()> {
    output.line(format!(
        "RustUse facade fix - root: {}",
        plan.root.display()
    ))?;

    output.line(format!("mode: {}", mode.as_str()))?;
    output.line(format!(
        "status: {}",
        summary_status(plan, summary).as_str()
    ))?;
    output.line(format!("files inspected: {}", summary.files_inspected))?;
    output.line(format!("files changed: {}", plan.files_changed()))?;
    output.line(format!("files written: {}", summary.files_written()))?;
    output.line(format!("files created: {}", plan.files_created()))?;
    output.line(format!("files unchanged: {}", summary.files_unchanged))?;

    if plan.is_empty() {
        return Ok(());
    }

    output.line("")?;
    output.line("changed files:")?;

    for change in &summary.changes {
        let action = if change.wrote { "wrote" } else { "would write" };

        let created = if change.created { " (created)" } else { "" };

        output.line(format!("- {action}: {}{created}", change.path.display()))?;
    }

    Ok(())
}

fn summary_status(plan: &FacadeFixPlan, summary: &FacadeFixSummary) -> FacadeFixStatus {
    if !summary.has_changes() {
        return FacadeFixStatus::Ok;
    }

    if summary.files_written() == plan.files_changed() {
        return FacadeFixStatus::Fixed;
    }

    FacadeFixStatus::DryRun
}
