use std::path::{Path, PathBuf};

pub(crate) const DEFAULT_REPORT_FILE_NAME: &str = "rustuse-report.md";

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

pub(crate) fn report_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

pub(crate) fn default_report_path(root: &Path) -> PathBuf {
    root.join(DEFAULT_REPORT_FILE_NAME)
}
