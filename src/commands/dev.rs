use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

pub mod inspect;
pub mod interactive;
pub mod report;

#[derive(Debug, Args)]
pub struct DevArgs {
    #[command(subcommand)]
    pub command: Option<DevCommands>,
}

#[derive(Debug, Subcommand)]
pub enum DevCommands {
    /// Inspect one RustUse facade repository.
    Inspect(inspect::DevInspectArgs),

    /// Generate a RustUse development report.
    Report(report::DevReportArgs),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct DevCommandContext {
    pub non_interactive: bool,
    pub yes: bool,
}

pub fn run(args: DevArgs, output: Output, non_interactive: bool, yes: bool) -> Result<()> {
    let context = DevCommandContext {
        non_interactive,
        yes,
    };

    match args.command {
        Some(command) => run_command(command, output, context),
        None => interactive::run(output, context),
    }
}

fn run_command(command: DevCommands, output: Output, context: DevCommandContext) -> Result<()> {
    match command {
        DevCommands::Inspect(args) => inspect::run(args, output),
        DevCommands::Report(args) => report::run(args, output, context),
    }
}
