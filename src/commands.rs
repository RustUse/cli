//! Defines the public RustUse command surface and routes parsed commands to
//! their command adapters.

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cli, output::Output};

pub mod add;
pub mod catalog;
pub mod check;
pub mod ci;
pub mod completions;
pub mod dev;
pub mod diff;
pub mod docs;
pub mod doctor;
pub mod ferris;
pub mod info;
pub mod init;
pub mod interactive;
pub mod list;
pub mod outdated;
pub mod remove;
pub mod search;
pub mod update;
pub mod upgrade;

/// Top-level commands accepted by the `rustuse` executable.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a RustUse crate as a Cargo dependency.
    Add(add::AddArgs),

    /// Discover, inspect, generate, search, and validate the RustUse catalog.
    Catalog(catalog::CatalogArgs),

    /// Check a Cargo project and its optional RustUse tracking state.
    Check(check::CheckArgs),

    /// Run stable, non-interactive RustUse validation for CI systems.
    Ci(ci::CiArgs),

    /// Generate a shell completion script for `rustuse`.
    Completions(completions::CompletionsArgs),

    /// Run RustUse repository maintenance and development workflows.
    Dev(dev::DevArgs),

    /// Compare RustUse crate versions or a facade against its catalog definition.
    Diff(diff::DiffArgs),

    /// Print URLs for RustUse website, API, or workspace documentation.
    Docs(docs::DocsArgs),

    /// Diagnose RustUse CLI, Cargo, and project configuration problems.
    Doctor(doctor::DoctorArgs),

    /// Print a friendly greeting from Ferris.
    #[command(visible_alias = "🦀")]
    Ferris(ferris::FerrisArgs),

    /// Show catalog metadata for a RustUse crate or primitive.
    Info(info::InfoArgs),

    /// Initialize optional RustUse project tracking with `rustuse.toml`.
    Init(init::InitArgs),

    /// Show the current project's optional `rustuse.toml` tracking state.
    List(list::ListArgs),

    /// Find outdated RustUse crate and primitive dependencies.
    Outdated(outdated::OutdatedArgs),

    /// Remove a RustUse crate from a Cargo project.
    Remove(remove::RemoveArgs),

    /// Search the RustUse catalog for crates and primitives.
    Search(search::SearchArgs),

    /// Update RustUse Cargo dependencies to their latest compatible versions.
    Update(update::UpdateArgs),

    /// Upgrade the installed RustUse CLI to the latest available release.
    Upgrade(upgrade::UpgradeArgs),
}

/// Shared positional argument containing a named RustUse item.
#[derive(Clone, Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate, primitive, facade, or package name.
    #[arg(value_name = "NAME")]
    pub name: String,
}

/// Routes parsed CLI arguments to a command or the guided interactive menu.
pub fn run(cli: Cli) -> Result<()> {
    let output = Output::new(cli.json, cli.quiet, cli.verbose);

    match cli.command {
        Some(command) => run_command(command, output, cli.non_interactive, cli.yes),
        None => interactive::run(output, cli.non_interactive, cli.yes),
    }
}

/// Dispatches a selected top-level command to its command adapter.
fn run_command(command: Commands, output: Output, non_interactive: bool, yes: bool) -> Result<()> {
    match command {
        Commands::Add(args) => add::run(args, output),
        Commands::Catalog(args) => catalog::run(args, output),
        Commands::Check(args) => check::run(args, output),
        Commands::Ci(args) => ci::run(args, output),
        Commands::Completions(args) => completions::run(args),
        Commands::Dev(args) => dev::run(args, output, non_interactive, yes),
        Commands::Diff(args) => diff::run(args, output),
        Commands::Docs(args) => docs::run(args, output),
        Commands::Doctor(args) => doctor::run(args, output),
        Commands::Ferris(args) => ferris::run(args, output),
        Commands::Info(args) => info::run(args, output),
        Commands::Init(args) => init::run(args, output),
        Commands::List(args) => list::run(args, output),
        Commands::Outdated(args) => outdated::run(args, output),
        Commands::Remove(args) => remove::run(args, output),
        Commands::Search(args) => search::run(args, output),
        Commands::Update(args) => update::run(args, output),
        Commands::Upgrade(args) => upgrade::run(args, output),
    }
}
