use anyhow::{Result, bail};
use clap::{Args, Subcommand};

use crate::output::Output;

pub(crate) mod check;
pub(crate) mod facade;
pub(crate) mod info;
pub(crate) mod root;
pub(crate) mod utils;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevArgs {
    #[command(subcommand)]
    pub command: DevCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevCommand {
    /// Check a RustUse facade/workspace for standard project shape.
    Check(check::DevCheckArgs),

    /// Show RustUse development info for a workspace.
    Info(info::DevInfoArgs),

    /// Manage a root of RustUse facades.
    Root(root::DevRootArgs),

    /// Manage one RustUse facade repository.
    Facade(facade::DevFacadeArgs),
}

pub fn run(args: DevArgs, output: Output) -> Result<()> {
    match args.command {
        DevCommand::Check(args) => {
            let options = check::CheckOptions::new(args.workspace);
            let report = check::run(options)?;

            println!("{}", report.to_text());

            if report.is_clean() {
                Ok(())
            } else {
                bail!("RustUse dev check failed")
            }
        },
        DevCommand::Facade(args) => facade::run(args, output),
        DevCommand::Info(args) => info::run(args, output),
        DevCommand::Root(args) => root::run(args, output),
    }
}
