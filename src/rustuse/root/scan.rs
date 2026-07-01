use std::path::PathBuf;

// use anyhow::{Result, bail};
use clap::{Args, ValueEnum};

// use crate::output::Output;
// use crate::rustuse;

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Path to scan.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Scan kind to run.
    #[arg(long, value_enum, default_value_t = ScanKind::Auto)]
    pub kind: ScanKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ScanKind {
    /// Detect the scan kind from the path.
    Auto,

    /// Scan a RustUse development root containing many use-* repositories.
    Root,

    /// Scan a single use-* facade repository.
    Facade,
}

/* pub fn run(args: ScanArgs, output: Output) -> Result<()> {
    match resolve_kind(&args)? {
        ScanKind::Auto => unreachable!("resolve_kind never returns auto"),
        ScanKind::Root => unreachable!(
            "resolve_kind never returns Root {{ is_json: {} }}",
            output.is_json()
        ),
        ScanKind::Facade => bail!(
            "facade scan is not wired yet; use `rustuse report {} --kind facade` for now",
            args.path.display()
        ),
    }
}

fn resolve_kind(args: &ScanArgs) -> Result<ScanKind> {
    match args.kind {
        ScanKind::Auto => {
            if looks_like_facade(&args.path) {
                Ok(ScanKind::Facade)
            } else {
                Ok(ScanKind::Root)
            }
        },
        ScanKind::Root | ScanKind::Facade => Ok(args.kind),
    }
}

fn looks_like_facade(path: &PathBuf) -> bool {
    let name_is_facade = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("use-"));

    name_is_facade && path.join("Cargo.toml").is_file() && path.join("crates").is_dir()
} */
