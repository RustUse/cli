use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct DevSetupArgs {
    /// Facade repository path to setup.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: DevSetupArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade setup - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev setup", "unknown", &summary)
}
