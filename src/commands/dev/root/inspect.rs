use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::DevRootPathArgs;
use crate::output::Output;

pub(crate) fn run(args: DevRootPathArgs, output: Output) -> Result<()> {
    let root = fs::canonicalize(&args.root).unwrap_or(args.root);
    let root_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>");

    let summary = inspect_root(&root)?;

    output.line(format!(
        "RustUse dev root inspect - root: {}",
        root.display()
    ));
    output.line(format!("root.name: {root_name}"));
    output.line(format!(
        "standard root name: {}",
        if root_name == "rustuse" {
            "yes"
        } else {
            "no, expected rustuse"
        }
    ));
    output.line(format!("cli: {}", yes_no(summary.has_cli)));
    output.line(format!("docs: {}", yes_no(summary.has_docs)));
    output.line(format!("use-* directories: {}", summary.use_dir_count));
    output.line(format!("facades using .git: {}", summary.facade_git_count));
    output.line(format!(
        "facades missing .git: {}",
        summary.missing_git.len()
    ));

    if !summary.missing_git.is_empty() {
        output.line("missing .git:");

        for path in &summary.missing_git {
            output.line(format!("- {}", display_name(path)));
        }
    }

    if summary.has_cli && summary.use_dir_count > 0 && summary.missing_git.is_empty() {
        output.line("status: ok");
    } else {
        output.line("status: warning");
    }

    Ok(())
}

#[derive(Debug, Default)]
struct RootSummary {
    has_cli: bool,
    has_docs: bool,
    use_dir_count: usize,
    facade_git_count: usize,
    missing_git: Vec<PathBuf>,
}

fn inspect_root(root: &Path) -> Result<RootSummary> {
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

fn display_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
