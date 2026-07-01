use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

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
        CatalogCommand::Discover(args) => placeholder(
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

            placeholder(
                output,
                "catalog generate",
                format!("path={}, output={}", args.path.display(), catalog_output),
            )
        },
        CatalogCommand::Check(args) => placeholder(
            output,
            "catalog check",
            format!("path={}", args.path.display()),
        ),
        CatalogCommand::Info(args) => {
            placeholder(output, "catalog info", format!("name={}", args.name))
        },
        CatalogCommand::Search(args) => placeholder(
            output,
            "catalog search",
            format!("query={}, limit={}", args.query, args.limit),
        ),
    }
}
