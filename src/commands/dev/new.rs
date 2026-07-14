use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct DevNewArgs {
    /// Facade repository path to create.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: DevNewArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade create - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev new", "unknown", &summary);

    Ok(())
}
