//! Facade repository surface inspection.

use crate::commands::dev::utils::scan::{
    RepositorySurfaceReport, SurfaceProfile, inspect_repository_surface,
};

use super::discover::FacadeInfo;

const REQUIRED_ROOT_FILES: &[(&str, &str)] = &[
    (".clippy.toml", "Clippy configuration"),
    (".editorconfig", "EditorConfig"),
    (".gitattributes", "Git attributes"),
    (".gitignore", "Git ignore rules"),
    (".gitleaks.toml", "Gitleaks configuration"),
    (".rustfmt.toml", "Rustfmt configuration"),
    (".taplo.toml", "Taplo configuration"),
    (".trivyignore", "Trivy ignore rules"),
    ("Cargo.lock", "Cargo lockfile"),
    ("Cargo.toml", "Workspace manifest"),
    ("CHANGELOG.md", "Changelog"),
    ("CONTRIBUTING.md", "Contribution guide"),
    ("deny.toml", "cargo-deny configuration"),
    ("GOVERNANCE.md", "Governance"),
    ("LICENSE-APACHE", "Apache license"),
    ("LICENSE-MIT", "MIT license"),
    ("MAINTAINERS.md", "Maintainers"),
    ("Makefile", "Make targets"),
    ("README.md", "Facade README"),
    ("release-plz.toml", "release-plz configuration"),
    ("RELEASE.md", "Release notes"),
    ("RELEASING.md", "Release process"),
    ("rust-toolchain.toml", "Rust toolchain pin"),
];

const OPTIONAL_ROOT_FILES: &[(&str, &str)] = &[(".gitlab-ci.yml", "GitLab CI")];

const REQUIRED_DIRECTORIES: &[(&str, &str)] = &[
    (".cargo", "Cargo configuration"),
    (".github", "GitHub configuration"),
    ("crates", "Crate workspace members"),
];

const OPTIONAL_DIRECTORIES: &[(&str, &str)] = &[
    (".devcontainer", "Dev container"),
    (".gitlab", "GitLab configuration"),
    (".vscode", "VS Code workspace settings"),
    ("scripts", "Repository scripts"),
];

const FACADE_SURFACE_PROFILE: SurfaceProfile = SurfaceProfile {
    required_files: REQUIRED_ROOT_FILES,
    optional_files: OPTIONAL_ROOT_FILES,
    required_directories: REQUIRED_DIRECTORIES,
    optional_directories: OPTIONAL_DIRECTORIES,
};

pub(crate) type FacadeRepositoryReport = RepositorySurfaceReport;

pub(crate) fn inspect_facade_repository(facade: &FacadeInfo) -> FacadeRepositoryReport {
    inspect_repository_surface(&facade.root, &FACADE_SURFACE_PROFILE)
}
