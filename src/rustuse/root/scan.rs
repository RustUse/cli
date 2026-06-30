//! Root scan and shared repository surface inspection.

use std::{fs, path::Path};

use anyhow::Result;

// use super::DevRootPathArgs;
use crate::output::Output;
use crate::rustuse::facade::discover::{FacadeEntry, discover_facades, display_version};
use crate::rustuse::facade::layout::StandardPath;

use crate::rustuse::utils::report::yes_no;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SurfaceStatus {
    Ok,
    Warning,
}

impl SurfaceStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RepositorySurfaceCheck {
    pub(crate) path: &'static str,
    pub(crate) label: &'static str,
    pub(crate) required: bool,
    pub(crate) present: bool,
    pub(crate) status: SurfaceStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SurfaceProfile {
    pub(crate) required_files: &'static [StandardPath],
    pub(crate) optional_files: &'static [StandardPath],
    pub(crate) required_directories: &'static [StandardPath],
    pub(crate) optional_directories: &'static [StandardPath],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RepositorySurfaceReport {
    pub(crate) files: Vec<RepositorySurfaceCheck>,
    pub(crate) directories: Vec<RepositorySurfaceCheck>,
}

impl RepositorySurfaceReport {
    pub(crate) fn status(&self) -> SurfaceStatus {
        let missing_required_file = self
            .files
            .iter()
            .any(|check| check.required && !check.present);

        let missing_required_directory = self
            .directories
            .iter()
            .any(|check| check.required && !check.present);

        if missing_required_file || missing_required_directory {
            SurfaceStatus::Warning
        } else {
            SurfaceStatus::Ok
        }
    }

    pub(crate) fn missing_required_files(&self) -> Vec<&RepositorySurfaceCheck> {
        self.files
            .iter()
            .filter(|check| check.required && !check.present)
            .collect()
    }

    pub(crate) fn missing_required_directories(&self) -> Vec<&RepositorySurfaceCheck> {
        self.directories
            .iter()
            .filter(|check| check.required && !check.present)
            .collect()
    }
}

pub(crate) fn inspect_repository_surface(
    root: &Path,
    profile: &SurfaceProfile,
) -> RepositorySurfaceReport {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    files.extend(inspect_paths(
        root,
        profile.required_files,
        true,
        SurfaceKind::File,
    ));
    files.extend(inspect_paths(
        root,
        profile.optional_files,
        false,
        SurfaceKind::File,
    ));

    directories.extend(inspect_paths(
        root,
        profile.required_directories,
        true,
        SurfaceKind::Directory,
    ));
    directories.extend(inspect_paths(
        root,
        profile.optional_directories,
        false,
        SurfaceKind::Directory,
    ));

    RepositorySurfaceReport { files, directories }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurfaceKind {
    File,
    Directory,
}

fn inspect_paths(
    root: &Path,
    paths: &'static [StandardPath],
    required: bool,
    kind: SurfaceKind,
) -> Vec<RepositorySurfaceCheck> {
    paths
        .iter()
        .map(|(path, label)| {
            let candidate = root.join(path);

            let present = match kind {
                SurfaceKind::File => candidate.is_file(),
                SurfaceKind::Directory => candidate.is_dir(),
            };

            let status = if required && !present {
                SurfaceStatus::Warning
            } else {
                SurfaceStatus::Ok
            };

            RepositorySurfaceCheck {
                path,
                label,
                required,
                present,
                status,
            }
        })
        .collect()
}

#[derive(Debug)]
struct ScanSummary {
    repo_count: usize,
    missing_git_count: usize,
    child_crate_count: usize,
    warning_count: usize,
    is_empty: bool,
}

impl ScanSummary {
    fn from_facades(facades: &[FacadeEntry]) -> Self {
        let repo_count = facades.iter().filter(|facade| facade.has_git()).count();
        let missing_git_count = facades.len().saturating_sub(repo_count);
        let child_crate_count = facades.iter().map(FacadeEntry::child_crate_count).sum();
        let warning_count = facades
            .iter()
            .filter(|facade| facade.status() != "ok")
            .count();

        Self {
            repo_count,
            missing_git_count,
            child_crate_count,
            warning_count,
            is_empty: facades.is_empty(),
        }
    }

    fn status(&self) -> &'static str {
        if self.is_empty || self.warning_count > 0 {
            "warning"
        } else {
            "ok"
        }
    }
}

fn print_facade_table(facades: &[FacadeEntry], output: Output) {
    output.line(format!(
        "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
        "Status", "Facade", "Version", "Git", "Cargo.toml", "crates/", "Children"
    ));

    output.line(format!(
        "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
        "------", "------", "-------", "---", "----------", "-------", "--------"
    ));

    for facade in facades {
        output.line(format!(
            "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
            facade.status(),
            facade.name,
            display_version(&facade.version),
            yes_no(facade.has_git()),
            yes_no(facade.has_cargo_toml()),
            yes_no(facade.has_crates_dir()),
            facade.child_crate_count(),
        ));
    }
}

fn print_missing_git_facades(facades: &[FacadeEntry], output: Output) {
    output.line("");
    output.line("facades missing .git:");

    for facade in facades.iter().filter(|facade| !facade.has_git()) {
        output.line(format!("- {}", facade.name));
    }
}

pub(crate) fn run_path(root: impl Into<std::path::PathBuf>, output: Output) -> Result<()> {
    let root = root.into();
    let root = fs::canonicalize(&root).unwrap_or(root);
    let facades = discover_facades(&root)?;
    let summary = ScanSummary::from_facades(&facades);

    output.line(format!("RustUse root scan - root: {}", root.display()));
    output.line(format!("found {} use-* directories", facades.len()));
    output.line(format!("facade repos with .git: {}", summary.repo_count));
    output.line(format!(
        "facades missing .git: {}",
        summary.missing_git_count
    ));
    output.line(format!(
        "child crates detected: {}",
        summary.child_crate_count
    ));
    output.line("");

    print_facade_table(&facades, output);

    if summary.missing_git_count > 0 {
        print_missing_git_facades(&facades, output);
    }

    output.line("");
    output.line(format!("status: {}", summary.status()));

    Ok(())
}
