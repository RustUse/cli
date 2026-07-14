//! Data models used by facade repair planning and application.

use std::path::PathBuf;

/// Controls whether a facade repair plan is previewed or written.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FixMode {
    /// Calculate and report changes without modifying files.
    #[default]
    DryRun,

    /// Apply the calculated changes to the facade repository.
    Write,
}

impl FixMode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::DryRun => "dry-run",
            Self::Write => "write",
        }
    }

    pub(crate) const fn writes_files(self) -> bool {
        matches!(self, Self::Write)
    }
}

/// Options controlling repairs for one facade repository.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FacadeFixOptions {
    /// Optional diagnostic code or repair group to target.
    ///
    /// When absent, all supported facade repairs are planned.
    pub(crate) codes: Vec<String>,

    /// Whether to preview or apply the planned changes.
    pub(crate) mode: FixMode,
}

impl FacadeFixOptions {
    pub(crate) const fn new(mode: FixMode) -> Self {
        Self {
            codes: Vec::new(),
            mode,
        }
    }

    pub(crate) fn with_code(mut self, code: impl Into<String>) -> Self {
        self.codes.push(code.into());
        self
    }
}

/// A complete read-only repair plan for one facade repository.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FacadeFixPlan {
    /// Canonical root of the facade repository.
    pub(crate) root: PathBuf,

    /// Number of files inspected while building the plan.
    pub(crate) files_inspected: usize,

    /// Number of inspected files already matching the expected state.
    pub(crate) files_unchanged: usize,

    /// Files that differ from the expected state.
    pub(crate) changes: Vec<PlannedFileChange>,
}

impl FacadeFixPlan {
    pub(crate) fn files_changed(&self) -> usize {
        self.changes.len()
    }

    pub(crate) fn files_created(&self) -> usize {
        self.changes.iter().filter(|change| change.created).count()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

/// A file replacement proposed by a facade repair plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlannedFileChange {
    /// Destination path relative to the facade root.
    pub(crate) path: PathBuf,

    /// Complete contents that should be written to the destination.
    pub(crate) contents: String,

    /// Whether the destination did not exist when the plan was built.
    pub(crate) created: bool,
}

/// Result of previewing or applying a facade repair plan.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FacadeFixSummary {
    /// Number of files inspected while planning repairs.
    pub(crate) files_inspected: usize,

    /// Number of files that differ from the expected state.
    pub(crate) files_changed: usize,

    /// Number of inspected files already matching the expected state.
    pub(crate) files_unchanged: usize,

    /// Number of changed files that did not previously exist.
    pub(crate) files_created: usize,

    /// Individual planned or applied file changes.
    pub(crate) changes: Vec<FacadeFixChange>,
}

impl FacadeFixSummary {
    pub(crate) fn has_changes(&self) -> bool {
        self.files_changed != 0
    }

    pub(crate) fn files_written(&self) -> usize {
        self.changes.iter().filter(|change| change.wrote).count()
    }
}

/// Result for one file included in a facade repair operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FacadeFixChange {
    /// File path relative to the facade root.
    pub(crate) path: PathBuf,

    /// Whether the file did not exist when the plan was built.
    pub(crate) created: bool,

    /// Whether the file was written.
    ///
    /// This is `false` during a dry run.
    pub(crate) wrote: bool,
}
