//! Shared Markdown report output helpers for `RustUse` development commands.

use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{BufWriter, Write as IoWrite};
use std::path::Path;

use anyhow::{Context, Result};

use crate::rustuse::report::destination::PresenceCheck;

pub(crate) fn write_presence_table(markdown: &mut String, checks: &[PresenceCheck]) {
    markdown.push_str("| Surface | Present |\n");
    markdown.push_str("|---|---:|\n");

    for check in checks {
        let _ = writeln!(markdown, "| `{}` | {} |", check.path, yes_no(check.present));
    }

    markdown.push('\n');
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

#[must_use]
pub(crate) const fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
