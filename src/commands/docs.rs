/* use anyhow::{Context, Result};
use clap::Args;

use crate::commands::NamedCommandArgs;
use crate::output::Output;

use super::entry_for;

#[derive(Debug, Args)]
pub struct DocsArgs {
    #[command(flatten)]
    pub target: NamedCommandArgs,

    /// Print API RustDocs URL.
    #[arg(long, conflicts_with = "workspace")]
    pub api: bool,

    /// Print workspace RustDocs URL.
    #[arg(long, conflicts_with = "api")]
    pub workspace: bool,
}

pub fn run(args: DocsArgs, output: Output) -> Result<()> {
    let entry = entry_for(&args.target.name)?;

    let url = if args.workspace {
        entry.workspace_docs_url.with_context(|| {
            format!(
                "`{}` does not have workspace RustDocs in the placeholder index",
                entry.name
            )
        })?
    } else if args.api {
        entry.api_docs_url
    } else {
        entry.docs_url
    };

    output.record("docs", "ok", &url);

    Ok(())
}
 */

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
