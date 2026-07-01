use anyhow::Result;
use clap::Args;

use crate::output::Output;

use super::placeholder;

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show all tracked entries.
    #[arg(long)]
    pub all: bool,
}

pub fn run(args: ListArgs, output: Output) -> Result<()> {
    placeholder(output, "list", format!("all={}", args.all))
}
