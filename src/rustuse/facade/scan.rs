//! Concise scan summary for one RustUse facade repository.
//!
//! Scan is intentionally lighter than `report`: it prints a short health
//! summary to the terminal instead of writing a full Markdown report.

use std::path::Path;

use anyhow::Result;

use crate::output::Output;
use crate::rustuse::facade::discover::discover_facade;
use crate::rustuse::report::markdown::yes_no;

pub(crate) fn scan_facade(root: &Path, output: Output) -> Result<()> {
    let facade = discover_facade(root)?;

    if output.is_json() {
        output.record(
            "scan",
            facade.status(),
            &format!(
                "facade={}, git={}, manifest={}, crates_dir={}, child_crates={}",
                facade.name,
                facade.has_git(),
                facade.has_manifest(),
                facade.has_crates_dir(),
                facade.crate_count()
            ),
        );

        return Ok(());
    }

    output.line(format!("RustUse facade scan - {}", facade.name));
    output.line(format!("- root: {}", facade.root.display()));
    output.line(format!("- git: {}", yes_no(facade.has_git())));
    output.line(format!("- Cargo.toml: {}", yes_no(facade.has_manifest())));
    output.line(format!("- crates/: {}", yes_no(facade.has_crates_dir())));
    output.line(format!("- child crates: {}", facade.crate_count()));
    output.line(format!("- status: {}", facade.status()));

    Ok(())
}
