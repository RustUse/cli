//! Data model for `.github` consistency checks.

use std::path::{Path, PathBuf};

use super::issue::{GithubIssue, GithubIssueSeverity};

/// Options used when checking a facade repository's `.github` directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubCheckOptions {
    /// Sort issues before returning the report.
    pub sort_issues: bool,

    /// Include successful file checks in rendered reports.
    pub include_ok_checks: bool,
}

impl Default for GithubCheckOptions {
    fn default() -> Self {
        Self {
            sort_issues: true,
            include_ok_checks: true,
        }
    }
}

/// A checked `.github` target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubTarget {
    /// Facade name, usually the directory name such as `use-quant`.
    pub facade: String,

    /// Facade repository path.
    pub path: PathBuf,
}

impl GithubTarget {
    /// Creates a target from a facade repository path.
    #[must_use]
    pub fn from_facade_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();

        let facade = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(".")
            .to_owned();

        Self { facade, path }
    }
}

/// Full `.github` check report for one facade repository.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubCheckReport {
    /// Checked target.
    pub target: GithubTarget,

    /// Facade repository path.
    pub facade_path: PathBuf,

    /// Individual file checks.
    pub checks: Vec<GithubFileCheck>,

    /// Issues discovered while checking the target.
    pub issues: Vec<GithubIssue>,
}

impl GithubCheckReport {
    /// Returns true when there are no warning or error issues.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.error_count() == 0 && self.warning_count() == 0
    }

    /// Returns true when the report has one or more error issues.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Number of error issues.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == GithubIssueSeverity::Error)
            .count()
    }

    /// Number of warning issues.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == GithubIssueSeverity::Warning)
            .count()
    }

    /// Number of informational issues.
    #[must_use]
    pub fn info_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == GithubIssueSeverity::Info)
            .count()
    }

    /// Number of file checks with status `ok`.
    #[must_use]
    pub fn ok_check_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == GithubFileStatus::Ok)
            .count()
    }

    /// Number of missing required files.
    #[must_use]
    pub fn missing_check_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == GithubFileStatus::Missing)
            .count()
    }

    /// Number of stale required files.
    #[must_use]
    pub fn stale_check_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == GithubFileStatus::Stale)
            .count()
    }

    /// Number of invalid required paths.
    #[must_use]
    pub fn invalid_kind_check_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == GithubFileStatus::InvalidKind)
            .count()
    }

    /// Sorts issues by severity, path, then code.
    pub fn sort_issues(&mut self) {
        self.issues.sort_by(|left, right| {
            left.severity
                .cmp(&right.severity)
                .then_with(|| left.path.cmp(&right.path))
                .then_with(|| left.code.cmp(&right.code))
        });
    }
}

/// Result of checking one required `.github` file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubFileCheck {
    /// Repository-relative path, such as `.github/workflows/ci.yml`.
    pub relative_path: &'static str,

    /// File check status.
    pub status: GithubFileStatus,

    /// Expected content hash when the file is template-backed.
    pub expected_hash: Option<String>,

    /// Actual content hash when the file exists and is readable.
    pub actual_hash: Option<String>,
}

impl GithubFileCheck {
    /// Returns true when the file matches the expected RustUse standard.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status == GithubFileStatus::Ok
    }

    /// Returns true when the file is missing.
    #[must_use]
    pub fn is_missing(&self) -> bool {
        self.status == GithubFileStatus::Missing
    }

    /// Returns true when the file exists but differs from the expected template.
    #[must_use]
    pub fn is_stale(&self) -> bool {
        self.status == GithubFileStatus::Stale
    }

    /// Returns true when the path exists but is not a normal file.
    #[must_use]
    pub fn is_invalid_kind(&self) -> bool {
        self.status == GithubFileStatus::InvalidKind
    }
}

/// Status for a required `.github` file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GithubFileStatus {
    /// File exists and matches the expected template.
    Ok,

    /// File does not exist.
    Missing,

    /// File exists but differs from the expected template.
    Stale,

    /// Path exists but is not a file.
    InvalidKind,
}

impl GithubFileStatus {
    /// User-facing status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::InvalidKind => "invalid-kind",
        }
    }
}

/// Kind of `.github` file being checked.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GithubFileKind {
    /// GitHub Actions workflow under `.github/workflows/`.
    Workflow,

    /// Dependabot configuration.
    Dependabot,

    /// GitHub funding metadata.
    Funding,

    /// GitHub issue template.
    IssueTemplate,

    /// GitHub pull request template.
    PullRequestTemplate,
}

impl GithubFileKind {
    /// User-facing kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workflow => "workflow",
            Self::Dependabot => "dependabot",
            Self::Funding => "funding",
            Self::IssueTemplate => "issue-template",
            Self::PullRequestTemplate => "pull-request-template",
        }
    }
}

/// One required file in the RustUse `.github` policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequiredGithubFile {
    /// Repository-relative path.
    pub relative_path: &'static str,

    /// Expected file contents.
    pub contents: &'static str,

    /// Required file kind.
    pub kind: GithubFileKind,
}
