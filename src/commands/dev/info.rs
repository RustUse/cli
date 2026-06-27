use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

// use crate::cli::DevInfoArgs;
use crate::output::Output;

#[derive(Debug, Args)]
pub struct DevInfoArgs {
    /// Workspace root to inspect.
    #[arg(default_value = ".")]
    pub workspace: PathBuf,
}

pub fn run(args: DevInfoArgs, output: Output) -> Result<()> {
    let workspace = std::fs::canonicalize(&args.workspace).unwrap_or(args.workspace);

    output.line(format!(
        "RustUse dev info - workspace: {}",
        workspace.display()
    ));

    Ok(())
}
