//! Discovery helpers for a local `RustUse` fleet.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

const FLEET_REPOS: &[&str] = &[
    ".github",
    ".github-private",
    "cli",
    "docs",
    "infra",
    "mcp",
    "rustuse",
];

const GIT_DIR_NAME: &str = ".git";
const CARGO_MANIFEST_FILE_NAME: &str = "Cargo.toml";
const CRATES_DIR_NAME: &str = "crates";

#[derive(Debug, Clone)]
pub(crate) struct FleetRepoEntry {
    pub(crate) name: &'static str,
    pub(crate) present: bool,
    pub(crate) has_git: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct FleetFacadeEntry {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) has_git: bool,
    pub(crate) has_cargo_toml: bool,
    pub(crate) has_crates_dir: bool,
    pub(crate) child_crate_count: usize,
}

impl FleetFacadeEntry {
    pub(crate) fn status(&self) -> &'static str {
        if self.has_git && self.has_cargo_toml && self.has_crates_dir && self.version.is_some() {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn discover_fleet_repos(fleet: &Path) -> Vec<FleetRepoEntry> {
    FLEET_REPOS
        .iter()
        .map(|name| discover_fleet_repo(fleet, name))
        .collect()
}

fn discover_fleet_repo(fleet: &Path, name: &'static str) -> FleetRepoEntry {
    let path = fleet.join(name);

    FleetRepoEntry {
        name,
        present: path.is_dir(),
        has_git: path.join(GIT_DIR_NAME).is_dir(),
    }
}

pub(crate) fn discover_facades(fleet: &Path) -> Result<Vec<FleetFacadeEntry>> {
    let mut facades = Vec::new();

    for entry in fs::read_dir(fleet)
        .with_context(|| format!("failed to read fleet directory `{}`", fleet.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !name.starts_with("use-") {
            continue;
        }

        facades.push(discover_facade_entry(&path, name)?);
    }

    facades.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(facades)
}

fn discover_facade_entry(path: &Path, name: &str) -> Result<FleetFacadeEntry> {
    let crates_dir = path.join(CRATES_DIR_NAME);

    Ok(FleetFacadeEntry {
        name: name.to_owned(),
        version: read_facade_crate_version(path, name)?,
        has_git: path.join(GIT_DIR_NAME).is_dir(),
        has_cargo_toml: path.join(CARGO_MANIFEST_FILE_NAME).is_file(),
        has_crates_dir: crates_dir.is_dir(),
        child_crate_count: count_child_crates(&crates_dir)?,
    })
}

pub(crate) fn display_version(version: &Option<String>) -> &str {
    version.as_deref().unwrap_or("<missing>")
}

pub(crate) fn resolve_existing_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = fs::canonicalize(path) {
        return canonical;
    }

    if path.is_absolute() {
        return path.to_path_buf();
    }

    std::env::current_dir()
        .map(|current_dir| current_dir.join(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

fn read_facade_crate_version(facade_root: &Path, facade_name: &str) -> Result<Option<String>> {
    let root_manifest_path = facade_root.join(CARGO_MANIFEST_FILE_NAME);
    let facade_manifest_path = facade_root
        .join(CRATES_DIR_NAME)
        .join(facade_name)
        .join(CARGO_MANIFEST_FILE_NAME);

    if !facade_manifest_path.is_file() {
        return Ok(None);
    }

    let facade_manifest = read_toml_value(&facade_manifest_path)?;

    if let Some(version) = facade_manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(toml::Value::as_str)
    {
        return Ok(Some(version.to_owned()));
    }

    let uses_workspace_version = facade_manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(|version| version.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    if uses_workspace_version && root_manifest_path.is_file() {
        let root_manifest = read_toml_value(&root_manifest_path)?;

        if let Some(version) = root_manifest
            .get("workspace")
            .and_then(|workspace| workspace.get("package"))
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
        {
            return Ok(Some(version.to_owned()));
        }
    }

    Ok(None)
}

fn read_toml_value(path: &Path) -> Result<toml::Value> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read `{}`", path.display()))?;

    toml::from_str(&raw).with_context(|| format!("failed to parse `{}`", path.display()))
}

fn count_child_crates(crates_dir: &Path) -> Result<usize> {
    if !crates_dir.is_dir() {
        return Ok(0);
    }

    let mut count = 0usize;

    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.join(CARGO_MANIFEST_FILE_NAME).is_file() {
            count += 1;
        }
    }

    Ok(count)
}
