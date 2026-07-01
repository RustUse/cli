use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::output::Output;

use super::{NamedCommandArgs, placeholder};

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,

    /// Adoption mode.
    #[arg(long, value_enum, default_value_t = AddMode::Cargo)]
    pub mode: AddMode,

    /// Do not write changes; only show intent.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum AddMode {
    Cargo,
    Copy,
}

pub fn run(args: AddArgs, output: Output) -> Result<()> {
    placeholder(
        output,
        "add",
        format!(
            "name={}, mode={:?}, dry_run={}",
            args.name.name, args.mode, args.dry_run
        ),
    )
}
