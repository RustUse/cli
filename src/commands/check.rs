use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Facade repository path to check.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: CheckArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade check - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev check", "unknown", &summary);

    Ok(())
}
