use anyhow::Result;
use clap::Args;

// use crate::cli::SearchArgs;
use crate::index;
use crate::output::Output;

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search text, such as geometry or use-slug.
    pub query: String,
}

pub fn run(args: SearchArgs, output: Output) -> Result<()> {
    let total_entries = index::all_entries().len();
    let matches = index::search(&args.query);

    if matches.is_empty() {
        let message = format!("No RustUse entries matched `{}`.", args.query);
        output.record("search", "empty", &message);
        return Ok(());
    }

    if output.is_json() {
        for entry in matches {
            let message = format!(
                "name={}, kind={}, set={}, docs={}",
                entry.name, entry.kind, entry.set, entry.docs_url
            );
            output.record("search", "match", &message);
        }
        return Ok(());
    }

    output.detail(format!("Searched {total_entries} placeholder entries."));
    output.line(format!(
        "Found {} RustUse entr{}:",
        matches.len(),
        plural_y(matches.len())
    ));
    for entry in matches {
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
