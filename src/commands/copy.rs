/* use anyhow::{Result, bail, ensure};
use clap::Args;

use crate::commands::NamedCommandArgs;
use crate::output::Output;
use crate::project;
use crate::rustuse::catalog::DistributionMode;

use super::{entry_for, tests_label};

#[derive(Debug, Args)]
pub struct CopyArgs {
    #[command(flatten)]
    pub target: NamedCommandArgs,

    #[command(flatten)]
    pub options: CopyOptions,

    /// Track the planned copied primitive in rustuse.toml; requires rustuse init first.
    #[arg(long)]
    pub track: bool,
}

#[derive(Debug, Args)]
pub struct CopyOptions {
    /// Include tests when planning copy mode.
    #[arg(long)]
    pub with_tests: bool,
}

pub fn run(args: CopyArgs, output: Output) -> Result<()> {
    let entry = entry_for(&args.target.name)?;
    let state = project::current_state()?;

    ensure!(
        entry.supports_mode(DistributionMode::Copy),
        "`{}` is not currently modeled for copy mode",
        entry.name
    );

    if args.track && !state.has_config {
        bail!("--track requires rustuse.toml. Run `rustuse init` to opt into managed workflows.");
    }

    let tracking = if args.track {
        "rustuse.toml found; the copied primitive would be tracked."
    } else {
        "No RustUse project state is required for copy-only usage."
    };
    let message = format!(
        "Would copy {} source into this project {}. Copy mode: the project owns the copied source. {}",
        entry.name,
        tests_label(args.options.with_tests),
        tracking
    );
    output.record("copy", "dry-run", &message);
    output.detail(format!("Source docs: {}", entry.docs_url));

    Ok(())
}
 */
use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

use super::{NamedCommandArgs, placeholder};

#[derive(Debug, Args)]
pub struct CopyArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,

    /// Destination directory.
    #[arg(long, default_value = ".", value_name = "PATH")]
    pub to: PathBuf,

    /// Overwrite existing files.
    #[arg(long)]
    pub force: bool,
}

pub fn run(args: CopyArgs, output: Output) -> Result<()> {
    placeholder(
        output,
        "copy",
        format!(
            "name={}, to={}, force={}",
            args.name.name,
            args.to.display(),
            args.force
        ),
    )
}
