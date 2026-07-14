//! Lists RustUse dependencies, tracked entries, or available catalog entries.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;

use crate::{
    output::Output,
    rustuse::{catalog, config, project},
};

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show all tracked entries.
    #[arg(long)]
    pub all: bool,

    /// Project directory to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub fn run(args: ListArgs, output: Output) -> Result<()> {
    let root = std::fs::canonicalize(&args.path)
        .with_context(|| format!("failed to resolve `{}`", args.path.display()))?;
    let state = project::detect(&root);

    if args.all {
        for entry in catalog::all_entries() {
            print_entry(output, &entry.name, "catalog");
        }
        return Ok(());
    }

    if !state.has_config {
        bail!(
            "no rustuse.toml found in `{}`; run `rustuse init` first",
            root.display()
        );
    }

    let raw = std::fs::read_to_string(&state.config_path)
        .with_context(|| format!("failed to read `{}`", state.config_path.display()))?;
    let rustuse_config = config::from_toml(&raw)?;

    if rustuse_config.primitives.is_empty() {
        output.record("list", "empty", "No tracked RustUse primitives.");
        return Ok(());
    }

    let mut primitives = rustuse_config.primitives;
    primitives.sort_by(|left, right| left.name.cmp(&right.name));
    for primitive in primitives {
        print_entry(output, &primitive.name, primitive.mode.as_str());
    }

    Ok(())
}

fn print_entry(output: Output, name: &str, mode: &str) {
    if output.is_json() {
        output.record("list", "entry", &format!("name={name}, mode={mode}"));
    } else {
        output.line(format!("{name} ({mode})"));
    }
}
