use anyhow::Result;
use clap::Args;

use crate::output::Output;

use super::{NamedCommandArgs, placeholder};

#[derive(Debug, Args)]
pub struct DocsArgs {
    /// Optional RustUse crate, primitive, or facade name.
    #[command(flatten)]
    pub name: Option<NamedCommandArgs>,

    /// Print API documentation URL.
    #[arg(long)]
    pub api: bool,

    /// Print workspace documentation URL.
    #[arg(long)]
    pub workspace: bool,
}

pub fn run(args: DocsArgs, output: Output) -> Result<()> {
    let name = args
        .name
        .map(|args| args.name)
        .unwrap_or_else(|| "<root>".to_owned());

    placeholder(
        output,
        "docs",
        format!(
            "name={}, api={}, workspace={}",
            name, args.api, args.workspace
        ),
    )
}
