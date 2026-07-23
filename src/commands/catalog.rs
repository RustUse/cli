//! Searches, inspects, generates, and validates the RustUse crate catalog.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use serde::Serialize;

use crate::{output::Output, rustuse::catalog};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CatalogArgs {
    #[command(subcommand)]
    pub command: CatalogCommand,
}

#[derive(Debug, Subcommand)]
pub enum CatalogCommand {
    /// Discover catalog entries from local repositories.
    Discover(CatalogPathArgs),

    /// Generate catalog artifacts.
    Generate(CatalogGenerateArgs),

    /// Check catalog consistency.
    Check(CatalogPathArgs),

    /// Print one catalog entry.
    Info(CatalogInfoArgs),

    /// Search catalog entries.
    Search(CatalogSearchArgs),
}

#[derive(Debug, Args)]
pub struct CatalogPathArgs {
    /// RustUse root path.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct CatalogGenerateArgs {
    /// RustUse root path.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Optional output file or directory.
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct CatalogInfoArgs {
    /// Catalog entry name.
    #[arg(value_name = "NAME")]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CatalogSearchArgs {
    /// Search query.
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Limit result count.
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CatalogStatus {
    Ok,
    Empty,
    Missing,
    Generated,
    Written,
}

#[derive(Debug, Serialize)]
struct DiscoverResponse {
    command: &'static str,
    status: CatalogStatus,
    root: PathBuf,
    returned: usize,
    entries: Vec<String>,
}

#[derive(Debug, Serialize)]
struct GenerateResponse {
    command: &'static str,
    status: CatalogStatus,
    root: PathBuf,
    output: Option<PathBuf>,
    generated: usize,
    entries: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CheckResponse {
    command: &'static str,
    status: CatalogStatus,
    root: PathBuf,
    checked: usize,
}

#[derive(Debug, Serialize)]
struct InfoResponse {
    command: &'static str,
    status: CatalogStatus,
    query: String,
    entry: Option<CatalogEntry>,
}

#[derive(Debug, Serialize)]
struct SearchResponse {
    command: &'static str,
    status: CatalogStatus,
    query: String,
    total_entries: usize,
    total_matches: usize,
    returned: usize,
    truncated: bool,
    results: Vec<CatalogEntry>,
}

#[derive(Debug, Serialize)]
struct CatalogEntry {
    name: String,
    kind: String,
    facade: String,
    docs_url: String,
}

pub fn run(args: CatalogArgs, output: Output) -> Result<()> {
    match args.command {
        CatalogCommand::Discover(args) => discover(args, &output),
        CatalogCommand::Generate(args) => generate(args, &output),
        CatalogCommand::Check(args) => check(args, &output),
        CatalogCommand::Info(args) => run_info(args, &output),
        CatalogCommand::Search(args) => run_search(args, &output),
    }
}

fn discover(args: CatalogPathArgs, output: &Output) -> Result<()> {
    let mut entries = discover_entries(&args.path)?;
    entries.sort();

    let response = DiscoverResponse {
        command: "catalog discover",
        status: if entries.is_empty() {
            CatalogStatus::Empty
        } else {
            CatalogStatus::Ok
        },
        root: args.path,
        returned: entries.len(),
        entries,
    };

    render_discover(output, &response)
}

fn render_discover(output: &Output, response: &DiscoverResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    if response.entries.is_empty() {
        return output.line("No local use-* Cargo packages found.");
    }

    for entry in &response.entries {
        output.line(entry)?;
    }

    Ok(())
}

fn generate(args: CatalogGenerateArgs, output: &Output) -> Result<()> {
    let mut entries = discover_entries(&args.path)?;
    entries.sort();

    let contents = entries
        .iter()
        .map(|entry| format!("- {entry}\n"))
        .collect::<String>();

    let status = if let Some(path) = &args.output {
        fs::write(path, contents)
            .with_context(|| format!("failed to write `{}`", path.display()))?;

        CatalogStatus::Written
    } else {
        CatalogStatus::Generated
    };

    let response = GenerateResponse {
        command: "catalog generate",
        status,
        root: args.path,
        output: args.output,
        generated: entries.len(),
        entries,
    };

    render_generate(output, &response)
}

fn render_generate(output: &Output, response: &GenerateResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    if let Some(path) = &response.output {
        return output.line(format!(
            "Wrote {} catalog {} to `{}`.",
            response.generated,
            entry_label(response.generated),
            path.display()
        ));
    }

    if response.entries.is_empty() {
        return output.line("No local use-* Cargo packages found.");
    }

    for entry in &response.entries {
        output.line(format!("- {entry}"))?;
    }

    Ok(())
}

fn check(args: CatalogPathArgs, output: &Output) -> Result<()> {
    let entries = discover_entries(&args.path)?;
    let mut sorted = entries.clone();
    sorted.sort();

    if entries != sorted {
        bail!("local catalog entries are not deterministic");
    }

    let response = CheckResponse {
        command: "catalog check",
        status: CatalogStatus::Ok,
        root: args.path,
        checked: entries.len(),
    };

    if output.is_json() {
        return output.json(&response);
    }

    output.line(format!(
        "Checked {} local use-* Cargo {}.",
        response.checked,
        package_label(response.checked)
    ))
}

fn discover_entries(path: &Path) -> Result<Vec<String>> {
    let mut entries = Vec::new();

    for entry in
        fs::read_dir(path).with_context(|| format!("failed to read `{}`", path.display()))?
    {
        let entry = entry.with_context(|| format!("failed to inspect `{}`", path.display()))?;
        let child = entry.path();
        let manifest_path = child.join("Cargo.toml");

        if !child.is_dir() || !manifest_path.is_file() {
            continue;
        }

        let raw = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read `{}`", manifest_path.display()))?;

        let manifest: toml::Value = toml::from_str(&raw)
            .with_context(|| format!("failed to parse `{}`", manifest_path.display()))?;

        let Some(name) = manifest
            .get("package")
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
        else {
            continue;
        };

        if name.starts_with("use-") {
            entries.push(name.to_owned());
        }
    }

    Ok(entries)
}

fn run_info(args: CatalogInfoArgs, output: &Output) -> Result<()> {
    let entry = catalog::find_by_name(&args.name).map(|entry| CatalogEntry {
        name: entry.name.to_string(),
        kind: entry.kind.to_string(),
        facade: entry.set.to_string(),
        docs_url: entry.docs_url.to_string(),
    });

    let response = InfoResponse {
        command: "catalog info",
        status: if entry.is_some() {
            CatalogStatus::Ok
        } else {
            CatalogStatus::Missing
        },
        query: args.name,
        entry,
    };

    render_info(output, &response)
}

fn render_info(output: &Output, response: &InfoResponse) -> Result<()> {
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

fn run_search(args: CatalogSearchArgs, output: &Output) -> Result<()> {
    let total_entries = catalog::all_entries().len();
    let matches = catalog::search(&args.query);
    let total_matches = matches.len();

    let results: Vec<_> = matches
        .into_iter()
        .take(args.limit)
        .map(|entry| CatalogEntry {
            name: entry.name.to_string(),
            kind: entry.kind.to_string(),
            facade: entry.set.to_string(),
            docs_url: entry.docs_url.to_string(),
        })
        .collect();

    let returned = results.len();

    let response = SearchResponse {
        command: "catalog search",
        status: if results.is_empty() {
            CatalogStatus::Empty
        } else {
            CatalogStatus::Ok
        },
        query: args.query,
        total_entries,
        total_matches,
        returned,
        truncated: total_matches > returned,
        results,
    };

    render_search(output, &response)
}

fn render_search(output: &Output, response: &SearchResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    if response.results.is_empty() {
        return output.line(format!("No RustUse entries matched `{}`.", response.query));
    }

    output.detail(format!(
        "Searched {} RustUse entries.",
        response.total_entries
    ))?;

    output.line(format!(
        "Found {} RustUse {}:",
        response.returned,
        entry_label(response.returned)
    ))?;

    for entry in &response.results {
        output.line(format!(
            "- {} ({}, {}) - {}",
            entry.name, entry.kind, entry.facade, entry.docs_url
        ))?;
    }

    Ok(())
}

const fn entry_label(count: usize) -> &'static str {
    if count == 1 { "entry" } else { "entries" }
}

const fn package_label(count: usize) -> &'static str {
    if count == 1 { "package" } else { "packages" }
}
