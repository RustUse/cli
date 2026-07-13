use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Facade repository path to remove.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: RemoveArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade remove - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev remove", "unknown", &summary);

    Ok(())
}
