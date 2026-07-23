//! Resolves RustUse website, API, and workspace documentation URLs.

use anyhow::{Result, bail};
use clap::Args;
use serde::Serialize;

use crate::{output::Output, rustuse::catalog};

#[derive(Debug, Args)]
pub struct DocsArgs {
    /// Optional RustUse crate, primitive, or facade name.
    #[arg(value_name = "NAME")]
    pub name: Option<String>,

    /// Resolve the API documentation URL.
    #[arg(long)]
    pub api: bool,

    /// Resolve the workspace documentation URL.
    #[arg(long)]
    pub workspace: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum DocsKind {
    Website,
    Docs,
    Api,
    Workspace,
}

#[derive(Debug, Serialize)]
struct DocsResponse {
    command: &'static str,
    status: &'static str,
    name: Option<String>,
    kind: DocsKind,
    url: String,
}

pub fn run(args: DocsArgs, output: Output) -> Result<()> {
    if args.api && args.workspace {
        bail!("--api and --workspace cannot be used together");
    }

    let response = resolve(&args)?;

    if output.is_json() {
        return output.json(&response);
    }

    output.line(&response.url)
}

fn resolve(args: &DocsArgs) -> Result<DocsResponse> {
    let Some(name) = args.name.as_deref() else {
        return Ok(DocsResponse {
            command: "docs",
            status: "ok",
            name: None,
            kind: DocsKind::Website,
            url: "https://rustuse.org/".to_owned(),
        });
    };

    let Some(entry) = catalog::find_by_name(name) else {
        bail!("No RustUse entry found for `{name}`.");
    };

    let (kind, url) = if args.api {
        (DocsKind::Api, entry.api_docs_url.clone())
    } else if args.workspace {
        let Some(url) = entry.workspace_docs_url.clone() else {
            bail!("RustUse entry `{name}` has no workspace documentation URL.");
        };

        (DocsKind::Workspace, url)
    } else {
        (DocsKind::Docs, entry.docs_url.clone())
    };

    Ok(DocsResponse {
        command: "docs",
        status: "ok",
        name: Some(entry.name.clone()),
        kind,
        url,
    })
}
