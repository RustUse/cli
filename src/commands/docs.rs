use anyhow::{Context, Result};
use clap::Args;

use crate::cli::NamedCommandArgs;
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

    output.record("docs", "ok", url);

    Ok(())
}
