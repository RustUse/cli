use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct DevCheckArgs {
    /// Facade repository path to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: DevCheckArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade inspect - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev inspect", "unknown", &summary);

    Ok(())
}
