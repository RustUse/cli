use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct FerrisArgs {}

pub fn run(_args: FerrisArgs, output: Output) -> Result<()> {
    output.line("🦀 Hello from Ferris! Welcome to RustUse.");

    Ok(())
}
