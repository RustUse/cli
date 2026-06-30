//! Cargo manifest reading and generic Cargo.toml helpers.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub(crate) const CARGO_MANIFEST_FILE: &str = "Cargo.toml";

pub(crate) fn manifest_path(root: &Path) -> PathBuf {
    root.join(CARGO_MANIFEST_FILE)
}

pub(crate) fn read_manifest(path: &Path) -> Result<toml::Value> {
    let raw = read_manifest_string(path)?;

    parse_manifest_str(path, &raw)
}

pub(crate) fn read_manifest_from_root(root: &Path) -> Result<toml::Value> {
    read_manifest(&manifest_path(root))
}

pub(crate) fn read_manifest_string(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read Cargo manifest `{}`", path.display()))
}

pub(crate) fn parse_manifest_str(path: &Path, raw: &str) -> Result<toml::Value> {
    toml::from_str(raw)
        .with_context(|| format!("failed to parse Cargo manifest `{}`", path.display()))
}

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

pub(crate) fn table<'a>(value: &'a toml::Value, key: &str) -> Option<&'a toml::Table> {
    value.get(key).and_then(toml::Value::as_table)
}

pub(crate) fn string_field<'a>(table: &'a toml::Table, key: &str) -> Option<&'a str> {
    table.get(key).and_then(toml::Value::as_str)
}

pub(crate) fn bool_field(table: &toml::Table, key: &str) -> Option<bool> {
    table.get(key).and_then(toml::Value::as_bool)
}

pub(crate) fn array_field<'a>(table: &'a toml::Table, key: &str) -> Option<&'a Vec<toml::Value>> {
    table.get(key).and_then(toml::Value::as_array)
}

pub(crate) fn package_table(manifest: &toml::Value) -> Option<&toml::Table> {
    table(manifest, "package")
}

pub(crate) fn workspace_table(manifest: &toml::Value) -> Option<&toml::Table> {
    table(manifest, "workspace")
}

pub(crate) fn workspace_package_table(manifest: &toml::Value) -> Option<&toml::Table> {
    workspace_table(manifest)?
        .get("package")
        .and_then(toml::Value::as_table)
}

pub(crate) fn workspace_dependencies_table(manifest: &toml::Value) -> Option<&toml::Table> {
    workspace_table(manifest)?
        .get("dependencies")
        .and_then(toml::Value::as_table)
}

pub(crate) fn dependencies_table(manifest: &toml::Value) -> Option<&toml::Table> {
    table(manifest, "dependencies")
}

pub(crate) fn features_table(manifest: &toml::Value) -> Option<&toml::Table> {
    table(manifest, "features")
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

pub(crate) fn collect_string_array(value: &toml::Value) -> Option<Vec<String>> {
    value.as_array().map(|array| {
        array
            .iter()
            .filter_map(toml::Value::as_str)
            .map(ToOwned::to_owned)
            .collect()
    })
}

pub(crate) fn collect_string_array_strict(value: &toml::Value) -> Option<Vec<String>> {
    let array = value.as_array()?;

    if array.iter().all(|item| item.as_str().is_some()) {
        Some(
            array
                .iter()
                .filter_map(toml::Value::as_str)
                .map(ToOwned::to_owned)
                .collect(),
        )
    } else {
        None
    }
}

pub(crate) fn array_contains_string(array: &[toml::Value], expected: &str) -> bool {
    array
        .iter()
        .any(|value| value.as_str().is_some_and(|actual| actual == expected))
}
