use anyhow::{Context, Result};

use crate::cli::DocsArgs;
use crate::output::Output;

use super::entry_for;

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
