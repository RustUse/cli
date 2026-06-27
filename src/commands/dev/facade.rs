//! Facade-level development commands for one RustUse facade repository.

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

pub(crate) mod discover;
pub(crate) mod inspect;
pub(crate) mod report;
pub(crate) mod run;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevFacadeArgs {
    #[command(subcommand)]
    pub command: DevFacadeCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevFacadeCommand {
    /// Run facade-level development checks.
    Run(run::DevFacadeRunArgs),

    /// Generate a report for one RustUse facade repository.
    Report(report::DevFacadeReportArgs),
}

pub(crate) fn run(args: DevFacadeArgs, output: Output) -> Result<()> {
    match args.command {
        DevFacadeCommand::Run(args) => run::run(args, output),
        DevFacadeCommand::Report(args) => report::run(args, output),
    }
}
