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
