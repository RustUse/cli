use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{Args, Subcommand};

use crate::{
    output::Output,
    rustuse::{
        self,
        facade::{discover::discover_facade, inspect::inspect_facade_repository},
        report::destination::ReportDestination,
    },
};

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
        FacadeCommand::Inspect(args) => run_inspect(&args.path, output),
        FacadeCommand::Scan(args) => rustuse::facade::scan::scan_facade(&args.path, output),
        FacadeCommand::Check(args) => run_check(args, output),
        FacadeCommand::Report(args) => rustuse::facade::report::generate_markdown_report(
            &args.path,
            output,
            ReportDestination::File(args.output),
        ),
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

fn run_inspect(path: &std::path::Path, output: Output) -> Result<()> {
    let facade = discover_facade(path)?;
    let report = inspect_facade_repository(&facade);

    let missing_required_files = report.missing_required_files();
    let missing_required_directories = report.missing_required_directories();
    let status = report.status().as_str();

    if output.is_json() {
        output.record(
            "facade inspect",
            status,
            &format!(
                "path={}, facade={}, missing_required_files={}, missing_required_directories={}",
                facade.root.display(),
                facade.name,
                missing_required_files.len(),
                missing_required_directories.len()
            ),
        );

        return Ok(());
    }

    output.line(format!("RustUse facade inspect - {}", facade.name));
    output.line(format!("- root: {}", facade.root.display()));
    output.line(format!("- status: {status}"));
    output.line(format!(
        "- missing required files: {}",
        missing_required_files.len()
    ));
    output.line(format!(
        "- missing required directories: {}",
        missing_required_directories.len()
    ));

    Ok(())
}

fn run_check(args: FacadeCheckArgs, output: Output) -> Result<()> {
    let facade = discover_facade(&args.path)?;
    let report = inspect_facade_repository(&facade);
    let status = report.status().as_str();

    let missing_required_files = report.missing_required_files();
    let missing_required_directories = report.missing_required_directories();
    let warning_count = missing_required_files.len() + missing_required_directories.len();

    if output.is_json() {
        output.record(
            "facade check",
            status,
            &format!(
                "path={}, facade={}, warnings={}, deny_warnings={}",
                facade.root.display(),
                facade.name,
                warning_count,
                args.deny_warnings
            ),
        );
    } else {
        output.line(format!("RustUse facade check - {}", facade.name));
        output.line(format!("- root: {}", facade.root.display()));
        output.line(format!("- status: {status}"));
        output.line(format!("- warnings: {warning_count}"));
    }

    if args.deny_warnings && warning_count > 0 {
        bail!("facade check failed: found {warning_count} warning(s) and --deny-warnings was set");
    }

    Ok(())
}
