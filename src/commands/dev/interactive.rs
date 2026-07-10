//! Guided maintainer menu shown when `rustuse dev` runs without a subcommand.
//!
//! This layer only collects missing inputs and delegates to dev command
//! implementations. It must not contain business logic.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Result, bail};
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::output::Output;

use super::DevCommandContext;
use super::report::{self, DevReportArgs};

pub(crate) fn run(output: Output, context: DevCommandContext) -> Result<()> {
    if context.non_interactive {
        bail!(
            "a dev subcommand is required when running non-interactively; try `rustuse dev --help`"
        );
    }

    if context.yes {
        output.detail("--yes selected; running `rustuse dev report .` with safe defaults");

        return report::run(
            DevReportArgs {
                path: PathBuf::from("."),
                fleet: false,
            },
            output,
            context,
        );
    }

    if !std::io::stdin().is_terminal() {
        bail!(
            "a dev subcommand is required when running non-interactively; try `rustuse dev --help`"
        );
    }

    let choices = ["Generate RustUse development report", "Exit"];

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What RustUse dev task do you want to run?")
        .items(choices)
        .default(0)
        .interact()?;

    match selected {
        0 => report::run(
            DevReportArgs {
                path: prompt_path("Path to report", ".")?,
                fleet: false,
            },
            output,
            context,
        ),
        _ => Ok(()),
    }
}

fn prompt_path(prompt: &str, default: &str) -> Result<PathBuf> {
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_owned())
        .interact_text()?;

    Ok(PathBuf::from(value))
}
