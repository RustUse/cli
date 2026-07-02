//! Concise scan summary for a local RustUse development root.
//!
//! Scan aggregates the discovered `use-*` facade repositories and prints a
//! short summary. Full Markdown output belongs to `report`.

use std::path::Path;

use anyhow::Result;

use crate::output::Output;
use crate::rustuse::facade::discover::{discover_facades, display_version};

pub(crate) fn scan_root(root: &Path, output: Output) -> Result<()> {
    let facades = discover_facades(root)?;

    if output.is_json() {
        output.record(
            "scan",
            "ok",
            &format!("root={}, facades={}", root.display(), facades.len()),
        );

        for facade in &facades {
            output.record(
                "scan",
                facade.status(),
                &format!(
                    "facade={}, version={}, child_crates={}",
                    facade.name,
                    display_version(&facade.version),
                    facade.child_crate_count()
                ),
            );
        }

        return Ok(());
    }

    output.line(format!(
        "RustUse development root scan - root: {}",
        root.display()
    ));
    output.line(format!("- facades: {}", facades.len()));

    for facade in &facades {
        output.line(format!(
            "- {} (version: {}, child crates: {}, status: {})",
            facade.name,
            display_version(&facade.version),
            facade.child_crate_count(),
            facade.status()
        ));
    }

    Ok(())
}
