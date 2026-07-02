use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{output::Output, rustuse::catalog};

use super::placeholder;

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
        CatalogCommand::Discover(args) => staged(
            output,
            "catalog discover",
            format!("path={}", args.path.display()),
        ),
        CatalogCommand::Generate(args) => {
            let catalog_output = args
                .output
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<default>".to_owned());

            staged(
                output,
                "catalog generate",
                format!("path={}, output={}", args.path.display(), catalog_output),
            )
        },
        CatalogCommand::Check(args) => staged(
            output,
            "catalog check",
            format!("path={}", args.path.display()),
        ),
        CatalogCommand::Info(args) => run_info(args, output),
        CatalogCommand::Search(args) => run_search(args, output),
    }
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

fn staged(output: Output, command: &str, detail: String) -> Result<()> {
    let staged_detail = format!("staged=true, {detail}");
    placeholder(output, command, staged_detail)
}
