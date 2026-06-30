/* use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::{output::Output, rustuse};

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// RustUse root path to scan.
    #[arg(default_value = ".", value_name = "ROOT")]
    pub root: PathBuf,
}

pub fn run(args: ScanArgs, output: Output) -> Result<()> {
    rustuse::root::scan::run_path(args.root, output)
}
 */

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::output::Output;

use super::placeholder;

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Path to scan.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Scan target kind.
    #[arg(long, value_enum, default_value_t = ScanKind::Auto)]
    pub kind: ScanKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ScanKind {
    Auto,
    Facade,
    Root,
    Catalog,
}

pub fn run(args: ScanArgs, output: Output) -> Result<()> {
    placeholder(
        output,
        "scan",
        format!("path={}, kind={:?}", args.path.display(), args.kind),
    )
}
