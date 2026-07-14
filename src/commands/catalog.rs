//! Searches, inspects, generates, and validates the RustUse crate catalog.

use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};

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

pub fn run(args: CatalogArgs, output: Output) -> Result<()> {
    match args.command {
        CatalogCommand::Discover(args) => discover(args, output),
        CatalogCommand::Generate(args) => generate(args, output),
        CatalogCommand::Check(args) => check(args, output),
        CatalogCommand::Info(args) => run_info(args, output),
        CatalogCommand::Search(args) => run_search(args, output),
    }
}

fn discover(args: CatalogPathArgs, output: Output) -> Result<()> {
    let mut entries = discover_entries(&args.path)?;
    entries.sort();
    if entries.is_empty() {
        output.record(
            "catalog discover",
            "empty",
            "No local use-* Cargo packages found.",
        );
    } else {
        for entry in entries {
            output.record("catalog discover", "entry", &entry);
        }
    }
    Ok(())
}

fn generate(args: CatalogGenerateArgs, output: Output) -> Result<()> {
    let mut entries = discover_entries(&args.path)?;
    entries.sort();
    let contents = entries
        .iter()
        .map(|entry| format!("- {entry}\n"))
        .collect::<String>();
    if let Some(path) = args.output {
        fs::write(&path, &contents)
            .with_context(|| format!("failed to write `{}`", path.display()))?;
        output.record("catalog generate", "written", &path.display().to_string());
    } else if output.is_json() {
        output.record("catalog generate", "generated", &contents);
    } else {
        output.line(contents.trim_end());
    }
    Ok(())
}

fn check(args: CatalogPathArgs, output: Output) -> Result<()> {
    let entries = discover_entries(&args.path)?;
    let mut sorted = entries.clone();
    sorted.sort();
    if entries != sorted {
        bail!("local catalog entries are not deterministic");
    }
    output.record(
        "catalog check",
        "ok",
        &format!("checked {} local use-* Cargo package(s)", entries.len()),
    );
    Ok(())
}

fn discover_entries(path: &std::path::Path) -> Result<Vec<String>> {
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

fn run_info(args: CatalogInfoArgs, output: Output) -> Result<()> {
    let Some(entry) = catalog::find_by_name(&args.name) else {
        let message = format!("No RustUse entry found for `{}`.", args.name);
        output.record("catalog info", "missing", &message);
        return Ok(());
    };

    if output.is_json() {
        output.record(
            "catalog info",
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

fn run_search(args: CatalogSearchArgs, output: Output) -> Result<()> {
    let total_entries = catalog::all_entries().len();
    let matches = catalog::search(&args.query);
    let limited_matches: Vec<_> = matches.into_iter().take(args.limit).collect();

    if limited_matches.is_empty() {
        let message = format!("No RustUse entries matched `{}`.", args.query);
        output.record("catalog search", "empty", &message);
        return Ok(());
    }

    if output.is_json() {
        for entry in limited_matches {
            let message = format!(
                "name={}, kind={}, set={}, docs={}",
                entry.name, entry.kind, entry.set, entry.docs_url
            );
            output.record("catalog search", "match", &message);
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
