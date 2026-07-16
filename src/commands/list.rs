//! Lists RustUse dependencies, tracked entries, or available catalog entries.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;
use serde::Serialize;

use crate::{
    output::Output,
    rustuse::{catalog, config, project},
};

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show all available catalog entries.
    #[arg(long)]
    pub all: bool,

    /// Project directory to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
struct ListResponse {
    command: &'static str,
    status: &'static str,
    scope: ListScope,
    returned: usize,
    entries: Vec<ListEntry>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ListScope {
    Catalog,
    Tracked,
}

#[derive(Debug, Serialize)]
struct ListEntry {
    name: String,
    mode: String,
}

pub fn run(args: ListArgs, output: Output) -> Result<()> {
    if args.all {
        return list_catalog(&output);
    }

    list_tracked(&args.path, &output)
}

fn list_catalog(output: &Output) -> Result<()> {
    let mut entries: Vec<_> = catalog::all_entries()
        .iter()
        .map(|entry| ListEntry {
            name: entry.name.to_string(),
            mode: "catalog".to_owned(),
        })
        .collect();

    entries.sort_by(|left, right| left.name.cmp(&right.name));

    render(
        output,
        ListResponse {
            command: "list",
            status: if entries.is_empty() { "empty" } else { "ok" },
            scope: ListScope::Catalog,
            returned: entries.len(),
            entries,
        },
    )
}

fn list_tracked(path: &PathBuf, output: &Output) -> Result<()> {
    let root = std::fs::canonicalize(path)
        .with_context(|| format!("failed to resolve `{}`", path.display()))?;
    let state = project::detect(&root);

    if !state.has_config {
        bail!(
            "no rustuse.toml found in `{}`; run `rustuse init` first",
            root.display()
        );
    }

    let raw = std::fs::read_to_string(&state.config_path)
        .with_context(|| format!("failed to read `{}`", state.config_path.display()))?;
    let rustuse_config = config::from_toml(&raw)?;

    let mut entries: Vec<_> = rustuse_config
        .primitives
        .into_iter()
        .map(|primitive| ListEntry {
            name: primitive.name,
            mode: primitive.mode.as_str().to_owned(),
        })
        .collect();

    entries.sort_by(|left, right| left.name.cmp(&right.name));

    render(
        output,
        ListResponse {
            command: "list",
            status: if entries.is_empty() { "empty" } else { "ok" },
            scope: ListScope::Tracked,
            returned: entries.len(),
            entries,
        },
    )
}

fn render(output: &Output, response: ListResponse) -> Result<()> {
    if output.is_json() {
        return output.json(&response);
    }

    if response.entries.is_empty() {
        let message = match response.scope {
            ListScope::Catalog => "No RustUse catalog entries are available.",
            ListScope::Tracked => "No tracked RustUse primitives.",
        };

        return output.line(message);
    }

    for entry in response.entries {
        output.line(format!("{} ({})", entry.name, entry.mode))?;
    }

    Ok(())
}
