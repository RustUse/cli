/* use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{output::Output, rustuse};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct FacadeArgs {
    #[command(subcommand)]
    pub command: FacadeCommand,
}

#[derive(Debug, Subcommand)]
pub enum FacadeCommand {
    /// Generate a report for one RustUse facade repository.
    Report(FacadeReportArgs),
}

#[derive(Debug, Args)]
pub struct FacadeReportArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,
}

pub fn run(args: FacadeArgs, output: Output) -> Result<()> {
    match args.command {
        FacadeCommand::Report(args) => rustuse::facade::report::run_path(&args.root, output),
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
pub struct FacadeArgs {
    #[command(subcommand)]
    pub command: FacadeCommand,
}

#[derive(Debug, Subcommand)]
pub enum FacadeCommand {
    /// Inspect one facade repository.
    Inspect(FacadePathArgs),

    /// Scan one facade repository.
    Scan(FacadePathArgs),

    /// Check one facade repository.
    Check(FacadeCheckArgs),

    /// Generate a report for one facade repository.
    Report(FacadeReportArgs),

    /// Fix one facade repository.
    Fix(FacadeFixArgs),

    /// Inspect facade Cargo manifests.
    Manifest(FacadeManifestArgs),

    /// Inspect facade CI configuration.
    Ci(FacadePathArgs),

    /// Inspect facade documentation.
    Docs(FacadePathArgs),

    /// Inspect facade release configuration.
    Release(FacadePathArgs),

    /// Inspect facade standard files.
    Standards(FacadePathArgs),
}

#[derive(Debug, Args)]
pub struct FacadePathArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct FacadeCheckArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Fail if warnings are found.
    #[arg(long)]
    pub deny_warnings: bool,
}

#[derive(Debug, Args)]
pub struct FacadeReportArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Optional output file.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct FacadeFixArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Only show intended changes.
    #[arg(long)]
    pub dry_run: bool,

    /// Optional issue code or fix group.
    #[arg(long)]
    pub code: Option<String>,
}

#[derive(Debug, Args)]
pub struct FacadeManifestArgs {
    /// Facade repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Filter by issue code.
    #[arg(long)]
    pub code: Option<String>,

    /// Show only errors.
    #[arg(long)]
    pub errors: bool,

    /// Show only warnings.
    #[arg(long)]
    pub warnings: bool,
}

pub fn run(args: FacadeArgs, output: Output) -> Result<()> {
    match args.command {
        FacadeCommand::Inspect(args) => placeholder(
            output,
            "facade inspect",
            format!("path={}", args.path.display()),
        ),
        FacadeCommand::Scan(args) => placeholder(
            output,
            "facade scan",
            format!("path={}", args.path.display()),
        ),
        FacadeCommand::Check(args) => placeholder(
            output,
            "facade check",
            format!(
                "path={}, deny_warnings={}",
                args.path.display(),
                args.deny_warnings
            ),
        ),
        FacadeCommand::Report(args) => {
            let report_output = args
                .output
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<default>".to_owned());

            placeholder(
                output,
                "facade report",
                format!("path={}, output={}", args.path.display(), report_output),
            )
        },
        FacadeCommand::Fix(args) => placeholder(
            output,
            "facade fix",
            format!(
                "path={}, dry_run={}, code={}",
                args.path.display(),
                args.dry_run,
                args.code.as_deref().unwrap_or("<none>")
            ),
        ),
        FacadeCommand::Manifest(args) => placeholder(
            output,
            "facade manifest",
            format!(
                "path={}, code={}, errors={}, warnings={}",
                args.path.display(),
                args.code.as_deref().unwrap_or("<none>"),
                args.errors,
                args.warnings
            ),
        ),
        FacadeCommand::Ci(args) => {
            placeholder(output, "facade ci", format!("path={}", args.path.display()))
        },
        FacadeCommand::Docs(args) => placeholder(
            output,
            "facade docs",
            format!("path={}", args.path.display()),
        ),
        FacadeCommand::Release(args) => placeholder(
            output,
            "facade release",
            format!("path={}", args.path.display()),
        ),
        FacadeCommand::Standards(args) => placeholder(
            output,
            "facade standards",
            format!("path={}", args.path.display()),
        ),
    }
}
