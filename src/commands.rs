use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cli, output::Output};

pub mod add;
pub mod catalog;
pub mod check;
pub mod ci;
pub mod copy;
pub mod docs;
pub mod doctor;
pub mod facade;
pub mod ferris;
pub mod info;
pub mod init;
pub mod interactive;
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

    /// Manage RustUse catalog generation and validation.
    Catalog(catalog::CatalogArgs),

    /// Print a friendly greeting from Ferris.
    #[command(visible_alias = "🦀")]
    Ferris(ferris::FerrisArgs),
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
        Some(command) => run_command(command, output),
        None => interactive::run(output, cli.non_interactive, cli.yes),
    }
}

fn run_command(command: Commands, output: Output) -> Result<()> {
    match command {
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
        Commands::Catalog(args) => catalog::run(args, output),
        Commands::Ferris(args) => ferris::run(args, output),
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
