//! Issue model for `.github` consistency checks.

use std::fmt;

/// Severity for a `.github` consistency issue.
///
/// The variant order is intentional so default sorting places errors first.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GithubIssueSeverity {
    /// A publish or CI/CD blocker.
    Error,

    /// Standardization drift that should be fixed.
    Warning,

    /// Informational issue.
    Info,
}

impl GithubIssueSeverity {
    /// User-facing severity label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

impl fmt::Display for GithubIssueSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable issue code for a `.github` consistency issue.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GithubIssueCode {
    /// No issue; used only for successful check records when needed.
    Ok,

    /// The facade repository is missing `.github/`.
    MissingGithubDirectory,

    /// `.github/` exists but is not a directory.
    InvalidGithubDirectory,

    /// The facade repository is missing `.github/workflows/`.
    MissingGithubWorkflowsDirectory,

    /// `.github/workflows/` exists but is not a directory.
    InvalidGithubWorkflowsDirectory,

    /// The facade repository is missing `.github/ISSUE_TEMPLATE/`.
    MissingGithubIssueTemplateDirectory,

    /// A required GitHub Actions workflow is missing.
    MissingGithubWorkflow,

    /// A required GitHub Actions workflow differs from the RustUse standard.
    StaleGithubWorkflow,

    /// A GitHub Actions workflow path exists but is not a file.
    InvalidGithubWorkflow,

    /// `.github/dependabot.yml` is missing.
    MissingGithubDependabot,

    /// `.github/dependabot.yml` differs from the RustUse standard.
    StaleGithubDependabot,

    /// `.github/dependabot.yml` exists but is not a file.
    InvalidGithubDependabot,

    /// `.github/FUNDING.yml` is missing.
    MissingGithubFunding,

    /// `.github/FUNDING.yml` differs from the RustUse standard.
    StaleGithubFunding,

    /// `.github/FUNDING.yml` exists but is not a file.
    InvalidGithubFunding,

    /// A required issue template is missing.
    MissingGithubIssueTemplate,

    /// A required issue template differs from the RustUse standard.
    StaleGithubIssueTemplate,

    /// An issue template path exists but is not a file.
    InvalidGithubIssueTemplate,

    /// `.github/pull_request_template.md` is missing.
    MissingGithubPullRequestTemplate,

    /// `.github/pull_request_template.md` differs from the RustUse standard.
    StaleGithubPullRequestTemplate,

    /// `.github/pull_request_template.md` exists but is not a file.
    InvalidGithubPullRequestTemplate,
}

impl GithubIssueCode {
    /// Stable kebab-case issue code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::MissingGithubDirectory => "missing-github-directory",
            Self::InvalidGithubDirectory => "invalid-github-directory",
            Self::MissingGithubWorkflowsDirectory => "missing-github-workflows-directory",
            Self::InvalidGithubWorkflowsDirectory => "invalid-github-workflows-directory",
            Self::MissingGithubIssueTemplateDirectory => "missing-github-issue-template-directory",
            Self::MissingGithubWorkflow => "missing-github-workflow",
            Self::StaleGithubWorkflow => "stale-github-workflow",
            Self::InvalidGithubWorkflow => "invalid-github-workflow",
            Self::MissingGithubDependabot => "missing-github-dependabot",
            Self::StaleGithubDependabot => "stale-github-dependabot",
            Self::InvalidGithubDependabot => "invalid-github-dependabot",
            Self::MissingGithubFunding => "missing-github-funding",
            Self::StaleGithubFunding => "stale-github-funding",
            Self::InvalidGithubFunding => "invalid-github-funding",
            Self::MissingGithubIssueTemplate => "missing-github-issue-template",
            Self::StaleGithubIssueTemplate => "stale-github-issue-template",
            Self::InvalidGithubIssueTemplate => "invalid-github-issue-template",
            Self::MissingGithubPullRequestTemplate => "missing-github-pr-template",
            Self::StaleGithubPullRequestTemplate => "stale-github-pr-template",
            Self::InvalidGithubPullRequestTemplate => "invalid-github-pr-template",
        }
    }
}

impl fmt::Display for GithubIssueCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One `.github` consistency issue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubIssue {
    /// Issue severity.
    pub severity: GithubIssueSeverity,

    /// Stable issue code.
    pub code: GithubIssueCode,

    /// Repository-relative path associated with the issue.
    pub path: String,

    /// Human-readable issue message.
    pub message: String,
}

impl GithubIssue {
    /// Creates a new `.github` consistency issue.
    #[must_use]
    pub fn new(
        severity: GithubIssueSeverity,
        code: GithubIssueCode,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            path: path.into(),
            message: message.into(),
        }
    }

    /// Returns true when this issue is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, GithubIssueSeverity::Error)
    }

    /// Returns true when this issue is a warning.
    #[must_use]
    pub const fn is_warning(&self) -> bool {
        matches!(self.severity, GithubIssueSeverity::Warning)
    }

    /// Returns true when this issue is informational.
    #[must_use]
    pub const fn is_info(&self) -> bool {
        matches!(self.severity, GithubIssueSeverity::Info)
    }
}

#[cfg(test)]
mod tests {
    use super::{GithubIssue, GithubIssueCode, GithubIssueSeverity};

    #[test]
    fn severity_labels_are_stable() {
        assert_eq!(GithubIssueSeverity::Error.as_str(), "error");
        assert_eq!(GithubIssueSeverity::Warning.as_str(), "warning");
        assert_eq!(GithubIssueSeverity::Info.as_str(), "info");
    }

    #[test]
    fn issue_code_labels_are_stable() {
        assert_eq!(
            GithubIssueCode::MissingGithubWorkflow.as_str(),
            "missing-github-workflow"
        );
        assert_eq!(
            GithubIssueCode::StaleGithubPullRequestTemplate.as_str(),
            "stale-github-pr-template"
        );
    }

    #[test]
    fn issue_helpers_match_severity() {
        let issue = GithubIssue::new(
            GithubIssueSeverity::Error,
            GithubIssueCode::InvalidGithubDirectory,
            ".github",
            "Expected `.github/` to be a directory.",
        );

        assert!(issue.is_error());
        assert!(!issue.is_warning());
        assert!(!issue.is_info());
    }
}
