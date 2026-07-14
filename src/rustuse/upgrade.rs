//! Plans and performs upgrades of the installed RustUse CLI.

use anyhow::Result;

use crate::rustuse::adapter::cargo::install::{CargoInstallOptions, install_cargo_package};

const PACKAGE_NAME: &str = "rustuse-cli";

/// Options controlling a RustUse CLI upgrade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UpgradeOptions {
    /// Whether to plan the upgrade without executing Cargo.
    pub(crate) dry_run: bool,
}

/// Result of planning or performing a RustUse CLI upgrade.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UpgradeOutcome {
    /// Final workflow status.
    pub(crate) status: UpgradeStatus,

    /// Cargo command associated with the upgrade.
    pub(crate) command: String,
}

/// Final status of a RustUse CLI upgrade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UpgradeStatus {
    /// The upgrade was planned but not executed.
    Planned,

    /// The upgrade completed successfully.
    Completed,
}

/// Plans or performs an upgrade of the installed RustUse CLI.
pub(crate) fn run(options: UpgradeOptions) -> Result<UpgradeOutcome> {
    let install_options = CargoInstallOptions {
        package: PACKAGE_NAME,
        force: true,
        locked: true,
    };

    let command = install_options.display_command();

    if options.dry_run {
        return Ok(UpgradeOutcome {
            status: UpgradeStatus::Planned,
            command,
        });
    }

    install_cargo_package(install_options)?;

    Ok(UpgradeOutcome {
        status: UpgradeStatus::Completed,
        command,
    })
}

#[cfg(test)]
mod tests {
    use super::{UpgradeOptions, UpgradeStatus, run};

    #[test]
    fn dry_run_returns_the_planned_upgrade() {
        let outcome =
            run(UpgradeOptions { dry_run: true }).expect("dry-run upgrade should succeed");

        assert_eq!(outcome.status, UpgradeStatus::Planned);
        assert_eq!(
            outcome.command,
            "cargo install rustuse-cli --force --locked"
        );
    }
}
