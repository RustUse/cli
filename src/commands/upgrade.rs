//! Parses arguments for upgrading the installed RustUse CLI.

use anyhow::Result;
use clap::Args;

use crate::output::Output;
use crate::rustuse::upgrade::{self, UpgradeOptions, UpgradeStatus};

/// Arguments for upgrading the installed RustUse CLI.
#[derive(Clone, Copy, Debug, Args)]
pub struct UpgradeArgs {
    /// Show the planned Cargo command without running it.
    #[arg(long)]
    pub dry_run: bool,
}

/// Adapts CLI arguments into the RustUse CLI upgrade workflow.
pub(crate) fn run(args: UpgradeArgs, output: Output) -> Result<()> {
    let outcome = upgrade::run(UpgradeOptions {
        dry_run: args.dry_run,
    })?;

    match outcome.status {
        UpgradeStatus::Planned => output.record(
            "upgrade",
            "planned",
            &format!("would run `{}`", outcome.command),
        ),
        UpgradeStatus::Completed => {
            output.record("upgrade", "ok", "RustUse CLI upgraded successfully")
        },
    }

    Ok(())
}
