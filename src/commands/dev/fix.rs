//! CLI adapter for repairing one RustUse facade repository.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{ArgAction, Args};
use dialoguer::{MultiSelect, theme::ColorfulTheme};

use crate::{
    output::Output,
    rustuse::facade::fix::{self, FacadeFixOptions, FixMode},
};

const FIX_CHOICES: &[FixChoice] = &[
    FixChoice {
        code: "facade-wiring",
        label: "Facade wiring",
        description: "optional child dependencies and facade features",
    },
    FixChoice {
        code: "workspace-shape",
        label: "Workspace shape",
        description: "members, resolver, shared metadata, and lints",
    },
    FixChoice {
        code: "workspace-dependencies",
        label: "Workspace dependencies",
        description: "child dependency versions and paths",
    },
    FixChoice {
        code: "package-metadata",
        label: "Package metadata",
        description: "inherited fields, documentation, and package lints",
    },
];

#[derive(Clone, Copy, Debug)]
struct FixChoice {
    code: &'static str,
    label: &'static str,
    description: &'static str,
}

impl FixChoice {
    fn display(self) -> String {
        format!("{} — {}", self.label, self.description)
    }
}

/// Repair supported issues in one RustUse facade repository.
#[derive(Debug, Args)]
pub struct DevFixArgs {
    /// Path to the facade repository.
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Repair every supported rule.
    #[arg(long, conflicts_with = "codes")]
    pub all: bool,

    /// Restrict repairs to a rule or repair group.
    ///
    /// May be supplied multiple times. When neither `--code` nor `--all`
    /// is supplied, an interactive rule selector is shown.
    #[arg(
        long = "code",
        value_name = "CODE",
        action = ArgAction::Append
    )]
    pub codes: Vec<String>,

    /// Write the planned repairs.
    ///
    /// Without this flag, the command performs a dry run.
    #[arg(long)]
    pub write: bool,
}

pub fn run(args: DevFixArgs, output: Output) -> Result<()> {
    let codes = resolve_codes(&args)?;

    let mode = if args.write {
        FixMode::Write
    } else {
        FixMode::DryRun
    };

    let options = codes
        .into_iter()
        .fold(FacadeFixOptions::new(mode), |options, code| {
            options.with_code(code)
        });

    fix::run(&args.path, options, output)?;

    Ok(())
}

fn resolve_codes(args: &DevFixArgs) -> Result<Vec<String>> {
    if args.all {
        return Ok(vec!["all".to_owned()]);
    }

    if !args.codes.is_empty() {
        return validate_codes(&args.codes);
    }

    select_codes_interactively()
}

fn select_codes_interactively() -> Result<Vec<String>> {
    if !std::io::stdin().is_terminal() {
        bail!(
            "`rustuse dev fix` requires an interactive terminal when no \
             rules are specified; use `--all` or one or more `--code` flags"
        );
    }

    let labels = FIX_CHOICES
        .iter()
        .copied()
        .map(FixChoice::display)
        .collect::<Vec<_>>();

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the rules to fix")
        .items(&labels)
        .interact()
        .context("failed to read facade fix selections")?;

    if selected.is_empty() {
        bail!("no facade fix rules were selected");
    }

    Ok(selected
        .into_iter()
        .map(|index| FIX_CHOICES[index].code.to_owned())
        .collect())
}

fn validate_codes(codes: &[String]) -> Result<Vec<String>> {
    let mut validated = Vec::with_capacity(codes.len());

    for code in codes {
        let code = code.trim();

        if code.is_empty() {
            bail!("facade fix code cannot be empty");
        }

        if code == "all" {
            bail!("use `--all` instead of `--code all`");
        }

        if !is_supported_code(code) {
            bail!(
                "unknown facade fix rule or group `{code}`; \
                 supported groups: {}",
                supported_codes()
            );
        }

        if !validated.iter().any(|existing| existing == code) {
            validated.push(code.to_owned());
        }
    }

    Ok(validated)
}

fn is_supported_code(code: &str) -> bool {
    FIX_CHOICES.iter().any(|choice| choice.code == code)
}

fn supported_codes() -> String {
    FIX_CHOICES
        .iter()
        .map(|choice| choice.code)
        .collect::<Vec<_>>()
        .join(", ")
}
