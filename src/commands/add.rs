//! Adds RustUse crates to a Cargo project.

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{
    output::Output,
    rustuse::adoption::{self, AdoptionMode, AdoptionRequest},
};

use super::NamedCommandArgs;

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,

    /// How the RustUse package should be adopted.
    #[arg(long, value_enum, default_value_t = AddMode::Cargo)]
    pub mode: AddMode,

    /// Show the intended adoption without writing changes.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum AddMode {
    /// Add the published crate as a Cargo dependency.
    Cargo,

    /// Copy the source into the project for the user to own and maintain.
    Own,
}

impl From<AddMode> for AdoptionMode {
    fn from(mode: AddMode) -> Self {
        match mode {
            AddMode::Cargo => Self::Cargo,
            AddMode::Own => Self::Own,
        }
    }
}

pub fn run(args: AddArgs, output: Output) -> Result<()> {
    let root = std::env::current_dir()?;

    let result = adoption::adopt(AdoptionRequest {
        root: &root,
        name: &args.name.name,
        mode: args.mode.into(),
        dry_run: args.dry_run,
    })?;

    let message = format!(
        "name={}, mode={}, dry_run={}",
        result.name,
        result.mode.as_str(),
        result.dry_run
    );

    output.record("add", result.status(), &message);

    Ok(())
}
