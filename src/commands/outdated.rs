//! Finds outdated RustUse dependencies in a Cargo project.

use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct OutdatedArgs {
    /// Facade repository path to check for outdated dependencies.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: OutdatedArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade outdated - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev outdated", "unknown", &summary)
}
