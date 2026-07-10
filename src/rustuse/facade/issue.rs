use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Clone, Debug)]
pub(crate) struct FacadeIssue {
    pub(crate) severity: FacadeIssueSeverity,
    pub(crate) code: &'static str,
    pub(crate) bucket: &'static str,
    pub(crate) path: Option<PathBuf>,
    pub(crate) message: String,
    pub(crate) fix: Option<FacadeFixKind>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum FacadeFixKind {
    AddWorkspaceLints,
    RestoreStandardFile,
    RestoreStandardDirectory,
    RestoreGithubWorkflow,
    RemoveNonStandardPath,
}
