//! Searches the RustUse catalog for crates and primitives.

use anyhow::Result;
use clap::Args;
use serde::Serialize;

use crate::{output::Output, rustuse::catalog};

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search query.
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Limit result count.
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Debug, Serialize)]
struct SearchResponse<'a> {
    command: &'static str,
    status: &'static str,
    query: &'a str,
    total_entries: usize,
    total_matches: usize,
    returned: usize,
    truncated: bool,
    results: Vec<SearchResult>,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    name: String,
    kind: String,
    set: String,
    docs_url: String,
}

pub fn run(args: SearchArgs, output: Output) -> Result<()> {
    let total_entries = catalog::all_entries().len();
    let matches = catalog::search(&args.query);
    let total_matches = matches.len();

    let results: Vec<_> = matches
        .into_iter()
        .take(args.limit)
        .map(|entry| SearchResult {
            name: entry.name.to_string(),
            kind: entry.kind.to_string(),
            set: entry.set.to_string(),
            docs_url: entry.docs_url.to_string(),
        })
        .collect();

    let returned = results.len();

    let response = SearchResponse {
        command: "search",
        status: if results.is_empty() { "empty" } else { "ok" },
        query: &args.query,
        total_entries,
        total_matches,
        returned,
        truncated: total_matches > returned,
        results,
    };

    if output.is_json() {
        return output.json(&response);
    }

    output.detail(format!("Searched {total_entries} RustUse entries."))?;

    if response.results.is_empty() {
        return output.line(format!("No RustUse entries matched `{}`.", response.query));
    }

    output.line(format!(
        "Found {} RustUse {}:",
        response.returned,
        entry_label(response.returned)
    ))?;

    for entry in response.results {
        output.line(format!(
            "- {} ({}, {}) - {}",
            entry.name, entry.kind, entry.set, entry.docs_url
        ))?;
    }

    Ok(())
}

const fn entry_label(count: usize) -> &'static str {
    if count == 1 { "entry" } else { "entries" }
}
