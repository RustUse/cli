/* use clap::{Args, Parser, Subcommand};

use crate::commands::add::AddArgs;
use crate::commands::copy::CopyArgs;
use crate::commands::dev::DevArgs;
use crate::commands::docs::DocsArgs;
use crate::commands::init::InitArgs;
use crate::commands::search::SearchArgs;

#[derive(Debug, Parser)]
#[command(
    name = "rustuse",
    version,
    about = "RustUse command-line adoption helper.",
    long_about = "rustuse helps find, inspect, and plan RustUse adoption. rustuse.toml is optional: Cargo-only and copy-only workflows do not require project state. Use rustuse init to opt into managed tracking."
)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        help = "Show extra detail about planned actions."
    )]
    pub verbose: bool,

    #[arg(
        short,
        long,
        global = true,
        conflicts_with = "verbose",
        help = "Only print essential output."
    )]
    pub quiet: bool,

    #[arg(
        long,
        global = true,
        help = "Print scaffold-friendly JSON-like output."
    )]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Opt into optional RustUse project tracking with rustuse.toml.
    Init(InitArgs),

    /// Search the placeholder RustUse index.
    Search(SearchArgs),

    /// Show placeholder metadata for a RustUse crate or primitive.
    Info(NamedCommandArgs),

    /// Plan adding a Cargo dependency or a copy-mode primitive without requiring rustuse.toml.
    Add(AddArgs),

    /// Plan copying RustUse source without requiring RustUse project state.
    Copy(CopyArgs),

    /// Show optional rustuse.toml project tracking state.
    List,

    /// Print RustUse docs URLs.
    Docs(DocsArgs),

    /// Check this directory for Cargo and RustUse project tracking state.
    Doctor,

    /// Development commands for RustUse itself.
    Dev(DevArgs),
}

#[derive(Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate or primitive name.
    pub name: String,
}
 */

use clap::{Args, Parser, Subcommand};

use crate::commands::add::AddArgs;
use crate::commands::copy::CopyArgs;
use crate::commands::dev::DevArgs;
use crate::commands::docs::DocsArgs;
use crate::commands::init::InitArgs;
use crate::commands::search::SearchArgs;

const CLI_ABOUT: &str = "RustUse command-line adoption helper.";

const CLI_LONG_ABOUT: &str = "\
rustuse helps find, inspect, and plan RustUse adoption.

rustuse.toml is optional: Cargo-only and copy-only workflows do not require
project state. Use `rustuse init` to opt into managed tracking.";

#[derive(Debug, Parser)]
#[command(
    name = "rustuse",
    version,
    about = CLI_ABOUT,
    long_about = CLI_LONG_ABOUT,
    arg_required_else_help = true,
    propagate_version = true
)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        help = "Show extra detail about planned actions."
    )]
    pub verbose: bool,

    #[arg(
        short,
        long,
        global = true,
        conflicts_with = "verbose",
        help = "Only print essential output."
    )]
    pub quiet: bool,

    #[arg(
        long,
        global = true,
        help = "Print machine-oriented output where supported."
    )]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Opt into optional RustUse project tracking with rustuse.toml.
    Init(InitArgs),

    /// Search the RustUse crate and primitive index.
    Search(SearchArgs),

    /// Show metadata for a RustUse crate or primitive.
    Info(NamedCommandArgs),

    /// Plan adding a Cargo dependency or copy-mode primitive.
    Add(AddArgs),

    /// Plan copying RustUse source without requiring RustUse project state.
    Copy(CopyArgs),

    /// Show optional rustuse.toml project tracking state.
    List,

    /// Print RustUse documentation URLs.
    Docs(DocsArgs),

    /// Check this directory for Cargo and RustUse project tracking state.
    Doctor,

    /// Development commands for maintaining RustUse repositories.
    Dev(DevArgs),
}

#[derive(Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate or primitive name.
    #[arg(value_name = "NAME")]
    pub name: String,
}
