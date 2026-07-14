use clap::Parser;

use crate::commands::Commands;

/// RustUse command-line adoption and maintenance helper.
///
/// `rustuse` helps find, inspect, adopt, validate, and maintain RustUse crates.
///
/// `rustuse.toml` is optional for Cargo adoption. Use `rustuse init` to opt into
/// managed tracking.
///
/// Use `rustuse dev inspect` and `rustuse dev report` for maintainer workflows.
/// Use `rustuse ci check` as the stable automation entry point for CI systems.
///
/// Run `rustuse` without a subcommand to open the guided interactive menu.
#[derive(Debug, Parser)]
#[command(name = "rustuse", version, propagate_version = true)]
pub(crate) struct Cli {
    #[arg(
        short = 'v',
        long,
        global = true,
        help = "Show additional execution details."
    )]
    pub verbose: bool,

    #[arg(
        short = 'q',
        long,
        global = true,
        conflicts_with = "verbose",
        help = "Only print essential output."
    )]
    pub quiet: bool,

    #[arg(long, global = true, help = "Emit machine-readable JSON output.")]
    pub json: bool,

    #[arg(long, global = true, help = "Accept safe defaults without prompting.")]
    pub yes: bool,

    #[arg(
        long,
        global = true,
        help = "Disable prompts and fail when required input has no default."
    )]
    pub non_interactive: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Cli;

    #[test]
    fn cli_configuration_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn cli_has_expected_about_text() {
        let command = Cli::command();
        let about = command.get_about().map(ToString::to_string);

        assert_eq!(
            about.as_deref(),
            Some("RustUse command-line adoption and maintenance helper")
        );
    }

    #[test]
    fn cli_has_expected_top_level_commands() {
        let command = Cli::command();
        let commands: Vec<_> = command
            .get_subcommands()
            .map(|command| command.get_name())
            .collect();

        assert_eq!(
            commands,
            [
                "add",
                "catalog",
                "check",
                "ci",
                "completions",
                "dev",
                "diff",
                "docs",
                "doctor",
                "ferris",
                "info",
                "init",
                "list",
                "outdated",
                "remove",
                "search",
                "update",
                "upgrade",
            ]
        );
    }
}
