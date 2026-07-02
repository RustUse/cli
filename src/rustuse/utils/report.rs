//! Shared Markdown report output helpers for RustUse development commands.

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub(crate) const DEFAULT_REPORT_FILE_NAME: &str = "rustuse-report.md";

#[derive(Debug, Clone)]
pub(crate) struct PresenceCheck {
    pub(crate) path: String,
    pub(crate) present: bool,
}

impl PresenceCheck {
    pub(crate) fn new(path: impl Into<String>, present: bool) -> Self {
        Self {
            path: path.into(),
            present,
        }
    }
}

pub(crate) fn markdown_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

pub(crate) fn write_presence_table(markdown: &mut String, checks: &[PresenceCheck]) {
    markdown.push_str("| Surface | Present |\n");
    markdown.push_str("|---|---:|\n");

    for check in checks {
        markdown.push_str(&format!(
            "| `{}` | {} |\n",
            check.path,
            yes_no(check.present)
        ));
    }

    markdown.push('\n');
}

pub(crate) fn default_report_path(root: &Path) -> PathBuf {
    root.join(DEFAULT_REPORT_FILE_NAME)
}

pub(crate) fn resolve_report_path(root: &Path, output: Option<PathBuf>) -> PathBuf {
    output.unwrap_or_else(|| default_report_path(root))
}

/// Where a generated Markdown report should be delivered.
#[derive(Clone, Debug)]
pub(crate) enum ReportDestination {
    /// Write the report to a file (default path unless a file is provided).
    File(Option<PathBuf>),

    /// Print the report to standard output.
    Stdout,
}

/// Prints a Markdown report to standard output, ensuring a trailing newline.
pub(crate) fn emit_markdown_to_stdout(report: &str) {
    print!("{report}");

    if !report.ends_with('\n') {
        println!();
    }
}

pub(crate) fn write_markdown_report(path: &Path, report: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create `{}`", parent.display()))?;
    }

    let file =
        File::create(path).with_context(|| format!("failed to create `{}`", path.display()))?;
    let mut writer = BufWriter::new(file);

    writer
        .write_all(report.as_bytes())
        .with_context(|| format!("failed to write `{}`", path.display()))?;

    writer
        .flush()
        .with_context(|| format!("failed to flush `{}`", path.display()))?;

    Ok(())
}

pub(crate) fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
