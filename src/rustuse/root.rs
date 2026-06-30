//! Root-level development commands for a local RustUse repository collection.

use std::path::PathBuf;

// use anyhow::Result;
use clap::{Args, Subcommand};

// use crate::output::Output;

// pub(crate) mod discover;
pub(crate) mod inspect;
// pub(crate) mod publish;
pub(crate) mod report;
pub(crate) mod scan;
// pub(crate) mod standards;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DevRootArgs {
    #[command(subcommand)]
    pub command: DevRootCommand,
}

#[derive(Debug, Subcommand)]
pub enum DevRootCommand {
    /// Inspect the local RustUse development root.
    Inspect(DevRootPathArgs),

    /// Inspect Cargo manifests in the local RustUse development root.
    Manifests(DevRootManifestArgs),

    /// Generate a report for the local RustUse development root.
    Report(DevRootReportArgs),

    /// Scan local use-* facade directories.
    Scan(DevRootPathArgs),
}

#[derive(Debug, Args)]
pub struct DevRootPathArgs {
    /// Root directory containing use-* facade repositories.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,
}

#[derive(Debug, Args)]
pub struct DevRootManifestArgs {
    /// Root directory containing use-* facade repositories.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,

    /// Only show manifest output for one facade, such as use-js.
    #[arg(long, value_name = "FACADE")]
    pub facade: Option<String>,

    /// Only show issues with this issue code.
    #[arg(long, value_name = "CODE")]
    pub code: Option<String>,

    /// Only show manifests of this kind: workspace-root, facade-package, or child-package.
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,

    /// Only show issues with this severity: error or warning.
    #[arg(long, value_name = "SEVERITY")]
    pub severity: Option<String>,

    /// Apply supported manifest fixes in memory.
    #[arg(long)]
    pub fix: bool,

    /// Write supported manifest fixes to disk.
    #[arg(long)]
    pub write: bool,
}

#[derive(Debug, Args)]
pub struct DevRootReportArgs {
    /// Local RustUse development root containing cli/, docs/, and use-* repos.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,

    /// Write the report to this path instead of rustuse-report.md in the root.
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Print the Markdown report to stdout instead of writing a file.
    #[arg(long)]
    pub stdout: bool,
}

/* pub(crate) fn run(args: DevRootArgs, output: Output) -> Result<()> {
    match args.command {
        DevRootCommand::Inspect(args) => inspect::run(args, output),
        DevRootCommand::Manifests(args) => manifests::run(args, output),
        DevRootCommand::Report(args) => report::run(args, output),
        DevRootCommand::Scan(args) => scan::run(args, output),
        // Example placeholder for handling commands when the modules are commented out. Currently, no commands are executed.
        DevRootCommand::Inspect(_)
        | DevRootCommand::Manifests(_)
        | DevRootCommand::Report(_)
        | DevRootCommand::Scan(_) => Err(anyhow::anyhow!(
            "No commands are executed because the modules are commented out."
        ))?,
    }
} */
