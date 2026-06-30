/* use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// RustUse project path to check.
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

pub fn run(args: CheckArgs, _output: Output) -> Result<()> {
    let report = rustuse::root::dev::check::check(&args.path)?;

    println!("{}", report.to_text());

    if report.is_clean() {
        Ok(())
    } else {
        bail!("RustUse dev check failed")
    }

    println!("check.rs works: {}", args.path.display());

    Ok(())
}
 */

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::output::Output;

use super::placeholder;

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Path to check.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Check target kind.
    #[arg(long, value_enum, default_value_t = CheckKind::Auto)]
    pub kind: CheckKind,

    /// Fail if warnings are found.
    #[arg(long)]
    pub deny_warnings: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum CheckKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

pub fn run(args: CheckArgs, output: Output) -> Result<()> {
    placeholder(
        output,
        "check",
        format!(
            "path={}, kind={:?}, deny_warnings={}",
            args.path.display(),
            args.kind,
            args.deny_warnings
        ),
    )
}
