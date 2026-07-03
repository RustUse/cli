use std::path::PathBuf;

pub(crate) const DEFAULT_REPORT_FILE_NAME: &str = "rustuse-report.md";

/// Where a generated Markdown report should be delivered.
#[derive(Clone, Debug)]
pub(crate) enum ReportDestination {
    /// Write the report to a file.
    ///
    /// `None` means use the default report path for the inspected root.
    File(Option<PathBuf>),

    /// Print the report to standard output.
    Stdout,
}

impl ReportDestination {
    pub(crate) fn from_output(stdout: bool, output: Option<PathBuf>) -> Self {
        if stdout {
            Self::Stdout
        } else {
            Self::File(output)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PresenceCheck {
    pub(crate) path: String,
    pub(crate) present: bool,
}

impl PresenceCheck {
    pub(crate) fn new(path: impl Into<String>, present: bool) -> Self {
        Self {
            path: path.into(),
            present,
        }
    }
}
