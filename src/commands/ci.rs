use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

pub mod check;

#[derive(Debug, Args)]
pub struct CiArgs {
    #[command(subcommand)]
    pub command: CiCommand,
}

#[derive(Debug, Subcommand)]
pub enum CiCommand {
    /// Check a RustUse root, facade, or repository for rule violations.
    Check(check::CheckCiArgs),
}

pub fn run(args: CiArgs, output: Output) -> Result<()> {
    match args.command {
        CiCommand::Check(args) => check::run(args, output),
    }
}
