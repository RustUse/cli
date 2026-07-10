use clap::Parser;

use crate::commands::Commands;

const CLI_ABOUT: &str = "RustUse command-line adoption and maintenance helper.";

const CLI_LONG_ABOUT: &str = "\
RustUse command-line adoption and maintenance helper.

rustuse helps find, inspect, adopt, validate, and maintain RustUse crates.

rustuse.toml is optional for Cargo adoption. Use `rustuse init` to opt into
managed tracking.

Use `rustuse dev inspect` and `rustuse dev report` for maintainer workflows.
Use `rustuse ci check` as the stable automation entrypoint for CI systems.

Run `rustuse` without a subcommand to open the guided interactive menu.";

#[derive(Debug, Parser)]
#[command(
    name = "rustuse",
    version,
    about = CLI_ABOUT,
    long_about = CLI_LONG_ABOUT,
    propagate_version = true
)]
pub(crate) struct Cli {
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

    #[arg(
        long,
        global = true,
        help = "Accept safe defaults without prompting where supported."
    )]
    pub yes: bool,

    #[arg(
        long,
        global = true,
        help = "Disable interactive prompts and fail when required input is missing."
    )]
    pub non_interactive: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
