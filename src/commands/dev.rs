/* use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

use super::{check, report};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevArgs {
    #[command(subcommand)]
    pub command: DevCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevCommand {
    /// Legacy alias for `rustuse check`.
    #[command(hide = true)]
    Check(check::CheckArgs),

    /// Legacy alias for `rustuse report`.
    #[command(hide = true)]
    Report(report::ReportArgs),
}

pub fn run(args: DevArgs, output: Output) -> Result<()> {
    match args.command {
        DevCommand::Check(args) => check::run(args, output),
        DevCommand::Report(args) => report::run(args, output),
    }
}
 */

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

use super::placeholder;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevArgs {
    #[command(subcommand)]
    pub command: DevCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevCommand {
    /// Run maintainer-focused development checks.
    Check(DevPathArgs),

    /// Generate a maintainer-focused development report.
    Report(DevReportArgs),

    /// Scan RustUse development repositories.
    Scan(DevPathArgs),

    /// Maintain one RustUse facade repository.
    Facade(DevFacadeArgs),

    /// Maintain a RustUse development root.
    Root(DevRootArgs),
}

#[derive(Debug, Args)]
pub struct DevPathArgs {
    /// Path to operate on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct DevReportArgs {
    /// Path to report on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Optional output file.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevFacadeArgs {
    #[command(subcommand)]
    pub command: DevFacadeCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevFacadeCommand {
    Inspect(DevPathArgs),
    Report(DevReportArgs),
    Check(DevPathArgs),
    Fix(DevPathArgs),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevRootArgs {
    #[command(subcommand)]
    pub command: DevRootCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevRootCommand {
    Inspect(DevPathArgs),
    Report(DevReportArgs),
    Scan(DevPathArgs),
    Check(DevPathArgs),
}

pub fn run(args: DevArgs, output: Output) -> Result<()> {
    match args.command {
        DevCommand::Check(args) => {
            placeholder(output, "dev check", format!("path={}", args.path.display()))
        },
        DevCommand::Report(args) => {
            let report_output = args
                .output
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<default>".to_owned());

            placeholder(
                output,
                "dev report",
                format!("path={}, output={}", args.path.display(), report_output),
            )
        },
        DevCommand::Scan(args) => {
            placeholder(output, "dev scan", format!("path={}", args.path.display()))
        },
        DevCommand::Facade(args) => match args.command {
            DevFacadeCommand::Inspect(args) => placeholder(
                output,
                "dev facade inspect",
                format!("path={}", args.path.display()),
            ),
            DevFacadeCommand::Report(args) => {
                let report_output = args
                    .output
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "<default>".to_owned());

                placeholder(
                    output,
                    "dev facade report",
                    format!("path={}, output={}", args.path.display(), report_output),
                )
            },
            DevFacadeCommand::Check(args) => placeholder(
                output,
                "dev facade check",
                format!("path={}", args.path.display()),
            ),
            DevFacadeCommand::Fix(args) => placeholder(
                output,
                "dev facade fix",
                format!("path={}", args.path.display()),
            ),
        },
        DevCommand::Root(args) => match args.command {
            DevRootCommand::Inspect(args) => placeholder(
                output,
                "dev root inspect",
                format!("path={}", args.path.display()),
            ),
            DevRootCommand::Report(args) => {
                let report_output = args
                    .output
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "<default>".to_owned());

                placeholder(
                    output,
                    "dev root report",
                    format!("path={}, output={}", args.path.display(), report_output),
                )
            },
            DevRootCommand::Scan(args) => placeholder(
                output,
                "dev root scan",
                format!("path={}", args.path.display()),
            ),
            DevRootCommand::Check(args) => placeholder(
                output,
                "dev root check",
                format!("path={}", args.path.display()),
            ),
        },
    }
}
