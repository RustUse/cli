use clap::{Args, Parser, Subcommand};

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
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefer copy-mode defaults in rustuse.toml.
    #[arg(long)]
    pub copy_first: bool,

    /// Prefer Cargo-mode defaults in rustuse.toml.
    #[arg(long)]
    pub cargo_first: bool,

    /// Accept the v0.1 defaults without prompting.
    #[arg(long)]
    pub yes: bool,

    /// Show what would be created without writing files.
    #[arg(long)]
    pub dry_run: bool,

    /// Override the configured copy root.
    #[arg(long, value_name = "PATH")]
    pub copy_root: Option<String>,

    /// Override the configured test root.
    #[arg(long, value_name = "PATH")]
    pub test_root: Option<String>,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search text, such as geometry or use-slug.
    pub query: String,
}

#[derive(Debug, Args)]
pub struct NamedCommandArgs {
    /// RustUse crate or primitive name.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CopyOptions {
    /// Include tests when planning copy mode.
    #[arg(long)]
    pub with_tests: bool,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(flatten)]
    pub target: NamedCommandArgs,

    /// Plan copy mode instead of Cargo dependency mode.
    #[arg(long)]
    pub copy: bool,

    #[command(flatten)]
    pub copy_options: CopyOptions,
}

#[derive(Debug, Args)]
pub struct CopyArgs {
    #[command(flatten)]
    pub target: NamedCommandArgs,

    #[command(flatten)]
    pub options: CopyOptions,

    /// Track the planned copied primitive in rustuse.toml; requires rustuse init first.
    #[arg(long)]
    pub track: bool,
}

#[derive(Debug, Args)]
pub struct DocsArgs {
    #[command(flatten)]
    pub target: NamedCommandArgs,

    /// Print API RustDocs URL.
    #[arg(long, conflicts_with = "workspace")]
    pub api: bool,

    /// Print workspace RustDocs URL.
    #[arg(long, conflicts_with = "api")]
    pub workspace: bool,
}
