//! Guided interactive menu shown when `rustuse` runs without a subcommand.
//!
//! This layer only collects missing inputs and then delegates to the existing
//! command implementations. It must not contain business logic of its own.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Result, bail};
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::output::Output;

use super::NamedCommandArgs;
use super::add::{self, AddArgs, AddMode};
use super::copy::{self, CopyArgs};
use super::doctor::{self, DoctorArgs};
use super::ferris::{self, FerrisArgs};
use super::info::{self, InfoArgs};
use super::init::{self, InitArgs};
use super::search::{self, SearchArgs};

pub(crate) fn run(output: Output, non_interactive: bool, yes: bool) -> Result<()> {
    if non_interactive {
        bail!("a command is required when running non-interactively; try `rustuse --help`");
    }

    if yes {
        output.detail("--yes selected; running `rustuse doctor .` with safe defaults");

        return doctor::run(
            DoctorArgs {
                path: PathBuf::from("."),
            },
            output,
        );
    }

    if !std::io::stdin().is_terminal() {
        bail!("a command is required when running non-interactively; try `rustuse --help`");
    }

    let choices = [
        "Search RustUse crates",
        "Show crate or primitive info",
        "Add a RustUse crate",
        "Copy RustUse source",
        "Scan a RustUse repository",
        "Generate a report",
        "Initialize RustUse tracking",
        "Run doctor checks",
        "Ask Ferris",
        "Exit",
    ];

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(choices)
        .default(0)
        .interact()?;

    match selected {
        0 => search::run(
            SearchArgs {
                query: prompt_text("Search query")?,
                limit: 20,
            },
            output,
        ),
        1 => info::run(
            InfoArgs {
                name: NamedCommandArgs {
                    name: prompt_text("Crate or primitive name")?,
                },
            },
            output,
        ),
        2 => add::run(
            AddArgs {
                name: NamedCommandArgs {
                    name: prompt_text("Crate name")?,
                },
                mode: AddMode::Cargo,
                dry_run: true,
            },
            output,
        ),
        3 => copy::run(
            CopyArgs {
                name: NamedCommandArgs {
                    name: prompt_text("Crate or primitive name")?,
                },
                to: prompt_path("Destination directory", ".")?,
                force: false,
            },
            output,
        ),
        4 => init::run(
            InitArgs {
                path: prompt_path("Directory to initialize", ".")?,
                copy_first: false,
                cargo_first: false,
                dry_run: true,
                force: false,
            },
            output,
        ),
        5 => doctor::run(
            DoctorArgs {
                path: prompt_path("Directory to inspect", ".")?,
            },
            output,
        ),
        6 => ferris::run(FerrisArgs {}, output),
        _ => Ok(()),
    }
}

fn prompt_text(prompt: &str) -> Result<String> {
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()?;

    Ok(value)
}

fn prompt_path(prompt: &str, default: &str) -> Result<PathBuf> {
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_owned())
        .interact_text()?;

    Ok(PathBuf::from(value))
}
