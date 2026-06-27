#![allow(dead_code)]

//! `.github` directory inspection, reporting, and repair.
//!
//! This module owns RustUse's standard GitHub repository metadata for each
//! facade repository. It intentionally covers the whole `.github/` directory,
//! not only GitHub Actions workflows.
//!
//! Command adapters such as `dev root report` and future `dev facade ...`
//! commands should stay thin and delegate GitHub-specific logic here.
//!

use std::path::Path;

use anyhow::{Context, Result};

use crate::commands::dev::root::discover::FacadeEntry;

pub(crate) mod check;
// pub(crate) mod fix;
pub(crate) mod hash;
pub(crate) mod issue;
pub(crate) mod model;
pub(crate) mod policy;
pub(crate) mod report;
pub(crate) mod workflows;

/// Directory name expected at the root of every facade repository.
pub(crate) const GITHUB_DIR_NAME: &str = ".github";

/// Relative path to the GitHub workflows directory.
pub(crate) const WORKFLOWS_DIR_RELATIVE_PATH: &str = ".github/workflows";

/// Relative path to Dependabot configuration.
pub(crate) const DEPENDABOT_RELATIVE_PATH: &str = ".github/dependabot.yml";

/// Relative path to the GitHub funding file.
pub(crate) const FUNDING_RELATIVE_PATH: &str = ".github/FUNDING.yml";

/// Relative path to the pull request template.
pub(crate) const PULL_REQUEST_TEMPLATE_RELATIVE_PATH: &str = ".github/pull_request_template.md";

/// Relative path to the issue template directory.
pub(crate) const ISSUE_TEMPLATE_DIR_RELATIVE_PATH: &str = ".github/ISSUE_TEMPLATE";

#[derive(Debug)]
pub(crate) struct RootGithubReport {
    pub(crate) markdown: String,
    pub(crate) facade_count: usize,
    pub(crate) clean_count: usize,
    pub(crate) warning_count: usize,
    pub(crate) error_count: usize,
}

impl RootGithubReport {
    pub(crate) fn status(&self) -> &'static str {
        if self.error_count > 0 {
            "error"
        } else if self.warning_count > 0 {
            "warning"
        } else {
            "ok"
        }
    }
}

pub(crate) fn analyze_root_github(
    root: &Path,
    facades: &[FacadeEntry],
) -> Result<RootGithubReport> {
    let mut markdown = String::new();

    markdown.push_str("# RustUse GitHub Report\n\n");
    markdown.push_str(&format!("- Root: `{}`\n", root.display()));
    markdown.push_str(&format!("- Facades inspected: `{}`\n\n", facades.len()));

    let mut clean_count = 0usize;
    let mut warning_count = 0usize;
    let mut error_count = 0usize;

    for facade in facades {
        let facade_path = root.join(&facade.name);

        let facade_report = check::check_facade(&facade_path)
            .with_context(|| format!("failed to check GitHub metadata for `{}`", facade.name))?;

        if facade_report.has_errors() {
            error_count += 1;
        } else if facade_report.warning_count() > 0 {
            warning_count += 1;
        } else {
            clean_count += 1;
        }

        markdown.push_str(&report::render_markdown(&facade_report));
        markdown.push('\n');
    }

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!("- Clean: `{clean_count}`\n"));
    markdown.push_str(&format!("- Warning: `{warning_count}`\n"));
    markdown.push_str(&format!("- Error: `{error_count}`\n"));

    Ok(RootGithubReport {
        markdown,
        facade_count: facades.len(),
        clean_count,
        warning_count,
        error_count,
    })
}
