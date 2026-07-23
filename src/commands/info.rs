//! Displays detailed information about a RustUse crate or primitive.

use anyhow::Result;
use clap::Args;
use serde::Serialize;

use crate::{output::Output, rustuse::catalog};

use super::NamedCommandArgs;

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[command(flatten)]
    pub name: NamedCommandArgs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum InfoStatus {
    Ok,
    Missing,
}

#[derive(Debug, Serialize)]
struct InfoResponse {
    command: &'static str,
    status: InfoStatus,
    query: String,
    entry: Option<InfoEntry>,
}

#[derive(Debug, Serialize)]
struct InfoEntry {
    name: String,
    kind: String,
    facade: String,
    docs_url: String,
}

pub fn run(args: InfoArgs, output: Output) -> Result<()> {
    let entry = catalog::find_by_name(&args.name.name).map(|entry| InfoEntry {
        name: entry.name.to_string(),
        kind: entry.kind.to_string(),
        facade: entry.set.to_string(),
        docs_url: entry.docs_url.to_string(),
    });

    let response = InfoResponse {
        command: "info",
        status: if entry.is_some() {
            InfoStatus::Ok
        } else {
            InfoStatus::Missing
        },
        query: args.name.name,
        entry,
    };

    render(&output, &response)
}

fn render(output: &Output, response: &InfoResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    let Some(entry) = &response.entry else {
        return output.line(format!("No RustUse entry found for `{}`.", response.query));
    };

    output.line(&entry.name)?;
    output.line(format!("kind: {}", entry.kind))?;
    output.line(format!("facade: {}", entry.facade))?;
    output.line(format!("docs: {}", entry.docs_url))
}
