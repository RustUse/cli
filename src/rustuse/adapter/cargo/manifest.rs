//! Cargo manifest reading and generic Cargo.toml helpers.

use std::fs;
use std::path::Path;

pub(crate) fn read_manifest_with_diagnostics(path: &Path) -> CargoManifestRead {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            return CargoManifestRead {
                value: None,
                diagnostic: Some(CargoManifestDiagnostic {
                    kind: CargoManifestDiagnosticKind::Read,
                    code: "read-manifest",
                    message: format!("failed to read `{}`: {error}", path.display()),
                }),
            };
        },
    };

    match toml::from_str(&raw) {
        Ok(value) => CargoManifestRead {
            value: Some(value),
            diagnostic: None,
        },
        Err(error) => CargoManifestRead {
            value: None,
            diagnostic: Some(CargoManifestDiagnostic {
                kind: CargoManifestDiagnosticKind::Parse,
                code: "parse-manifest",
                message: format!("failed to parse `{}`: {error}", path.display()),
            }),
        },
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CargoManifestRead {
    pub(crate) value: Option<toml::Value>,
    pub(crate) diagnostic: Option<CargoManifestDiagnostic>,
}

impl CargoManifestRead {
    pub(crate) fn into_value(self) -> Option<toml::Value> {
        self.value
    }

    pub(crate) fn diagnostic(&self) -> Option<&CargoManifestDiagnostic> {
        self.diagnostic.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CargoManifestDiagnostic {
    pub(crate) kind: CargoManifestDiagnosticKind,
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CargoManifestDiagnosticKind {
    Read,
    Parse,
}

pub(crate) fn workspace_true(value: &toml::Value) -> bool {
    value
        .as_table()
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
}

pub(crate) fn has_string_or_workspace_true(table: &toml::Table, key: &str) -> bool {
    table
        .get(key)
        .is_some_and(|value| value.as_str().is_some() || workspace_true(value))
}
