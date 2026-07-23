//! Data models used by facade repair planning and application.

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use crate::rustuse::facade::codes::FacadeIssueCode;

/// A supported group of facade manifest repairs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FacadeFixGroup {
    FacadeWiring,
    WorkspaceShape,
    WorkspaceDependencies,
    PackageMetadata,
}

impl FacadeFixGroup {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::FacadeWiring => "facade-wiring",
            Self::WorkspaceShape => "workspace-shape",
            Self::WorkspaceDependencies => "workspace-dependencies",
            Self::PackageMetadata => "package-metadata",
        }
    }
}

impl fmt::Display for FacadeFixGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FacadeFixGroup {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "facade-wiring" => Ok(Self::FacadeWiring),
            "workspace-shape" => Ok(Self::WorkspaceShape),
            "workspace-dependencies" => Ok(Self::WorkspaceDependencies),
            "package-metadata" => Ok(Self::PackageMetadata),
            _ => Err(()),
        }
    }
}

/// A diagnostic issue or repair group selected for facade repair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FacadeFixTarget {
    Issue(FacadeIssueCode),
    Group(FacadeFixGroup),
}

impl FromStr for FacadeFixTarget {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();

        if value.is_empty() {
            return Err("facade fix code cannot be empty".to_owned());
        }

        if value == "all" {
            return Err("use `--all` instead of `--code all`".to_owned());
        }

        if let Ok(group) = value.parse::<FacadeFixGroup>() {
            return Ok(Self::Group(group));
        }

        FacadeIssueCode::from_id(value)
            .map(Self::Issue)
            .ok_or_else(|| format!("unknown facade fix issue code or group `{value}`"))
    }
}

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
    /// Issue codes or repair groups to target.
    ///
    /// When empty, every supported facade repair is planned.
    pub(crate) targets: Vec<FacadeFixTarget>,

    /// Whether to preview or apply the planned changes.
    pub(crate) mode: FixMode,
}

impl FacadeFixOptions {
    pub(crate) const fn new(mode: FixMode) -> Self {
        Self {
            targets: Vec::new(),
            mode,
        }
    }

    #[must_use]
    pub(crate) fn with_target(mut self, target: FacadeFixTarget) -> Self {
        if !self.targets.contains(&target) {
            self.targets.push(target);
        }

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{FacadeFixGroup, FacadeFixOptions, FacadeFixTarget, FixMode};
    use crate::rustuse::facade::codes::FacadeIssueCode;

    #[test]
    fn parses_fix_group() {
        assert_eq!(
            FacadeFixTarget::from_str("workspace-shape"),
            Ok(FacadeFixTarget::Group(FacadeFixGroup::WorkspaceShape))
        );
    }

    #[test]
    fn parses_issue_code() {
        assert_eq!(
            FacadeFixTarget::from_str("missing-workspace-members"),
            Ok(FacadeFixTarget::Issue(
                FacadeIssueCode::MissingWorkspaceMembers
            ))
        );
    }

    #[test]
    fn trims_fix_target_input() {
        assert_eq!(
            FacadeFixTarget::from_str("  package-metadata  "),
            Ok(FacadeFixTarget::Group(FacadeFixGroup::PackageMetadata))
        );
    }

    #[test]
    fn rejects_empty_fix_target() {
        assert_eq!(
            FacadeFixTarget::from_str("   "),
            Err("facade fix code cannot be empty".to_owned())
        );
    }

    #[test]
    fn rejects_all_as_a_code() {
        assert_eq!(
            FacadeFixTarget::from_str("all"),
            Err("use `--all` instead of `--code all`".to_owned())
        );
    }

    #[test]
    fn rejects_unknown_fix_target() {
        assert_eq!(
            FacadeFixTarget::from_str("unknown-target"),
            Err("unknown facade fix issue code or group `unknown-target`".to_owned())
        );
    }

    #[test]
    fn with_target_ignores_duplicates() {
        let target = FacadeFixTarget::Group(FacadeFixGroup::FacadeWiring);

        let options = FacadeFixOptions::new(FixMode::DryRun)
            .with_target(target)
            .with_target(target);

        assert_eq!(options.targets, vec![target]);
    }

    #[test]
    fn fix_group_display_uses_canonical_id() {
        assert_eq!(
            FacadeFixGroup::WorkspaceDependencies.to_string(),
            "workspace-dependencies"
        );
    }
}
