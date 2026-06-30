/* use anyhow::Result;

// use crate::cli::NamedCommandArgs;
use crate::commands::NamedCommandArgs;
use crate::output::Output;

use super::entry_for;

pub fn run(args: NamedCommandArgs, output: Output) -> Result<()> {
    let entry = entry_for(&args.name)?;

    if output.is_json() {
        let message = format!(
            "name={}, kind={}, set={}, modes={}",
            entry.name,
            entry.kind,
            entry.set,
            entry.modes_label()
        );
        output.record("info", "ok", &message);
        return Ok(());
    }

    output.line(format!("name: {}", entry.name));
    output.line(format!("kind: {}", entry.kind));
    output.line(format!("set: {}", entry.set));
    output.line(format!("docs: {}", entry.docs_url));
    output.line(format!("api docs: {}", entry.api_docs_url));

    if let Some(workspace_docs_url) = &entry.workspace_docs_url {
        output.line(format!("workspace docs: {workspace_docs_url}"));
    }

    output.line(format!("modes: {}", entry.modes_label()));

    Ok(())
}
 */

/* use anyhow::Result;
use clap::Args;

use crate::output::Output;

use super::{NamedCommandArgs, placeholder};

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,
}

pub fn run(args: InfoArgs, output: Output) -> Result<()> {
    placeholder(output, "info", format!("name={}", args.name.name))
} */

/* use anyhow::Result;
use clap::Args;

use crate::{output::Output, rustuse::catalog};

use super::NamedCommandArgs;

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,
}

pub fn run(args: InfoArgs, output: Output) -> Result<()> {
    let Some(entry) = catalog::lookup(&args.name.name)? else {
        let message = format!("No RustUse entry found for `{}`.", args.name.name);
        output.record("info", "missing", &message);
        return Ok(());
    };

    if output.is_json() {
        output.record(
            "info",
            "entry",
            &format!(
                "name={}, kind={}, set={}, crate={}, docs={}",
                entry.name, entry.kind, entry.set, entry.crate_name, entry.docs_url
            ),
        );
        return Ok(());
    }

    output.line(format!("{}", entry.name));
    output.line(format!("kind: {}", entry.kind));
    output.line(format!("set: {}", entry.set));
    output.line(format!("crate: {}", entry.crate_name));
    output.line(format!("docs: {}", entry.docs_url));

    Ok(())
} */

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

    output.line(format!("{}", entry.name));
    output.line(format!("kind: {}", entry.kind));
    output.line(format!("set: {}", entry.set));
    output.line(format!("docs: {}", entry.docs_url));

    Ok(())
}
