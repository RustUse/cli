/* use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse};

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// RustUse project path to report on.
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Force report kind instead of auto-detecting.
    #[arg(long, value_enum, default_value_t = ReportKind::Auto)]
    pub kind: ReportKind,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ReportKind {
    Auto,
    Root,
    Facade,
}

pub fn run(args: ReportArgs, output: Output) -> Result<()> {
    match args.kind {
        ReportKind::Auto => run_auto(&args.path, output),
        ReportKind::Root => rustuse::root::report::run_path(&args.path, output),
        ReportKind::Facade => rustuse::facade::report::run_path(&args.path, output),
    }
}

fn run_auto(path: &Path, output: Output) -> Result<()> {
    if is_facade(path) {
        rustuse::facade::report::run_path(path, output)
    } else {
        rustuse::root::report::run_path(path, output)
    }
}

fn is_facade(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("use-"))
        && path.join("Cargo.toml").is_file()
        && path.join("crates").is_dir()
}
 */

/* use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse};

use super::placeholder;

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Path to report on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Report target kind.
    #[arg(long, value_enum, default_value_t = ReportKind::Auto)]
    pub kind: ReportKind,

    /// Optional output file.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReportKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

pub fn run(args: ReportArgs, output: Output) -> Result<()> {
    let report_output = args
        .output
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<default>".to_owned());

    placeholder(
        output,
        "report",
        format!(
            "path={}, kind={:?}, output={}",
            args.path.display(),
            args.kind,
            report_output
        ),
    )
} */

/* use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse};

use super::placeholder;

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Path to report on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Report target kind.
    #[arg(long, value_enum, default_value_t = ReportKind::Auto)]
    pub kind: ReportKind,

    /// Optional output file.
    ///
    /// Reserved for explicit report output routing. Existing domain report
    /// writers may still use their default report path until wired.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReportKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

pub fn run(args: ReportArgs, output: Output) -> Result<()> {
    if let Some(report_output) = &args.output {
        output.detail(format!(
            "requested report output: {}",
            report_output.display()
        ));
    }

    match args.kind {
        ReportKind::Auto => run_auto(&args.path, output),
        ReportKind::Facade => rustuse::facade::report::run_path(&args.path, output),
        ReportKind::Root => rustuse::root::report::run_path(&args.path, output),
        ReportKind::Catalog => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Catalog, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
        ReportKind::Ci => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Ci, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
    }
}

fn run_auto(path: &Path, output: Output) -> Result<()> {
    let resolved = resolve_existing_path(path);

    if is_facade(&resolved) {
        rustuse::facade::report::run_path(path, output)
    } else {
        rustuse::root::report::run_path(path, output)
    }
}

fn is_facade(path: &Path) -> bool {
    has_facade_shape(path) && (has_facade_directory_name(path) || has_facade_package_name(path))
}

fn has_facade_shape(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() && path.join("crates").is_dir()
}

fn has_facade_directory_name(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("use-"))
}

fn has_facade_package_name(path: &Path) -> bool {
    let manifest_path = path.join("Cargo.toml");
    let Ok(raw) = fs::read_to_string(manifest_path) else {
        return false;
    };

    let Ok(manifest) = toml::from_str::<toml::Value>(&raw) else {
        return false;
    };

    manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .is_some_and(|name| name.starts_with("use-"))
}

fn resolve_existing_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = fs::canonicalize(path) {
        return canonical;
    }

    if path.is_absolute() {
        return path.to_path_buf();
    }

    env::current_dir()
        .map(|current_dir| current_dir.join(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

fn output_path_label(path: Option<&Path>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "<default>".to_owned())
}
 */

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse};

use super::placeholder;

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Path to report on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Report target kind.
    #[arg(long, value_enum, default_value_t = ReportKind::Auto)]
    pub kind: ReportKind,

    /// Optional output file.
    ///
    /// Reserved for explicit report output routing. Existing domain report
    /// writers may still use their default report path until wired.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReportKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

pub fn run(args: ReportArgs, output: Output) -> Result<()> {
    if let Some(report_output) = &args.output {
        output.detail(format!(
            "requested report output: {}",
            report_output.display()
        ));
    }

    match args.kind {
        ReportKind::Auto => run_auto(&args.path, output),
        ReportKind::Facade => rustuse::facade::report::run_path(&args.path, output),
        ReportKind::Root => rustuse::root::report::run_path(&args.path, output),
        ReportKind::Catalog => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Catalog, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
        ReportKind::Ci => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Ci, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
    }
}

fn run_auto(path: &Path, output: Output) -> Result<()> {
    let resolved = resolve_existing_path(path);

    if is_facade(&resolved) {
        rustuse::facade::report::run_path(path, output)
    } else {
        rustuse::root::report::run_path(path, output)
    }
}

fn is_facade(path: &Path) -> bool {
    has_facade_shape(path) && (has_facade_directory_name(path) || has_facade_package_name(path))
}

fn has_facade_shape(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() && path.join("crates").is_dir()
}

fn has_facade_directory_name(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("use-"))
}

fn has_facade_package_name(path: &Path) -> bool {
    let manifest_path = path.join("Cargo.toml");
    let Ok(raw) = fs::read_to_string(manifest_path) else {
        return false;
    };

    let Ok(manifest) = toml::from_str::<toml::Value>(&raw) else {
        return false;
    };

    manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .is_some_and(|name| name.starts_with("use-"))
}

fn resolve_existing_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = fs::canonicalize(path) {
        return canonical;
    }

    if path.is_absolute() {
        return path.to_path_buf();
    }

    env::current_dir()
        .map(|current_dir| current_dir.join(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

fn output_path_label(path: Option<&Path>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "<default>".to_owned())
}
