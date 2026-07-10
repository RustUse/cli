use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cli, output::Output};

pub mod add;
pub mod catalog;
pub mod ci;
pub mod copy;
pub mod dev;
pub mod docs;
pub mod doctor;
pub mod ferris;
pub mod info;
pub mod init;
pub mod interactive;
pub mod list;
pub mod search;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Plan adding a Cargo dependency or copy-mode primitive.
    Add(add::AddArgs),

    /// Manage RustUse catalog generation and validation.
    Catalog(catalog::CatalogArgs),

    /// Run automation-safe RustUse validation.
    Ci(ci::CiArgs),

    /// Plan copying RustUse source without requiring RustUse project state.
    Copy(copy::CopyArgs),

    /// Run RustUse maintainer and repository development tools.
    Dev(dev::DevArgs),

    /// Print RustUse documentation URLs.
    Docs(docs::DocsArgs),

    /// Check this directory for Cargo and RustUse project tracking state.
    Doctor(doctor::DoctorArgs),

    /// Print a friendly greeting from Ferris.
    #[command(visible_alias = "🦀")]
    Ferris(ferris::FerrisArgs),

    /// Show metadata for a RustUse crate or primitive.
    Info(info::InfoArgs),

    /// Opt into optional RustUse project tracking with rustuse.toml.
    Init(init::InitArgs),

    /// Show optional rustuse.toml project tracking state.
    List(list::ListArgs),

    /// Search the RustUse crate and primitive catalog.
    Search(search::SearchArgs),
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
        Some(command) => run_command(command, output, cli.non_interactive, cli.yes),
        None => interactive::run(output, cli.non_interactive, cli.yes),
    }
}

fn run_command(command: Commands, output: Output, non_interactive: bool, yes: bool) -> Result<()> {
    match command {
        Commands::Add(args) => add::run(args, output),
        Commands::Catalog(args) => catalog::run(args, output),
        Commands::Ci(args) => ci::run(args, output),
        Commands::Copy(args) => copy::run(args, output),
        Commands::Dev(args) => dev::run(args, output, non_interactive, yes),
        Commands::Docs(args) => docs::run(args, output),
        Commands::Doctor(args) => doctor::run(args, output),
        Commands::Ferris(args) => ferris::run(args, output),
        Commands::Info(args) => info::run(args, output),
        Commands::Init(args) => init::run(args, output),
        Commands::List(args) => list::run(args, output),
        Commands::Search(args) => search::run(args, output),
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
