use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse};

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
    match args.kind {
        ScanKind::Auto => run_auto(&args.path, output),
        ScanKind::Facade => rustuse::facade::scan::scan_facade(&args.path, output),
        ScanKind::Root => rustuse::root::scan::scan_root(&args.path, output),
        ScanKind::Catalog => scan_catalog(output),
    }
}

fn run_auto(path: &Path, output: Output) -> Result<()> {
    if rustuse::facade::discover::is_facade(path) {
        rustuse::facade::scan::scan_facade(path, output)
    } else {
        rustuse::root::scan::scan_root(path, output)
    }
}

fn scan_catalog(output: Output) -> Result<()> {
    let entries = rustuse::catalog::all_entries();

    if output.is_json() {
        output.record("scan", "ok", &format!("catalog entries={}", entries.len()));
        return Ok(());
    }

    output.line(format!("RustUse catalog scan - {} entries", entries.len()));

    for entry in entries {
        output.line(format!("- {} ({}, {})", entry.name, entry.kind, entry.set));
    }

    Ok(())
}
