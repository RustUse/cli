use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

use super::placeholder;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CiArgs {
    #[command(subcommand)]
    pub command: CiCommand,
}

#[derive(Debug, Subcommand)]
pub enum CiCommand {
    /// Run default CI checks.
    Check(CiPathArgs),

    /// Inspect GitHub Actions workflows.
    GithubActions(CiPathArgs),

    /// Inspect trusted publishing configuration.
    TrustedPublishing(CiPathArgs),

    /// Generate a CI report.
    Report(CiReportArgs),
}

#[derive(Debug, Args)]
pub struct CiPathArgs {
    /// Repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct CiReportArgs {
    /// Repository root.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Optional output file.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

pub fn run(args: CiArgs, output: Output) -> Result<()> {
    match args.command {
        CiCommand::Check(args) => {
            staged(output, "ci check", format!("path={}", args.path.display()))
        },
        CiCommand::GithubActions(args) => staged(
            output,
            "ci github-actions",
            format!("path={}", args.path.display()),
        ),
        CiCommand::TrustedPublishing(args) => staged(
            output,
            "ci trusted-publishing",
            format!("path={}", args.path.display()),
        ),
        CiCommand::Report(args) => {
            let report_output = args
                .output
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<default>".to_owned());

            staged(
                output,
                "ci report",
                format!("path={}, output={}", args.path.display(), report_output),
            )
        },
    }
}

fn staged(output: Output, command: &str, detail: String) -> Result<()> {
    let staged_detail = format!("staged=true, {detail}");
    placeholder(output, command, staged_detail)
}
