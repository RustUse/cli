use anyhow::Result;
use clap::Args;

use crate::{output::Output, rustuse::catalog};

use super::NamedCommandArgs;

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,
}

pub fn run(args: InfoArgs, output: Output) -> Result<()> {
    let entries = catalog::all_entries()?;

    let Some(entry) = entries.iter().find(|entry| entry.name == args.name.name) else {
        let message = format!("No RustUse entry found for `{}`.", args.name.name);
        output.record("info", "missing", &message);
        return Ok(());
    };

    if output.is_json() {
        output.record(
            "info",
            "entry",
            &format!(
                "name={}, kind={}, set={}, docs={}",
                entry.name, entry.kind, entry.set, entry.docs_url
            ),
        );
        return Ok(());
    }

    output.line(&entry.name);
    output.line(format!("kind: {}", entry.kind));
    output.line(format!("set: {}", entry.set));
    output.line(format!("docs: {}", entry.docs_url));

    Ok(())
}
