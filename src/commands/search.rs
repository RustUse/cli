use anyhow::Result;
use clap::Args;

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

pub fn run(args: SearchArgs, output: Output) -> Result<()> {
    let total_entries = catalog::all_entries().len();
    let matches = catalog::search(&args.query);
    let limited_matches: Vec<_> = matches.into_iter().take(args.limit).collect();

    if limited_matches.is_empty() {
        let message = format!("No RustUse entries matched `{}`.", args.query);
        output.record("search", "empty", &message);
        return Ok(());
    }

    if output.is_json() {
        for entry in limited_matches {
            let message = format!(
                "name={}, kind={}, set={}, docs={}",
                entry.name, entry.kind, entry.set, entry.docs_url
            );
            output.record("search", "match", &message);
        }
        return Ok(());
    }

    output.detail(format!("Searched {total_entries} RustUse entries."));
    output.line(format!(
        "Found {} RustUse entr{}:",
        limited_matches.len(),
        plural_y(limited_matches.len())
    ));

    for entry in limited_matches {
        output.line(format!(
            "- {} ({}, {}) - {}",
            entry.name, entry.kind, entry.set, entry.docs_url
        ));
    }

    Ok(())
}

fn plural_y(count: usize) -> &'static str {
    if count == 1 { "y" } else { "ies" }
}
