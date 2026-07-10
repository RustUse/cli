//! Repair orchestration for one RustUse facade repository.
//!
//! This module coordinates planning, applying, and reporting facade repairs.
//! File inspection, TOML transformation, filesystem mutation, and output
//! rendering are delegated to focused submodules.

use std::path::Path;

use anyhow::Result;

use crate::output::Output;

mod apply;
mod manifest;
mod model;
mod plan;
mod render;

pub(crate) use model::{FacadeFixOptions, FacadeFixSummary, FixMode};

/// Plans and applies repairs for one RustUse facade repository.
///
/// `FixMode::DryRun` calculates and reports changes without writing files.
/// `FixMode::Write` applies the planned changes to the repository.
pub(crate) fn run(
    root: &Path,
    options: FacadeFixOptions,
    output: Output,
) -> Result<FacadeFixSummary> {
    let fix_plan = plan::build_plan(root, &options)?;
    let summary = apply::apply_plan(&fix_plan, options.mode)?;

    render::write_summary(output, options.mode, &fix_plan, &summary);

    Ok(summary)
}
