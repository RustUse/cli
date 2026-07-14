use std::fmt;
use std::path::PathBuf;

use super::codes::{FacadeIssueBucket, FacadeIssueCode};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum FacadeIssueSeverity {
    Error,
    Warning,
}

impl FacadeIssueSeverity {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }

    pub(crate) const fn sort_rank(self) -> u8 {
        match self {
            Self::Error => 0,
            Self::Warning => 1,
        }
    }
}

impl fmt::Display for FacadeIssueSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FacadeIssue {
    pub(crate) severity: FacadeIssueSeverity,
    pub(crate) code: FacadeIssueCode,
    pub(crate) path: Option<PathBuf>,
    pub(crate) message: String,
    pub(crate) fix: Option<FacadeFixKind>,
}

impl FacadeIssue {
    pub(crate) fn new(
        severity: FacadeIssueSeverity,
        code: FacadeIssueCode,
        path: Option<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            path,
            message: message.into(),
            fix: None,
        }
    }

    pub(crate) fn error(
        code: FacadeIssueCode,
        path: Option<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(FacadeIssueSeverity::Error, code, path, message)
    }

    pub(crate) fn warning(
        code: FacadeIssueCode,
        path: Option<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(FacadeIssueSeverity::Warning, code, path, message)
    }

    pub(crate) const fn bucket(&self) -> FacadeIssueBucket {
        self.code.bucket()
    }

    #[must_use]
    pub(crate) fn with_fix(mut self, fix: FacadeFixKind) -> Self {
        self.fix = Some(fix);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum FacadeFixKind {
    AddWorkspaceLints,
    RestoreStandardFile,
    RestoreStandardDirectory,
    RestoreGithubWorkflow,
    RemoveNonStandardPath,
}
