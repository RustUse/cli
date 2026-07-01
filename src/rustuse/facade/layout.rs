//! Shared RustUse repository layout standards.
//!
//! This module defines common paths, files, and directory names used across
//! RustUse repositories. It should not inspect the filesystem, render reports,
//! mutate files, or run commands.

/// Simple standard path entry used by scan/report code.
///
/// Tuple shape:
/// - `.0` = path
/// - `.1` = human-readable description
pub type StandardPath = (&'static str, &'static str);

// -----------------------------------------------------------------------------
// Common directory names
// -----------------------------------------------------------------------------

pub const CARGO_CONFIG_DIR: &str = ".cargo";
pub const GITHUB_DIR: &str = ".github";
pub const GITLAB_DIR: &str = ".gitlab";
pub const DEVCONTAINER_DIR: &str = ".devcontainer";
pub const VSCODE_DIR: &str = ".vscode";

pub const CRATES_DIR: &str = "crates";
pub const SRC_DIR: &str = "src";
pub const TESTS_DIR: &str = "tests";
pub const EXAMPLES_DIR: &str = "examples";
pub const DOCS_DIR: &str = "docs";
pub const TARGET_DIR: &str = "target";
pub const SCRIPTS_DIR: &str = "scripts";

// -----------------------------------------------------------------------------
// Common root file names
// -----------------------------------------------------------------------------

pub const CARGO_MANIFEST: &str = "Cargo.toml";
pub const CARGO_LOCK: &str = "Cargo.lock";
pub const RUST_TOOLCHAIN: &str = "rust-toolchain.toml";

pub const README: &str = "README.md";
pub const CHANGELOG: &str = "CHANGELOG.md";
pub const CONTRIBUTING: &str = "CONTRIBUTING.md";
pub const GOVERNANCE: &str = "GOVERNANCE.md";
pub const MAINTAINERS: &str = "MAINTAINERS.md";
pub const RELEASE: &str = "RELEASE.md";
pub const RELEASING: &str = "RELEASING.md";

pub const LICENSE: &str = "LICENSE";
pub const LICENSE_APACHE: &str = "LICENSE-APACHE";
pub const LICENSE_MIT: &str = "LICENSE-MIT";

pub const CLIPPY_CONFIG: &str = ".clippy.toml";
pub const EDITORCONFIG: &str = ".editorconfig";
pub const GITATTRIBUTES: &str = ".gitattributes";
pub const GITIGNORE: &str = ".gitignore";
pub const GITLEAKS_CONFIG: &str = ".gitleaks.toml";
pub const MARKDOWNLINTIGNORE: &str = ".markdownlintignore";
pub const RUSTFMT_CONFIG: &str = ".rustfmt.toml";
pub const TAPLO_CONFIG: &str = ".taplo.toml";
pub const TRIVYIGNORE: &str = ".trivyignore";

pub const DENY_CONFIG: &str = "deny.toml";
pub const RELEASE_PLZ_CONFIG: &str = "release-plz.toml";
pub const GITLAB_CI_CONFIG: &str = ".gitlab-ci.yml";
pub const MAKEFILE: &str = "Makefile";

// -----------------------------------------------------------------------------
// Common GitHub workflow paths
// -----------------------------------------------------------------------------

// pub const GITHUB_WORKFLOWS: &str = ".github/workflows";
/* pub const GITHUB_CI_WORKFLOW: &str = ".github/workflows/ci.yml";
pub const GITHUB_CODEQL_WORKFLOW: &str = ".github/workflows/codeql.yml";
pub const GITHUB_RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
pub const GITHUB_TRIVY_WORKFLOW: &str = ".github/workflows/trivy.yml"; */

// -----------------------------------------------------------------------------
// Common standard file groups
// -----------------------------------------------------------------------------

/* /// Standard formatting, editor, and repository hygiene files.
pub const STANDARD_TOOLING_FILES: &[StandardPath] = &[
    (CLIPPY_CONFIG, "Clippy configuration"),
    (EDITORCONFIG, "EditorConfig"),
    (GITATTRIBUTES, "Git attributes"),
    (GITIGNORE, "Git ignore rules"),
    (MARKDOWNLINTIGNORE, "Markdownlint ignore rules"),
    (RUSTFMT_CONFIG, "Rustfmt configuration"),
    (TAPLO_CONFIG, "Taplo configuration"),
];

/// Standard security and dependency policy files.
pub const STANDARD_SECURITY_FILES: &[StandardPath] = &[
    (DENY_CONFIG, "cargo-deny configuration"),
    (GITLEAKS_CONFIG, "Gitleaks configuration"),
    (TRIVYIGNORE, "Trivy ignore rules"),
];

/// Standard license files.
pub const STANDARD_LICENSE_FILES: &[StandardPath] = &[
    (LICENSE, "Repository license summary"),
    (LICENSE_APACHE, "Apache license"),
    (LICENSE_MIT, "MIT license"),
];

/// Standard documentation and governance files.
pub const STANDARD_DOCUMENTATION_FILES: &[StandardPath] = &[
    (README, "README"),
    (CHANGELOG, "Changelog"),
    (CONTRIBUTING, "Contribution guide"),
    (GOVERNANCE, "Governance"),
    (MAINTAINERS, "Maintainers"),
    (RELEASE, "Release notes"),
    (RELEASING, "Release process"),
];

/// Standard Rust/Cargo root files.
pub const STANDARD_RUST_FILES: &[StandardPath] = &[
    (CARGO_MANIFEST, "Cargo manifest"),
    (CARGO_LOCK, "Cargo lockfile"),
    (RUST_TOOLCHAIN, "Rust toolchain pin"),
];

/// Standard release automation files.
pub const STANDARD_RELEASE_FILES: &[StandardPath] =
    &[(RELEASE_PLZ_CONFIG, "release-plz configuration")];

/// Optional local/developer environment directories.
pub const OPTIONAL_DEVELOPMENT_DIRECTORIES: &[StandardPath] = &[
    (DEVCONTAINER_DIR, "Dev container"),
    (GITLAB_DIR, "GitLab configuration"),
];

/// Directories that usually should not be treated as committed RustUse standards.
pub const LOCAL_OR_GENERATED_DIRECTORIES: &[StandardPath] = &[
    (TARGET_DIR, "Cargo build output"),
    (VSCODE_DIR, "Local editor state"),
];

/// Generated files that should usually be ignored unless explicitly requested.
pub const GENERATED_REPORT_FILES: &[StandardPath] = &[
    ("rustuse-report.md", "Generated RustUse report"),
    ("sbom.cyclonedx.json", "Generated SBOM"),
    ("sbom.cyclonedx.xml", "Generated SBOM"),
];

/// Standard GitHub workflow files.
pub const STANDARD_GITHUB_WORKFLOW_FILES: &[StandardPath] = &[
    (GITHUB_CI_WORKFLOW, "GitHub CI workflow"),
    (GITHUB_CODEQL_WORKFLOW, "GitHub CodeQL workflow"),
    (GITHUB_RELEASE_WORKFLOW, "GitHub release workflow"),
    (GITHUB_TRIVY_WORKFLOW, "GitHub Trivy workflow"),
];

/// Core directories common to RustUse Rust repositories.
pub const STANDARD_RUST_REPOSITORY_DIRECTORIES: &[StandardPath] = &[
    (SRC_DIR, "Rust source directory"),
    (TESTS_DIR, "Integration tests"),
];

/// Core directories common to RustUse facade repositories.
pub const STANDARD_FACADE_DIRECTORIES: &[StandardPath] = &[
    (CARGO_CONFIG_DIR, "Cargo configuration"),
    (GITHUB_DIR, "GitHub configuration"),
    (CRATES_DIR, "Crate workspace members"),
];

/// Optional source-adjacent directories.
pub const OPTIONAL_RUST_SOURCE_DIRECTORIES: &[StandardPath] =
    &[(EXAMPLES_DIR, "Examples"), (DOCS_DIR, "Documentation")];
 */
