//! RustUse facade standards and vocabulary.
//!
//! This module defines what a RustUse facade repository is expected to look like.
//! It should not inspect the filesystem, mutate files, render reports, or run commands.

pub mod ci;
pub mod discover;
pub mod documentation;
pub mod fix;
pub mod flags;
pub mod inspect;
pub mod layout;
pub mod manifest;
pub mod model;
pub mod nonstandard;
pub mod policy;
pub mod release;
pub mod report;
pub mod scan;
pub mod standards;

use crate::rustuse::facade::layout::{
    CARGO_CONFIG_DIR, CARGO_LOCK, CARGO_MANIFEST, CHANGELOG, CLIPPY_CONFIG, CONTRIBUTING,
    CRATES_DIR, DENY_CONFIG, DEVCONTAINER_DIR, EDITORCONFIG, GITATTRIBUTES, GITHUB_DIR, GITIGNORE,
    GITLAB_CI_CONFIG, GITLAB_DIR, GITLEAKS_CONFIG, GOVERNANCE, LICENSE, LICENSE_APACHE,
    LICENSE_MIT, MAINTAINERS, MAKEFILE, MARKDOWNLINTIGNORE, README, RELEASE, RELEASE_PLZ_CONFIG,
    RELEASING, RUST_TOOLCHAIN, RUSTFMT_CONFIG, SCRIPTS_DIR, StandardPath, TAPLO_CONFIG, TARGET_DIR,
    TRIVYIGNORE, VSCODE_DIR,
};

/// Standard prefix for RustUse facade repositories and facade crates.
pub const FACADE_NAME_PREFIX: &str = "use-";

/// Standard directory containing child crates in a facade repository.
pub const FACADE_CRATES_DIR: &str = CRATES_DIR;

/// Standard workspace manifest path for a facade repository.
pub const FACADE_WORKSPACE_MANIFEST: &str = CARGO_MANIFEST;

/// Standard facade README path.
pub const FACADE_README: &str = README;

/// Required root-level files for a RustUse facade repository.
///
/// The tuple shape is intentionally simple so report/scan code can consume it
/// without needing to depend on richer RustUse-specific types.
pub const FACADE_REQUIRED_ROOT_FILES: &[StandardPath] = &[
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

/// Optional root-level files for a RustUse facade repository.
pub const FACADE_OPTIONAL_ROOT_FILES: &[StandardPath] =
    &[(GITLAB_CI_CONFIG, "GitLab CI"), (MAKEFILE, "Make targets")];

/// Required root-level directories for a RustUse facade repository.
pub const FACADE_REQUIRED_DIRECTORIES: &[StandardPath] = &[
    (CARGO_CONFIG_DIR, "Cargo configuration"),
    (GITHUB_DIR, "GitHub configuration"),
    (CRATES_DIR, "Crate workspace members"),
];

/// Optional root-level directories for a RustUse facade repository.
pub const FACADE_OPTIONAL_DIRECTORIES: &[StandardPath] = &[
    (DEVCONTAINER_DIR, "Dev container"),
    (GITLAB_DIR, "GitLab configuration"),
];

/// Directories that should generally not be committed to facade repositories.
///
/// These are not required or optional standards. They are useful for report
/// rules that want to flag local/generated/legacy surfaces.
pub const FACADE_DISCOURAGED_DIRECTORIES: &[StandardPath] = &[
    (VSCODE_DIR, "Local editor state"),
    (
        SCRIPTS_DIR,
        "Per-facade scripts; prefer rustuse-cli or GitHub Actions",
    ),
    (TARGET_DIR, "Cargo build output"),
];

/// Files that should generally not be committed to facade repositories.
pub const FACADE_DISCOURAGED_ROOT_FILES: &[StandardPath] = &[
    ("rustuse-report.md", "Generated RustUse report"),
    ("sbom.cyclonedx.json", "Generated SBOM"),
    ("sbom.cyclonedx.xml", "Generated SBOM"),
];

/// RustUse facade package role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FacadePackageKind {
    /// The root facade package that re-exports child crates.
    Facade,

    /// A child crate under `crates/`.
    ChildCrate,
}

/// Expected facade repository shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FacadeRepositoryStandard {
    /// Expected facade name prefix.
    pub name_prefix: &'static str,

    /// Expected child crate directory.
    pub crates_dir: &'static str,

    /// Expected workspace manifest path.
    pub workspace_manifest: &'static str,

    /// Required root files.
    pub required_root_files: &'static [StandardPath],

    /// Optional root files.
    pub optional_root_files: &'static [StandardPath],

    /// Required root directories.
    pub required_directories: &'static [StandardPath],

    /// Optional root directories.
    pub optional_directories: &'static [StandardPath],

    /// Discouraged root files.
    pub discouraged_root_files: &'static [StandardPath],

    /// Discouraged root directories.
    pub discouraged_directories: &'static [StandardPath],
}

/// The standard RustUse facade repository policy.
pub const FACADE_REPOSITORY_STANDARD: FacadeRepositoryStandard = FacadeRepositoryStandard {
    name_prefix: FACADE_NAME_PREFIX,
    crates_dir: FACADE_CRATES_DIR,
    workspace_manifest: FACADE_WORKSPACE_MANIFEST,
    required_root_files: FACADE_REQUIRED_ROOT_FILES,
    optional_root_files: FACADE_OPTIONAL_ROOT_FILES,
    required_directories: FACADE_REQUIRED_DIRECTORIES,
    optional_directories: FACADE_OPTIONAL_DIRECTORIES,
    discouraged_root_files: FACADE_DISCOURAGED_ROOT_FILES,
    discouraged_directories: FACADE_DISCOURAGED_DIRECTORIES,
};

/// Returns true when a repository or crate name uses the standard facade prefix.
#[must_use]
pub fn is_facade_name(name: &str) -> bool {
    name.starts_with(FACADE_NAME_PREFIX)
}
