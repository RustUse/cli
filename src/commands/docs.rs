use anyhow::{Result, bail};
use clap::Args;

use crate::{output::Output, rustuse::catalog};

#[derive(Debug, Args)]
pub struct DocsArgs {
    /// Optional RustUse crate, primitive, or facade name.
    #[arg(value_name = "NAME")]
    pub name: Option<String>,

    /// Print API documentation URL.
    #[arg(long)]
    pub api: bool,

    /// Print workspace documentation URL.
    #[arg(long)]
    pub workspace: bool,
}

pub fn run(args: DocsArgs, output: Output) -> Result<()> {
    let name = args.name.unwrap_or_else(|| "<root>".to_owned());

    if args.api && args.workspace {
        bail!("--api and --workspace cannot be used together");
    }

    let (url, kind) = if name == "<root>" {
        ("https://rustuse.org/".to_owned(), "root")
    } else {
        let Some(entry) = catalog::find_by_name(&name) else {
            bail!("No RustUse entry found for `{name}`.");
        };

        if args.api {
            (entry.api_docs_url.clone(), "api")
        } else if args.workspace {
            let Some(url) = entry.workspace_docs_url.clone() else {
                bail!("RustUse entry `{name}` has no workspace documentation URL.");
            };
            (url, "workspace")
        } else {
            (entry.docs_url.clone(), "docs")
        }
    };

    if output.is_json() {
        output.record(
            "docs",
            "ok",
            &format!("name={name}, kind={kind}, url={url}"),
        );
    } else {
        output.line(url);
    }

    Ok(())
}
