//! Command dispatch for the `rustuse` CLI.

pub mod add;
pub mod copy;
pub mod dev;
pub mod docs;
pub mod doctor;
pub mod info;
pub mod init;
pub mod list;
pub mod search;

use anyhow::{Context, Result};

use crate::cli::{Cli, Commands};
use crate::index::{self, RustUseEntry};
use crate::output::Output;

/// Runs the parsed CLI command.
pub fn run(cli: Cli) -> Result<()> {
    let output = Output::new(cli.json, cli.quiet, cli.verbose);

    match cli.command {
        Commands::Init(args) => init::run(args, output),
        Commands::Search(args) => search::run(args, output),
        Commands::Info(args) => info::run(args, output),
        Commands::Add(args) => add::run(args, output),
        Commands::Copy(args) => copy::run(args, output),
        Commands::List => list::run(output),
        Commands::Docs(args) => docs::run(args, output),
        Commands::Doctor => doctor::run(output),
        Commands::Dev(args) => dev::run(args, output),
    }
}

pub(crate) fn entry_for(name: &str) -> Result<RustUseEntry> {
    index::find_by_name(name)
        .with_context(|| format!("unknown RustUse entry `{name}` in the RustUse index"))
}

pub(crate) fn tests_label(with_tests: bool) -> &'static str {
    if with_tests {
        "with tests"
    } else {
        "without tests"
    }
}
