use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

pub(crate) fn add_cargo_dependency(root: &Path, name: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }

    let status = Command::new("cargo")
        .args(["add", name])
        .current_dir(root)
        .status()
        .with_context(|| format!("failed to run `cargo add {name}`"))?;

    if !status.success() {
        bail!("`cargo add {name}` failed with status {status}");
    }

    Ok(())
}
