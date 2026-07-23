//! CLI adapter for repairing one RustUse facade repository.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{ArgAction, Args};
use dialoguer::{MultiSelect, theme::ColorfulTheme};

use crate::{
    output::Output,
    rustuse::facade::fix::{self, FacadeFixGroup, FacadeFixOptions, FacadeFixTarget, FixMode},
};

const FIX_CHOICES: &[FixChoice] = &[
    FixChoice {
        group: FacadeFixGroup::FacadeWiring,
        label: "Facade wiring",
        description: "optional child dependencies and facade features",
    },
    FixChoice {
        group: FacadeFixGroup::WorkspaceShape,
        label: "Workspace shape",
        description: "members, resolver, shared metadata, and lints",
    },
    FixChoice {
        group: FacadeFixGroup::WorkspaceDependencies,
        label: "Workspace dependencies",
        description: "child dependency versions and paths",
    },
    FixChoice {
        group: FacadeFixGroup::PackageMetadata,
        label: "Package metadata",
        description: "inherited fields, documentation, and package lints",
    },
];

#[derive(Clone, Copy, Debug)]
struct FixChoice {
    group: FacadeFixGroup,
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

    /// Restrict repairs to an issue code or repair group.
    ///
    /// May be supplied multiple times. When neither `--code` nor `--all`
    /// is supplied, an interactive repair-group selector is shown.
    #[arg(
        long = "code",
        value_name = "CODE",
        action = ArgAction::Append
    )]
    pub codes: Vec<FacadeFixTarget>,

    /// Write the planned repairs.
    ///
    /// Without this flag, the command performs a dry run.
    #[arg(long)]
    pub write: bool,
}

pub fn run(args: DevFixArgs, output: Output) -> Result<()> {
    let targets = resolve_targets(&args)?;

    let mode = if args.write {
        FixMode::Write
    } else {
        FixMode::DryRun
    };

    let options = targets
        .into_iter()
        .fold(FacadeFixOptions::new(mode), |options, target| {
            options.with_target(target)
        });

    fix::run(&args.path, options, output)?;

    Ok(())
}

fn resolve_targets(args: &DevFixArgs) -> Result<Vec<FacadeFixTarget>> {
    if args.all {
        return Ok(Vec::new());
    }

    if !args.codes.is_empty() {
        return Ok(deduplicate_targets(&args.codes));
    }

    select_targets_interactively()
}

fn deduplicate_targets(targets: &[FacadeFixTarget]) -> Vec<FacadeFixTarget> {
    let mut unique = Vec::with_capacity(targets.len());

    for &target in targets {
        if !unique.contains(&target) {
            unique.push(target);
        }
    }

    unique
}

pub(crate) fn select_targets_interactively() -> Result<Vec<FacadeFixTarget>> {
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

    selected
        .into_iter()
        .map(|index| {
            let choice = FIX_CHOICES
                .get(index)
                .context("interactive facade fix selection was out of bounds")?;

            Ok(FacadeFixTarget::Group(choice.group))
        })
        .collect()
}
