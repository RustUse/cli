//! Shows planned RustUse-managed changes without writing them.

use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct DiffArgs {
    /// Facade repository path to diff.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: DiffArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade diff - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev diff", "unknown", &summary);

    Ok(())
}
