#![allow(dead_code)]

//! Standard RustUse `.github` policy.

/* use crate::dev::github::model::{GithubFileKind, RequiredGithubFile};
use crate::dev::github::{
    DEPENDABOT_RELATIVE_PATH, FUNDING_RELATIVE_PATH, ISSUE_TEMPLATE_DIR_RELATIVE_PATH,
    PULL_REQUEST_TEMPLATE_RELATIVE_PATH,
}; */

use super::model::{GithubFileKind, RequiredGithubFile};
/* use super::{
    DEPENDABOT_RELATIVE_PATH, FUNDING_RELATIVE_PATH, ISSUE_TEMPLATE_DIR_RELATIVE_PATH,
    PULL_REQUEST_TEMPLATE_RELATIVE_PATH,
}; */

use super::{DEPENDABOT_RELATIVE_PATH, PULL_REQUEST_TEMPLATE_RELATIVE_PATH};

/// Standard RustUse CI workflow path.
pub const CI_WORKFLOW_RELATIVE_PATH: &str = ".github/workflows/ci.yml";

/// Standard RustUse release-plz workflow path.
pub const RELEASE_PLZ_WORKFLOW_RELATIVE_PATH: &str = ".github/workflows/release-plz.yml";

/// Standard RustUse security workflow path.
pub const SECURITY_WORKFLOW_RELATIVE_PATH: &str = ".github/workflows/security.yml";

/// Standard bug report issue template path.
pub const BUG_REPORT_TEMPLATE_RELATIVE_PATH: &str = ".github/ISSUE_TEMPLATE/bug_report.yml";

/// Standard feature request issue template path.
pub const FEATURE_REQUEST_TEMPLATE_RELATIVE_PATH: &str =
    ".github/ISSUE_TEMPLATE/feature_request.yml";

/// Standard `.github` policy for RustUse facade repositories.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubPolicy {
    /// Required template-backed files.
    pub required_files: &'static [RequiredGithubFile],

    /// Whether `.github/ISSUE_TEMPLATE/` must exist.
    pub requires_issue_template_directory: bool,
}

/// Returns the standard RustUse `.github` policy.
#[must_use]
pub const fn standard_github_policy() -> GithubPolicy {
    GithubPolicy {
        required_files: REQUIRED_GITHUB_FILES,
        requires_issue_template_directory: true,
    }
}

/// Required `.github` files for every RustUse facade repository.
pub const REQUIRED_GITHUB_FILES: &[RequiredGithubFile] = &[
    RequiredGithubFile {
        relative_path: CI_WORKFLOW_RELATIVE_PATH,
        contents: CI_WORKFLOW,
        kind: GithubFileKind::Workflow,
    },
    RequiredGithubFile {
        relative_path: RELEASE_PLZ_WORKFLOW_RELATIVE_PATH,
        contents: RELEASE_PLZ_WORKFLOW,
        kind: GithubFileKind::Workflow,
    },
    RequiredGithubFile {
        relative_path: SECURITY_WORKFLOW_RELATIVE_PATH,
        contents: SECURITY_WORKFLOW,
        kind: GithubFileKind::Workflow,
    },
    RequiredGithubFile {
        relative_path: DEPENDABOT_RELATIVE_PATH,
        contents: DEPENDABOT,
        kind: GithubFileKind::Dependabot,
    },
    // RequiredGithubFile {
    //     relative_path: FUNDING_RELATIVE_PATH,
    //     contents: FUNDING,
    //     kind: GithubFileKind::Funding,
    // },
    RequiredGithubFile {
        relative_path: BUG_REPORT_TEMPLATE_RELATIVE_PATH,
        contents: BUG_REPORT_TEMPLATE,
        kind: GithubFileKind::IssueTemplate,
    },
    RequiredGithubFile {
        relative_path: FEATURE_REQUEST_TEMPLATE_RELATIVE_PATH,
        contents: FEATURE_REQUEST_TEMPLATE,
        kind: GithubFileKind::IssueTemplate,
    },
    RequiredGithubFile {
        relative_path: PULL_REQUEST_TEMPLATE_RELATIVE_PATH,
        contents: PULL_REQUEST_TEMPLATE,
        kind: GithubFileKind::PullRequestTemplate,
    },
];

/// Standard RustUse CI workflow.
pub const CI_WORKFLOW: &str = r#"name: CI

on:
  pull_request:
  push:
    branches:
      - main

permissions:
  contents: read

env:
  CARGO_TERM_COLOR: always

jobs:
  ci:
    name: CI
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache Cargo
        uses: Swatinem/rust-cache@v2

      - name: Format
        run: cargo fmt --all --check

      - name: Check
        run: cargo check --workspace --all-targets --all-features

      - name: Clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings

      - name: Test
        run: cargo test --workspace --all-features

      - name: Docs
        run: cargo doc --workspace --all-features --no-deps
"#;

/// Standard RustUse release-plz workflow.
pub const RELEASE_PLZ_WORKFLOW: &str = r#"name: Release

on:
  push:
    branches:
      - main

permissions:
  contents: write
  pull-requests: write

concurrency:
  group: release-plz-${{ github.ref }}
  cancel-in-progress: false

jobs:
  release-plz:
    name: Release
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo
        uses: Swatinem/rust-cache@v2

      - name: Release
        uses: release-plz/action@v0.5
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
"#;

/// Standard RustUse security workflow.
pub const SECURITY_WORKFLOW: &str = r#"name: Security

on:
  pull_request:
  push:
    branches:
      - main
  schedule:
    - cron: "0 8 * * 1"

permissions:
  contents: read

env:
  CARGO_TERM_COLOR: always

jobs:
  cargo-deny:
    name: cargo-deny
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo
        uses: Swatinem/rust-cache@v2

      - name: Install cargo-deny
        run: cargo install cargo-deny --locked

      - name: Run cargo-deny
        run: cargo deny check
"#;

/// Standard RustUse Dependabot configuration.
pub const DEPENDABOT: &str = r#"version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: weekly
    open-pull-requests-limit: 10

  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: weekly
    open-pull-requests-limit: 10
"#;

/// Standard RustUse funding metadata.
///
/// Empty by default until RustUse chooses a funding surface.
pub const FUNDING: &str = r#"# github: RustUse
"#;

/// Standard RustUse bug report issue template.
pub const BUG_REPORT_TEMPLATE: &str = r#"name: Bug report
description: Report a reproducible problem.
title: "bug: "
labels:
  - bug
body:
  - type: markdown
    attributes:
      value: |
        Thanks for reporting a RustUse issue. Please include a minimal reproduction when possible.

  - type: textarea
    id: description
    attributes:
      label: Description
      description: What happened?
    validations:
      required: true

  - type: textarea
    id: reproduction
    attributes:
      label: Reproduction
      description: Provide the smallest example that reproduces the issue.
      render: rust
    validations:
      required: false

  - type: input
    id: rust-version
    attributes:
      label: Rust version
      placeholder: rustc 1.95.0
    validations:
      required: false

  - type: input
    id: crate-version
    attributes:
      label: Crate version
      placeholder: use-example 0.1.0
    validations:
      required: false
"#;

/// Standard RustUse feature request issue template.
pub const FEATURE_REQUEST_TEMPLATE: &str = r#"name: Feature request
description: Suggest an improvement or new primitive.
title: "feat: "
labels:
  - enhancement
body:
  - type: markdown
    attributes:
      value: |
        RustUse favors small, composable, well-documented primitives.

  - type: textarea
    id: problem
    attributes:
      label: Problem
      description: What problem should this solve?
    validations:
      required: true

  - type: textarea
    id: proposal
    attributes:
      label: Proposal
      description: What API, crate, or behavior do you suggest?
    validations:
      required: true

  - type: textarea
    id: alternatives
    attributes:
      label: Alternatives
      description: What alternatives did you consider?
    validations:
      required: false
"#;

/// Standard RustUse pull request template.
pub const PULL_REQUEST_TEMPLATE: &str = r#"## Summary

Describe the change.

## Checklist

- [ ] Tests were added or updated when useful.
- [ ] Documentation was added or updated when useful.
- [ ] Public API changes are intentional.
- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test --workspace --all-features` passes.
"#;
