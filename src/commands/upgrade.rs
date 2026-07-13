use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct UpgradeArgs {
    /// Facade repository path to upgrade.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub(crate) fn run(args: UpgradeArgs, output: Output) -> Result<()> {
    let summary = format!(
        "RustUse facade upgrade - root: {}; errors: {}; warnings: {}",
        args.path.display(),
        1,
        1
    );

    output.record("dev upgrade", "unknown", &summary);

    Ok(())
}
