use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;
use crate::rustuse::facade::diagnostics::FacadeDiagnostics;

#[derive(Debug, Args)]
pub struct DevInspectArgs {
    /// Facade repository path to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: DevInspectArgs, output: Output) -> Result<()> {
    let diagnostics = FacadeDiagnostics::inspect(&args.path)?;
    let summary = format!(
        "RustUse facade inspect - root: {}; errors: {}; warnings: {}",
        diagnostics.facade.root.display(),
        diagnostics.error_count(),
        diagnostics.warning_count()
    );

    output.record("dev inspect", diagnostics.status(), &summary);

    Ok(())
}
