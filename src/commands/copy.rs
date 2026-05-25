use anyhow::{bail, ensure, Result};

use crate::cli::CopyArgs;
use crate::index::DistributionMode;
use crate::output::Output;
use crate::project;

use super::{entry_for, tests_label};

pub fn run(args: CopyArgs, output: Output) -> Result<()> {
    let entry = entry_for(&args.target.name)?;
    let state = project::current_state()?;

    ensure!(
        entry.supports_mode(DistributionMode::Copy),
        "`{}` is not currently modeled for copy mode",
        entry.name
    );

    if args.track && !state.has_config {
        bail!("--track requires rustuse.toml. Run `rustuse init` to opt into managed workflows.");
    }

    let tracking = if args.track {
        "rustuse.toml found; the copied primitive would be tracked."
    } else {
        "No RustUse project state is required for copy-only usage."
    };
    let message = format!(
        "Would copy {} source into this project {}. Copy mode: the project owns the copied source. {}",
        entry.name,
        tests_label(args.options.with_tests),
        tracking
    );
    output.record("copy", "dry-run", &message);
    output.detail(format!("Source docs: {}", entry.docs_url));

    Ok(())
}
