/* use std::fmt::Debug;

// use anyhow::{Result, bail};
use anyhow::Result;
use clap::{Args, Subcommand};

use crate::output::Output;

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "RustUse catalog management commands."
)]
pub struct CatalogArgs {
    #[command(subcommand)]
    pub command: CatalogCommand,
}

#[derive(Debug, Subcommand)]
pub enum CatalogCommand {
    // / Check a RustUse facade/workspace for standard project shape.
    // Check(check::CatalogCheckArgs),
    // / Show RustUse catalog info for a workspace.
    // Info(info::CatalogInfoArgs),

    // / Manage a root of RustUse facades.
    // Root(root::CatalogRootArgs),

    // / Manage one RustUse facade repository.
    // Facade(facade::CatalogFacadeArgs),
}

pub fn run(args: CatalogArgs, output: Output) -> Result<()> {
    match args.command {
        CatalogCommand::Check(args) => {
            let options = check::CheckOptions::new(args.workspace);
            let report = check::run(options)?;

            println!("{}", report.to_text());

            if report.is_clean() {
                Ok(())
            } else {
                bail!("RustUse catalog check failed")
            }
        },
        // CatalogCommand::Facade(args) => facade::run(args, output),
        // CatalogCommand::Info(args) => info::run(args, output),
        // CatalogCommand::Root(args) => root::run(args, output),
    }

    println!("catalog.rs works: {:?}, {:?}", args, output);

    Ok(())
}
 */

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
