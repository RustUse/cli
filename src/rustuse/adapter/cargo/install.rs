//! Cargo package installation adapter.

use std::process::Command;

use anyhow::{Context, Result, bail};

/// Options for installing a package through Cargo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CargoInstallOptions<'a> {
    /// Cargo package name to install.
    pub(crate) package: &'a str,

    /// Whether to replace an existing installation.
    pub(crate) force: bool,

    /// Whether to use the package's published lockfile.
    pub(crate) locked: bool,
}

impl CargoInstallOptions<'_> {
    /// Returns a printable representation of the Cargo command.
    pub(crate) fn display_command(self) -> String {
        let mut parts = vec![
            "cargo".to_owned(),
            "install".to_owned(),
            self.package.to_owned(),
        ];

        if self.force {
            parts.push("--force".to_owned());
        }

        if self.locked {
            parts.push("--locked".to_owned());
        }

        parts.join(" ")
    }
}

/// Installs a Cargo package using the supplied options.
pub(crate) fn install_cargo_package(options: CargoInstallOptions<'_>) -> Result<()> {
    let display_command = options.display_command();

    let mut command = Command::new("cargo");
    command.arg("install").arg(options.package);

    if options.force {
        command.arg("--force");
    }

    if options.locked {
        command.arg("--locked");
    }

    let status = command
        .status()
        .with_context(|| format!("failed to run `{display_command}`"))?;

    if !status.success() {
        bail!("`{display_command}` failed with status {status}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CargoInstallOptions;

    #[test]
    fn display_command_includes_enabled_options() {
        let options = CargoInstallOptions {
            package: "rustuse-cli",
            force: true,
            locked: true,
        };

        assert_eq!(
            options.display_command(),
            "cargo install rustuse-cli --force --locked"
        );
    }

    #[test]
    fn display_command_omits_disabled_options() {
        let options = CargoInstallOptions {
            package: "rustuse-cli",
            force: false,
            locked: false,
        };

        assert_eq!(options.display_command(), "cargo install rustuse-cli");
    }
}
