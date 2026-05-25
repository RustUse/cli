use anyhow::{ensure, Result};

use crate::cli::AddArgs;
use crate::index::DistributionMode;
use crate::output::Output;
use crate::project;

use super::{entry_for, tests_label};

pub fn run(args: AddArgs, output: Output) -> Result<()> {
    let entry = entry_for(&args.target.name)?;

    if args.copy {
        ensure!(
            entry.supports_mode(DistributionMode::Copy),
            "`{}` is not currently modeled for copy mode",
            entry.name
        );

        let message = format!(
            "Would copy {} source into this project {}. Copy mode: the project owns the copied source. {}",
            entry.name,
            tests_label(args.copy_options.with_tests),
            copy_tracking_message()?
        );
        output.record("add", "dry-run", &message);
        output.detail(format!("Source docs: {}", entry.docs_url));
        return Ok(());
    }

    ensure!(
        entry.supports_mode(DistributionMode::Cargo),
        "`{}` is not currently modeled for Cargo mode",
        entry.name
    );

    let message = format!(
        "Would add {} as a Cargo dependency. Cargo mode: RustUse owns the crate; this project depends on it.",
        entry.name
    );
    output.record("add", "dry-run", &message);

    if args.copy_options.with_tests {
        output
            .detail("--with-tests is only meaningful for copy mode; Cargo mode uses crate tests.");
    }

    Ok(())
}

fn copy_tracking_message() -> Result<&'static str> {
    let state = project::current_state()?;
    if state.has_config {
        Ok("rustuse.toml found; the copied primitive would be tracked.")
    } else {
        Ok(
            "No rustuse.toml found; this would be copied without tracking. Run `rustuse init` for managed workflows.",
        )
    }
}
