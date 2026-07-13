//! Generates shell completion scripts for the `rustuse` command.

use std::io;

use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};

use crate::cli::Cli;

const BIN_NAME: &str = "rustuse";

#[derive(Clone, Copy, Debug, Args)]
pub(crate) struct CompletionsArgs {
    /// Shell to generate a completion script for.
    #[arg(value_enum)]
    shell: Shell,
}

pub(super) fn run(args: CompletionsArgs) -> Result<()> {
    let mut command = Cli::command();
    let mut stdout = io::stdout().lock();

    generate(args.shell, &mut command, BIN_NAME, &mut stdout);

    Ok(())
}
