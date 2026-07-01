//! Facade repository surface inspection.
//!
//! This module inspects one `use-*` facade repository. Root-level scan/report
//! logic should call this module for each discovered facade instead of owning
//! duplicate repository-surface rules.

use std::path::Path;

use crate::rustuse::facade::layout::{
    CARGO_CONFIG_DIR, CARGO_LOCK, CARGO_MANIFEST, CHANGELOG, CLIPPY_CONFIG, CONTRIBUTING,
    CRATES_DIR, DENY_CONFIG, DEVCONTAINER_DIR, EDITORCONFIG, GITATTRIBUTES, GITHUB_DIR, GITIGNORE,
    GITLAB_CI_CONFIG, GITLAB_DIR, GITLEAKS_CONFIG, GOVERNANCE, LICENSE, LICENSE_APACHE,
    LICENSE_MIT, MAINTAINERS, MAKEFILE, MARKDOWNLINTIGNORE, README, RELEASE, RELEASE_PLZ_CONFIG,
    RELEASING, RUST_TOOLCHAIN, RUSTFMT_CONFIG, SCRIPTS_DIR, StandardPath, TAPLO_CONFIG, TARGET_DIR,
    TRIVYIGNORE, VSCODE_DIR,
};

use super::discover::FacadeInfo;

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

const FACADE_REQUIRED_ROOT_FILES: &[StandardPath] = &[
    (CLIPPY_CONFIG, "Clippy configuration"),
    (EDITORCONFIG, "EditorConfig"),
    (GITATTRIBUTES, "Git attributes"),
    (GITIGNORE, "Git ignore rules"),
    (GITLEAKS_CONFIG, "Gitleaks configuration"),
    (MARKDOWNLINTIGNORE, "Markdownlint ignore rules"),
    (RUSTFMT_CONFIG, "Rustfmt configuration"),
    (TAPLO_CONFIG, "Taplo configuration"),
    (TRIVYIGNORE, "Trivy ignore rules"),
    (CARGO_LOCK, "Cargo lockfile"),
    (CARGO_MANIFEST, "Workspace manifest"),
    (CHANGELOG, "Changelog"),
    (CONTRIBUTING, "Contribution guide"),
    (DENY_CONFIG, "cargo-deny configuration"),
    (GOVERNANCE, "Governance"),
    (LICENSE, "Repository license summary"),
    (LICENSE_APACHE, "Apache license"),
    (LICENSE_MIT, "MIT license"),
    (MAINTAINERS, "Maintainers"),
    (README, "Facade README"),
    (RELEASE_PLZ_CONFIG, "release-plz configuration"),
    (RELEASE, "Release notes"),
    (RELEASING, "Release process"),
    (RUST_TOOLCHAIN, "Rust toolchain pin"),
];

const FACADE_OPTIONAL_ROOT_FILES: &[StandardPath] =
    &[(GITLAB_CI_CONFIG, "GitLab CI"), (MAKEFILE, "Make targets")];

const FACADE_REQUIRED_DIRECTORIES: &[StandardPath] = &[
    (CARGO_CONFIG_DIR, "Cargo configuration"),
    (GITHUB_DIR, "GitHub configuration"),
    (CRATES_DIR, "Crate workspace members"),
];

const FACADE_OPTIONAL_DIRECTORIES: &[StandardPath] = &[
    (DEVCONTAINER_DIR, "Dev container"),
    (GITLAB_DIR, "GitLab configuration"),
];

pub(crate) const FACADE_DISCOURAGED_DIRECTORIES: &[StandardPath] = &[
    (VSCODE_DIR, "Local editor state"),
    (
        SCRIPTS_DIR,
        "Per-facade scripts; prefer rustuse-cli or GitHub Actions",
    ),
    (TARGET_DIR, "Cargo build output"),
];

pub(crate) const FACADE_DISCOURAGED_ROOT_FILES: &[StandardPath] = &[
    ("rustuse-report.md", "Generated RustUse report"),
    ("sbom.cyclonedx.json", "Generated SBOM"),
    ("sbom.cyclonedx.xml", "Generated SBOM"),
];

const FACADE_SURFACE_PROFILE: SurfaceProfile = SurfaceProfile {
    required_files: FACADE_REQUIRED_ROOT_FILES,
    optional_files: FACADE_OPTIONAL_ROOT_FILES,
    required_directories: FACADE_REQUIRED_DIRECTORIES,
    optional_directories: FACADE_OPTIONAL_DIRECTORIES,
};

pub(crate) type FacadeRepositoryReport = RepositorySurfaceReport;

pub(crate) fn inspect_facade_repository(facade: &FacadeInfo) -> FacadeRepositoryReport {
    inspect_repository_surface(&facade.root, &FACADE_SURFACE_PROFILE)
}

pub(crate) fn inspect_facade_path(root: &Path) -> FacadeRepositoryReport {
    inspect_repository_surface(root, &FACADE_SURFACE_PROFILE)
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
