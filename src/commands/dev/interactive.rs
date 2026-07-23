//! Guided maintainer menu shown when `rustuse dev` runs without a subcommand.
//!
//! This layer collects missing inputs and delegates to dev command
//! implementations.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Result, bail};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

use crate::output::Output;

use super::DevCommandContext;
use super::fix::{self, DevFixArgs};
use super::inspect::{self, DevInspectArgs};
use super::report::{self, DevReportArgs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DevMenuAction {
    Report,
    Inspect,
    Fix,
    Exit,
}

impl DevMenuAction {
    const ALL: [Self; 4] = [Self::Report, Self::Inspect, Self::Fix, Self::Exit];

    const fn label(self) -> &'static str {
        match self {
            Self::Report => "Generate a RustUse development report",
            Self::Inspect => "Inspect a RustUse facade",
            Self::Fix => "Fix a RustUse facade",
            Self::Exit => "Exit",
        }
    }
}

pub(crate) fn run(output: Output, context: DevCommandContext) -> Result<()> {
    validate_interactive_mode(
        context.non_interactive,
        context.yes,
        output.is_json(),
        std::io::stdin().is_terminal(),
    )?;

    let action = prompt_dev_menu_action()?;
    run_action(action, output, context)
}

fn validate_interactive_mode(
    non_interactive: bool,
    yes: bool,
    json: bool,
    stdin_is_terminal: bool,
) -> Result<()> {
    if non_interactive {
        bail!(
            "a dev subcommand is required when running non-interactively; \
             try `rustuse dev --help`"
        );
    }

    if yes {
        bail!("a dev subcommand is required when using `--yes`; try `rustuse dev --help`");
    }

    if json {
        bail!("a dev subcommand is required when using `--json`; try `rustuse dev --help`");
    }

    if !stdin_is_terminal {
        bail!(
            "a dev subcommand is required when running non-interactively; \
             try `rustuse dev --help`"
        );
    }

    Ok(())
}

fn prompt_dev_menu_action() -> Result<DevMenuAction> {
    let actions = DevMenuAction::ALL;
    let labels = actions.map(DevMenuAction::label);
    let selected = prompt_select("What RustUse dev task do you want to run?", &labels, 0)?;

    Ok(actions[selected])
}

fn run_action(action: DevMenuAction, output: Output, context: DevCommandContext) -> Result<()> {
    match action {
        DevMenuAction::Report => run_report(output, context),
        DevMenuAction::Inspect => run_inspect(output),
        DevMenuAction::Fix => run_fix(output),
        DevMenuAction::Exit => Ok(()),
    }
}

fn run_report(output: Output, context: DevCommandContext) -> Result<()> {
    report::run(
        DevReportArgs {
            path: prompt_path("Path to report", ".")?,
            fleet: prompt_confirm("Generate a fleet report?", false)?,
        },
        output,
        context,
    )
}

fn run_inspect(output: Output) -> Result<()> {
    inspect::run(
        DevInspectArgs {
            path: prompt_path("Path to inspect", ".")?,
        },
        output,
    )
}

fn run_fix(output: Output) -> Result<()> {
    let path = prompt_path("Path to fix", ".")?;
    let targets = fix::select_targets_interactively()?;
    let write = prompt_confirm("Write changes?", false)?;

    fix::run(
        DevFixArgs {
            path,
            all: false,
            codes: targets,
            write,
        },
        output,
    )
}

fn prompt_path(prompt: &str, default: &str) -> Result<PathBuf> {
    let value = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_owned())
        .interact_text()?;

    Ok(PathBuf::from(value))
}

fn prompt_confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

fn prompt_select(prompt: &str, items: &[&str], default: usize) -> Result<usize> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{DevMenuAction, validate_interactive_mode};

    #[test]
    fn dev_menu_labels_are_non_empty_and_unique() {
        let labels = DevMenuAction::ALL.map(DevMenuAction::label);
        assert_labels_are_valid(&labels);
    }

    #[test]
    fn navigation_actions_are_last() {
        assert_eq!(
            DevMenuAction::ALL.last().copied(),
            Some(DevMenuAction::Exit)
        );
    }

    #[test]
    fn interactive_mode_accepts_a_terminal_session() {
        assert!(validate_interactive_mode(false, false, false, true).is_ok());
    }

    #[test]
    fn interactive_mode_rejects_non_interactive_execution() {
        assert!(validate_interactive_mode(true, false, false, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_yes_without_a_subcommand() {
        assert!(validate_interactive_mode(false, true, false, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_json_without_a_subcommand() {
        assert!(validate_interactive_mode(false, false, true, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_non_terminal_input() {
        assert!(validate_interactive_mode(false, false, false, false).is_err());
    }

    fn assert_labels_are_valid(labels: &[&str]) {
        assert!(labels.iter().all(|label| !label.trim().is_empty()));

        let unique: HashSet<_> = labels.iter().copied().collect();
        assert_eq!(unique.len(), labels.len());
    }
}
