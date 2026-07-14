//! Data types used by the RustUse CLI upgrade workflow.

/// Options controlling a RustUse CLI upgrade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UpgradeOptions {
    /// Whether to return the planned operation without executing it.
    pub dry_run: bool,
}

/// Result of planning or performing a RustUse CLI upgrade.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UpgradeOutcome {
    /// Final state of the upgrade workflow.
    pub status: UpgradeStatus,

    /// Cargo command associated with the upgrade.
    pub command: String,
}

/// Final state of a RustUse CLI upgrade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UpgradeStatus {
    /// The command was planned but not executed.
    Planned,

    /// The upgrade command completed successfully.
    Completed,
}
