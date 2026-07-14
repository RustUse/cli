use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub(crate) struct RootSummary {
    pub(crate) has_cli: bool,
    pub(crate) has_docs: bool,
    pub(crate) use_dir_count: usize,
    pub(crate) facade_git_count: usize,
    pub(crate) missing_git: Vec<PathBuf>,
}

pub(crate) fn inspect_root(root: &Path) -> Result<RootSummary> {
    let mut summary = RootSummary {
        has_cli: root.join("cli").is_dir(),
        has_docs: root.join("docs").is_dir(),
        ..RootSummary::default()
    };

    for entry in fs::read_dir(root)
        .with_context(|| format!("failed to read root directory `{}`", root.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !name.starts_with("use-") {
            continue;
        }

        summary.use_dir_count += 1;

        if path.join(".git").is_dir() {
            summary.facade_git_count += 1;
        } else {
            summary.missing_git.push(path);
        }
    }

    summary.missing_git.sort();

    Ok(summary)
}

pub(crate) fn display_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
}
