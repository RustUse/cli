use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub struct DevCheckArgs {
    /// Workspace root to inspect.
    #[arg(default_value = ".")]
    pub workspace: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct CheckOptions {
    pub root: PathBuf,
}

impl CheckOptions {
    pub(crate) fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CheckReport {
    pub root: PathBuf,
    pub findings: Vec<CheckFinding>,
}

impl CheckReport {
    pub(crate) fn new(root: PathBuf) -> Self {
        Self {
            root,
            findings: Vec::new(),
        }
    }

    pub(crate) fn is_clean(&self) -> bool {
        self.findings
            .iter()
            .all(|finding| finding.severity != CheckSeverity::Error)
    }

    pub(crate) fn error_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity == CheckSeverity::Error)
            .count()
    }

    pub(crate) fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity == CheckSeverity::Warning)
            .count()
    }

    fn error(&mut self, message: impl Into<String>) {
        self.findings.push(CheckFinding {
            severity: CheckSeverity::Error,
            message: message.into(),
        });
    }

    fn warning(&mut self, message: impl Into<String>) {
        self.findings.push(CheckFinding {
            severity: CheckSeverity::Warning,
            message: message.into(),
        });
    }

    fn info(&mut self, message: impl Into<String>) {
        self.findings.push(CheckFinding {
            severity: CheckSeverity::Info,
            message: message.into(),
        });
    }

    pub(crate) fn to_text(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!("RustUse dev check: {}", self.root.display()));

        if self.findings.is_empty() {
            lines.push("ok: no findings".to_owned());
            return lines.join("\n");
        }

        for finding in &self.findings {
            let label = match finding.severity {
                CheckSeverity::Error => "error",
                CheckSeverity::Warning => "warning",
                CheckSeverity::Info => "info",
            };

            lines.push(format!("{label}: {}", finding.message));
        }

        lines.push(format!(
            "summary: {} error(s), {} warning(s)",
            self.error_count(),
            self.warning_count()
        ));

        lines.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckFinding {
    pub severity: CheckSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckSeverity {
    Error,
    Warning,
    Info,
}

pub(crate) fn run(options: CheckOptions) -> Result<CheckReport> {
    let root = normalize_root(&options.root)?;
    let mut report = CheckReport::new(root.clone());

    check_root_exists(&root, &mut report);
    check_workspace_manifest(&root, &mut report)?;
    check_crates_directory(&root, &mut report)?;
    check_facade_shape(&root, &mut report)?;

    Ok(report)
}

fn normalize_root(root: &Path) -> Result<PathBuf> {
    if root.is_absolute() {
        Ok(root.to_path_buf())
    } else {
        std::env::current_dir()
            .context("failed to read current directory")?
            .join(root)
            .canonicalize()
            .with_context(|| format!("failed to resolve workspace root `{}`", root.display()))
    }
}

fn check_root_exists(root: &Path, report: &mut CheckReport) {
    if !root.exists() {
        report.error(format!("workspace root does not exist: {}", root.display()));
        return;
    }

    if !root.is_dir() {
        report.error(format!(
            "workspace root is not a directory: {}",
            root.display()
        ));
    }
}

fn check_workspace_manifest(root: &Path, report: &mut CheckReport) -> Result<()> {
    let manifest_path = root.join("Cargo.toml");

    if !manifest_path.exists() {
        report.error("missing root Cargo.toml");
        return Ok(());
    }

    let manifest = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read `{}`", manifest_path.display()))?;

    if !manifest.contains("[workspace]") {
        report.warning("root Cargo.toml does not contain a [workspace] table");
    }

    if !manifest.contains("resolver = \"3\"") {
        report.warning("workspace resolver is not explicitly set to \"3\"");
    }

    if !manifest.contains("[workspace.package]") {
        report.warning("root Cargo.toml does not contain a [workspace.package] table");
    }

    if !manifest.contains("edition = \"2024\"") && !manifest.contains("edition.workspace = true") {
        report.warning("Rust 2024 edition was not detected");
    }

    Ok(())
}

fn check_crates_directory(root: &Path, report: &mut CheckReport) -> Result<()> {
    let crates_dir = root.join("crates");

    if !crates_dir.exists() {
        report.warning("missing crates/ directory");
        return Ok(());
    }

    if !crates_dir.is_dir() {
        report.error("crates exists but is not a directory");
        return Ok(());
    }

    let mut crate_count = 0usize;

    for entry in fs::read_dir(&crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry =
            entry.with_context(|| format!("failed to read entry in `{}`", crates_dir.display()))?;

        let path = entry.path();

        if path.is_dir() && path.join("Cargo.toml").exists() {
            crate_count += 1;
        }
    }

    if crate_count == 0 {
        report.warning("crates/ exists but no crate manifests were found");
    } else {
        report.info(format!("found {crate_count} crate(s) in crates/"));
    }

    Ok(())
}

fn check_facade_shape(root: &Path, report: &mut CheckReport) -> Result<()> {
    let Some(root_name) = root.file_name().and_then(|name| name.to_str()) else {
        report.warning("could not infer workspace name from root directory");
        return Ok(());
    };

    if !root_name.starts_with("use-") {
        report.info(format!(
            "workspace root `{root_name}` is not a use-* facade workspace"
        ));
        return Ok(());
    }

    let facade_manifest = root.join("crates").join(root_name).join("Cargo.toml");

    if facade_manifest.exists() {
        report.info(format!("found facade crate: crates/{root_name}"));
    } else {
        report.warning(format!(
            "expected facade crate at crates/{root_name}/Cargo.toml"
        ));
    }

    Ok(())
}
