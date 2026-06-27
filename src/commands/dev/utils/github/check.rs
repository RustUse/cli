#![allow(dead_code)]
/* //! Check `.github` directory consistency for RustUse facade repositories.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::commands::dev::github::hash::hash_bytes;
use crate::commands::dev::github::issue::{GithubIssue, GithubIssueCode, GithubIssueSeverity};
use crate::commands::dev::github::model::{
    GithubCheckOptions, GithubCheckReport, GithubFileCheck, GithubFileKind, GithubFileStatus,
    GithubTarget,
};
use crate::commands::dev::github::policy::{GithubPolicy, standard_github_policy};
use crate::commands::dev::github::{
    GITHUB_DIR_NAME, ISSUE_TEMPLATE_DIR_RELATIVE_PATH, WORKFLOWS_DIR_RELATIVE_PATH,
}; */

//! Check `.github` directory consistency for RustUse facade repositories.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::hash::hash_bytes;
use super::issue::{GithubIssue, GithubIssueCode, GithubIssueSeverity};
use super::model::{
    GithubCheckOptions, GithubCheckReport, GithubFileCheck, GithubFileKind, GithubFileStatus,
    GithubTarget,
};
use super::policy::{GithubPolicy, standard_github_policy};
use super::{GITHUB_DIR_NAME, ISSUE_TEMPLATE_DIR_RELATIVE_PATH, WORKFLOWS_DIR_RELATIVE_PATH};

/// Checks a single facade repository against the standard RustUse `.github` policy.
pub fn check_facade(path: impl AsRef<Path>) -> Result<GithubCheckReport> {
    check_facade_with_options(path, GithubCheckOptions::default())
}

/// Checks a single facade repository against the standard RustUse `.github` policy.
pub fn check_facade_with_options(
    path: impl AsRef<Path>,
    options: GithubCheckOptions,
) -> Result<GithubCheckReport> {
    let facade_path = path.as_ref().to_path_buf();
    let policy = standard_github_policy();

    check_facade_with_policy(facade_path, &policy, options)
}

/// Checks a single facade repository against a provided `.github` policy.
pub fn check_facade_with_policy(
    facade_path: PathBuf,
    policy: &GithubPolicy,
    options: GithubCheckOptions,
) -> Result<GithubCheckReport> {
    let target = GithubTarget::from_facade_path(&facade_path);
    let github_dir = facade_path.join(GITHUB_DIR_NAME);
    let workflows_dir = facade_path.join(WORKFLOWS_DIR_RELATIVE_PATH);
    let issue_template_dir = facade_path.join(ISSUE_TEMPLATE_DIR_RELATIVE_PATH);

    let mut checks = Vec::new();
    let mut issues = Vec::new();

    if !github_dir.exists() {
        issues.push(GithubIssue::new(
            GithubIssueSeverity::Warning,
            GithubIssueCode::MissingGithubDirectory,
            GITHUB_DIR_NAME,
            "Facade repository is missing `.github/`.",
        ));
    } else if !github_dir.is_dir() {
        issues.push(GithubIssue::new(
            GithubIssueSeverity::Error,
            GithubIssueCode::InvalidGithubDirectory,
            GITHUB_DIR_NAME,
            "Expected `.github/` to be a directory.",
        ));
    }

    if !workflows_dir.exists() {
        issues.push(GithubIssue::new(
            GithubIssueSeverity::Warning,
            GithubIssueCode::MissingGithubWorkflowsDirectory,
            WORKFLOWS_DIR_RELATIVE_PATH,
            "Facade repository is missing `.github/workflows/`.",
        ));
    } else if !workflows_dir.is_dir() {
        issues.push(GithubIssue::new(
            GithubIssueSeverity::Error,
            GithubIssueCode::InvalidGithubWorkflowsDirectory,
            WORKFLOWS_DIR_RELATIVE_PATH,
            "Expected `.github/workflows/` to be a directory.",
        ));
    }

    if policy.requires_issue_template_directory && !issue_template_dir.exists() {
        issues.push(GithubIssue::new(
            GithubIssueSeverity::Warning,
            GithubIssueCode::MissingGithubIssueTemplateDirectory,
            ISSUE_TEMPLATE_DIR_RELATIVE_PATH,
            "Facade repository is missing `.github/ISSUE_TEMPLATE/`.",
        ));
    }

    /* for required_file in &policy.required_files {
        let check = check_required_file(
            &facade_path,
            required_file.relative_path,
            required_file.contents,
        )
        .with_context(|| {
            format!(
                "failed to check required `.github` file `{}`",
                required_file.relative_path
            )
        })?;

        if !check.is_ok() {
            issues.push(issue_for_file_check(&check, required_file.kind));
        }

        checks.push(check);
    } */

    for required_file in policy.required_files {
        let check = check_required_file(
            &facade_path,
            required_file.relative_path,
            required_file.contents,
        )
        .with_context(|| {
            format!(
                "failed to check required `.github` file `{}`",
                required_file.relative_path
            )
        })?;

        if !check.is_ok() {
            issues.push(issue_for_file_check(&check, required_file.kind));
        }

        checks.push(check);
    }

    let mut report = GithubCheckReport {
        target,
        facade_path,
        checks,
        issues,
    };

    if options.sort_issues {
        report.sort_issues();
    }

    Ok(report)
}

fn check_required_file(
    facade_path: &Path,
    relative_path: &'static str,
    expected_contents: &'static str,
) -> Result<GithubFileCheck> {
    let path = facade_path.join(relative_path);

    if !path.exists() {
        return Ok(GithubFileCheck {
            relative_path,
            status: GithubFileStatus::Missing,
            expected_hash: Some(hash_bytes(expected_contents.as_bytes())),
            actual_hash: None,
        });
    }

    if !path.is_file() {
        return Ok(GithubFileCheck {
            relative_path,
            status: GithubFileStatus::InvalidKind,
            expected_hash: Some(hash_bytes(expected_contents.as_bytes())),
            actual_hash: None,
        });
    }

    let actual_contents = fs::read(&path)
        .with_context(|| format!("failed to read `.github` file `{}`", path.display()))?;

    let expected_hash = hash_bytes(expected_contents.as_bytes());
    let actual_hash = hash_bytes(&actual_contents);

    let status = if expected_hash == actual_hash {
        GithubFileStatus::Ok
    } else {
        GithubFileStatus::Stale
    };

    Ok(GithubFileCheck {
        relative_path,
        status,
        expected_hash: Some(expected_hash),
        actual_hash: Some(actual_hash),
    })
}

fn issue_for_file_check(check: &GithubFileCheck, kind: GithubFileKind) -> GithubIssue {
    match check.status {
        GithubFileStatus::Ok => GithubIssue::new(
            GithubIssueSeverity::Info,
            GithubIssueCode::Ok,
            check.relative_path,
            "File is consistent with the RustUse `.github` policy.",
        ),
        GithubFileStatus::Missing => GithubIssue::new(
            GithubIssueSeverity::Warning,
            missing_code_for_kind(kind),
            check.relative_path,
            "Required `.github` file is missing.",
        ),
        GithubFileStatus::Stale => GithubIssue::new(
            GithubIssueSeverity::Warning,
            stale_code_for_kind(kind),
            check.relative_path,
            "Required `.github` file differs from the RustUse standard template.",
        ),
        GithubFileStatus::InvalidKind => GithubIssue::new(
            GithubIssueSeverity::Error,
            invalid_kind_code_for_kind(kind),
            check.relative_path,
            "Expected `.github` path to be a file.",
        ),
    }
}

fn missing_code_for_kind(kind: GithubFileKind) -> GithubIssueCode {
    match kind {
        GithubFileKind::Workflow => GithubIssueCode::MissingGithubWorkflow,
        GithubFileKind::Dependabot => GithubIssueCode::MissingGithubDependabot,
        GithubFileKind::Funding => GithubIssueCode::MissingGithubFunding,
        GithubFileKind::IssueTemplate => GithubIssueCode::MissingGithubIssueTemplate,
        GithubFileKind::PullRequestTemplate => GithubIssueCode::MissingGithubPullRequestTemplate,
    }
}

fn stale_code_for_kind(kind: GithubFileKind) -> GithubIssueCode {
    match kind {
        GithubFileKind::Workflow => GithubIssueCode::StaleGithubWorkflow,
        GithubFileKind::Dependabot => GithubIssueCode::StaleGithubDependabot,
        GithubFileKind::Funding => GithubIssueCode::StaleGithubFunding,
        GithubFileKind::IssueTemplate => GithubIssueCode::StaleGithubIssueTemplate,
        GithubFileKind::PullRequestTemplate => GithubIssueCode::StaleGithubPullRequestTemplate,
    }
}

fn invalid_kind_code_for_kind(kind: GithubFileKind) -> GithubIssueCode {
    match kind {
        GithubFileKind::Workflow => GithubIssueCode::InvalidGithubWorkflow,
        GithubFileKind::Dependabot => GithubIssueCode::InvalidGithubDependabot,
        GithubFileKind::Funding => GithubIssueCode::InvalidGithubFunding,
        GithubFileKind::IssueTemplate => GithubIssueCode::InvalidGithubIssueTemplate,
        GithubFileKind::PullRequestTemplate => GithubIssueCode::InvalidGithubPullRequestTemplate,
    }
}
