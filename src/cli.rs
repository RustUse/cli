use clap::Parser;

use crate::commands::Commands;

const CLI_ABOUT: &str = "RustUse command-line adoption and maintenance helper.";

const CLI_LONG_ABOUT: &str = "\
rustuse helps find, inspect, adopt, validate, and maintain RustUse crates.

rustuse.toml is optional: Cargo-only and copy-only workflows do not require
project state. Use `rustuse init` to opt into managed tracking.

Use `rustuse check` and `rustuse report` for local RustUse repository validation.
Use `rustuse ci` as the stable automation entrypoint for CI systems.
Use `rustuse dev` for maintainer-focused development workflows.";

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
