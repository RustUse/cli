//! Applies a previously generated facade repair plan.
//!
//! This module performs filesystem mutations only. Planning, manifest
//! transformation, and terminal output belong to other fix submodules.

use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::model::{FacadeFixChange, FacadeFixPlan, FacadeFixSummary, FixMode, PlannedFileChange};

/// Applies a facade repair plan or evaluates it as a dry run.
///
/// The plan stores paths relative to the facade root. Every planned path is
/// validated before use so that repairs cannot write outside the facade
/// repository.
pub(crate) fn apply_plan(plan: &FacadeFixPlan, mode: FixMode) -> Result<FacadeFixSummary> {
    validate_plan(plan)?;

    let mut summary = FacadeFixSummary {
        files_inspected: plan.files_inspected,
        files_changed: plan.changes.len(),
        files_unchanged: plan.files_unchanged,
        files_created: plan.changes.iter().filter(|change| change.created).count(),
        changes: Vec::with_capacity(plan.changes.len()),
    };

    for change in &plan.changes {
        let path = resolve_change_path(&plan.root, &change.path)?;

        let wrote = if mode.writes_files() {
            apply_file_change(&path, change)?;
            true
        } else {
            false
        };

        summary.changes.push(FacadeFixChange {
            path: change.path.clone(),
            created: change.created,
            wrote,
        });
    }

    Ok(summary)
}

fn apply_file_change(destination: &Path, change: &PlannedFileChange) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create repair destination directory `{}`",
                parent.display()
            )
        })?;
    }

    fs::write(destination, &change.contents)
        .with_context(|| format!("failed to write repaired file `{}`", destination.display()))
}

fn validate_plan(plan: &FacadeFixPlan) -> Result<()> {
    if !plan.root.is_dir() {
        bail!(
            "facade repair root `{}` is not a directory",
            plan.root.display()
        );
    }

    for change in &plan.changes {
        validate_relative_path(&change.path)?;

        if change.contents.is_empty() {
            bail!(
                "planned repair for `{}` has empty contents",
                change.path.display()
            );
        }
    }

    Ok(())
}

fn resolve_change_path(root: &Path, relative_path: &Path) -> Result<PathBuf> {
    validate_relative_path(relative_path)?;

    Ok(root.join(relative_path))
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        bail!("planned repair contains an empty path");
    }

    if path.is_absolute() {
        bail!(
            "planned repair path `{}` must be relative to the facade root",
            path.display()
        );
    }

    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {},
            Component::ParentDir => {
                bail!(
                    "planned repair path `{}` cannot contain `..`",
                    path.display()
                );
            },
            Component::RootDir | Component::Prefix(_) => {
                bail!(
                    "planned repair path `{}` must be relative to the facade root",
                    path.display()
                );
            },
        }
    }

    Ok(())
}
