use anyhow::Result;

use crate::cli::NamedCommandArgs;
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

    if let Some(workspace_docs_url) = entry.workspace_docs_url {
        output.line(format!("workspace docs: {workspace_docs_url}"));
    }

    output.line(format!("modes: {}", entry.modes_label()));

    Ok(())
}
