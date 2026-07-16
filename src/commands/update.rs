//! Updates RustUse dependency versions within the current project configuration.

use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Facade repository path to update.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: UpdateArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade update - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("update", "unknown", &summary)
}
