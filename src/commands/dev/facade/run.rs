//! Facade-level development check runner.

use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use super::discover::{FacadeInfo, discover_facade};
use crate::output::Output;

#[derive(Debug, Args)]
pub struct DevFacadeRunArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,
}

pub(crate) fn run(args: DevFacadeRunArgs, output: Output) -> Result<()> {
    let facade = discover_facade(&args.root)?;

    print_facade_summary(&facade, output);

    Ok(())
}

fn print_facade_summary(facade: &FacadeInfo, output: Output) {
    output.line(format!(
        "RustUse dev facade run - root: {}",
        facade.root.display()
    ));
    output.line(format!("facade: {}", facade.name));
    output.line(format!("git: {}", yes_no(facade.has_git())));
    output.line(format!("Cargo.toml: {}", yes_no(facade.has_manifest())));
    output.line(format!("crates/: {}", yes_no(facade.has_crates_dir())));
    output.line(format!("crate manifests: {}", facade.crate_count()));
    output.line(format!("status: {}", facade.status()));
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
