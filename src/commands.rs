/* //! Command dispatch for the `rustuse` CLI.

pub mod add;
pub mod catalog;
pub mod check;
pub mod ci;
pub mod copy;
pub mod dev;
pub mod docs;
pub mod doctor;
pub mod facade;
pub mod info;
pub mod init;
pub mod list;
pub mod report;
pub mod scan;
pub mod search;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use crate::cli::Cli;
use crate::output::Output;
use crate::rustuse::catalog::{CatalogEntry, find_by_name};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Opt into optional RustUse project tracking with rustuse.toml.
    Init(init::InitArgs),

    /// Search the RustUse crate and primitive index.
    Search(search::SearchArgs),

    /// Show metadata for a RustUse crate or primitive.
    Info(NamedCommandArgs),

    /// Plan adding a Cargo dependency or copy-mode primitive.
    Add(add::AddArgs),

    /// Plan copying RustUse source without requiring RustUse project state.
    Copy(copy::CopyArgs),

    /// Show optional rustuse.toml project tracking state.
    List,

    /// Print RustUse documentation URLs.
    Docs(docs::DocsArgs),

    /// Check this directory for Cargo and RustUse project tracking state.
    Doctor,

    /// Check a RustUse root or facade repository.
    Check(check::CheckArgs),

    /// Generate a RustUse root or facade report.
    Report(report::ReportArgs),

    /// Scan a RustUse root for facade repositories.
    Scan(scan::ScanArgs),

    /// Manage one RustUse facade repository.
    Facade(facade::FacadeArgs),

    /// Run RustUse automation profiles for CI systems.
    Ci(ci::CiArgs),

    /// Development commands for maintaining RustUse repositories.
    Dev(dev::DevArgs),

    /// Manage RustUse catalog including facades and workspace checks.
    Catalog(catalog::CatalogArgs),
}

#[derive(Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate or primitive name.
    #[arg(value_name = "NAME")]
    pub name: String,
}

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
        Commands::Check(args) => check::run(args, output),
        Commands::Report(args) => report::run(args, output),
        Commands::Scan(args) => scan::run(args, output),
        Commands::Facade(args) => facade::run(args, output),
        Commands::Ci(args) => ci::run(args, output),
        Commands::Dev(args) => dev::run(args, output),
        Commands::Catalog(args) => catalog::run(args, output),
    }
}

pub(crate) fn entry_for(name: &str) -> Result<CatalogEntry> {
    find_by_name(name)?
        .with_context(|| format!("unknown RustUse entry `{name}` in the RustUse catalog"))
}

pub(crate) fn tests_label(with_tests: bool) -> &'static str {
    if with_tests {
        "with tests"
    } else {
        "without tests"
    }
}
 */

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cli, output::Output};

pub mod add;
pub mod catalog;
pub mod check;
pub mod ci;
pub mod copy;
pub mod dev;
pub mod docs;
pub mod doctor;
pub mod facade;
pub mod info;
pub mod init;
pub mod list;
pub mod report;
pub mod scan;
pub mod search;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Opt into optional RustUse project tracking with rustuse.toml.
    Init(init::InitArgs),

    /// Search the RustUse crate and primitive catalog.
    Search(search::SearchArgs),

    /// Show metadata for a RustUse crate or primitive.
    Info(info::InfoArgs),

    /// Plan adding a Cargo dependency or copy-mode primitive.
    Add(add::AddArgs),

    /// Plan copying RustUse source without requiring RustUse project state.
    Copy(copy::CopyArgs),

    /// Show optional rustuse.toml project tracking state.
    List(list::ListArgs),

    /// Print RustUse documentation URLs.
    Docs(docs::DocsArgs),

    /// Check this directory for Cargo and RustUse project tracking state.
    Doctor(doctor::DoctorArgs),

    /// Check a RustUse root or facade repository.
    Check(check::CheckArgs),

    /// Generate a RustUse root or facade report.
    Report(report::ReportArgs),

    /// Scan a RustUse root or facade repository.
    Scan(scan::ScanArgs),

    /// Manage one RustUse facade repository.
    Facade(facade::FacadeArgs),

    /// Run RustUse automation profiles for CI systems.
    Ci(ci::CiArgs),

    /// Development commands for maintaining RustUse repositories.
    Dev(dev::DevArgs),

    /// Manage RustUse catalog generation and validation.
    Catalog(catalog::CatalogArgs),
}

#[derive(Clone, Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate, primitive, facade, or package name.
    #[arg(value_name = "NAME")]
    pub name: String,
}

pub fn run(cli: Cli) -> Result<()> {
    let output = Output::new(cli.json, cli.quiet, cli.verbose);

    match cli.command {
        Commands::Init(args) => init::run(args, output),
        Commands::Search(args) => search::run(args, output),
        Commands::Info(args) => info::run(args, output),
        Commands::Add(args) => add::run(args, output),
        Commands::Copy(args) => copy::run(args, output),
        Commands::List(args) => list::run(args, output),
        Commands::Docs(args) => docs::run(args, output),
        Commands::Doctor(args) => doctor::run(args, output),
        Commands::Check(args) => check::run(args, output),
        Commands::Report(args) => report::run(args, output),
        Commands::Scan(args) => scan::run(args, output),
        Commands::Facade(args) => facade::run(args, output),
        Commands::Ci(args) => ci::run(args, output),
        Commands::Dev(args) => dev::run(args, output),
        Commands::Catalog(args) => catalog::run(args, output),
    }
}

pub(crate) fn placeholder(output: Output, command: &str, detail: impl AsRef<str>) -> Result<()> {
    let detail = detail.as_ref();

    if output.is_json() {
        output.record(command, "placeholder", detail);
    } else {
        output.line(format!("rustuse {command}: placeholder"));
        if !detail.is_empty() {
            output.line(format!("detail: {detail}"));
        }
    }

    Ok(())
}
