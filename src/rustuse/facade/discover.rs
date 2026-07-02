//! Facade repository discovery helpers.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub(crate) struct FacadeInfo {
    pub(crate) root: PathBuf,
    pub(crate) name: String,
    pub(crate) git_dir: PathBuf,
    pub(crate) manifest_path: PathBuf,
    pub(crate) crates_dir: PathBuf,
    pub(crate) crate_manifest_paths: Vec<PathBuf>,
}

impl FacadeInfo {
    pub(crate) fn has_git(&self) -> bool {
        self.git_dir.is_dir()
    }

    pub(crate) fn has_manifest(&self) -> bool {
        self.manifest_path.is_file()
    }

    pub(crate) fn has_crates_dir(&self) -> bool {
        self.crates_dir.is_dir()
    }

    pub(crate) fn crate_count(&self) -> usize {
        self.crate_manifest_paths.len()
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.has_git() && self.has_manifest() && self.has_crates_dir() {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn discover_facade(root: impl AsRef<Path>) -> Result<FacadeInfo> {
    let input_root = root.as_ref();
    let root = fs::canonicalize(input_root).unwrap_or_else(|_| input_root.to_path_buf());

    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
        .to_owned();

    let git_dir = root.join(".git");
    let manifest_path = root.join("Cargo.toml");
    let crates_dir = root.join("crates");
    let crate_manifest_paths = discover_crate_manifest_paths(&crates_dir)?;

    Ok(FacadeInfo {
        root,
        name,
        git_dir,
        manifest_path,
        crates_dir,
        crate_manifest_paths,
    })
}

fn discover_crate_manifest_paths(crates_dir: &Path) -> Result<Vec<PathBuf>> {
    if !crates_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();

    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let manifest_path = path.join("Cargo.toml");

            if manifest_path.is_file() {
                manifests.push(manifest_path);
            }
        }
    }

    manifests.sort();

    Ok(manifests)
}

// ! Discovery helpers for a local RustUse development root.

// use std::fs;
// use std::path::Path;

// use anyhow::{Context, Result};

const ROOT_REPOS: &[&str] = &[
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
pub(crate) struct RootRepoEntry {
    pub(crate) name: &'static str,
    pub(crate) present: bool,
    pub(crate) has_git: bool,
}

/* impl RootRepoEntry {
    pub(crate) fn present(&self) -> bool {
        self.present
    }

    pub(crate) fn has_git(&self) -> bool {
        self.has_git
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.present() && self.has_git() {
            "ok"
        } else {
            "warning"
        }
    }
} */

#[derive(Debug, Clone)]
pub(crate) struct FacadeEntry {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) has_git: bool,
    pub(crate) has_cargo_toml: bool,
    pub(crate) has_crates_dir: bool,
    pub(crate) child_crate_count: usize,
}

impl FacadeEntry {
    pub(crate) fn has_git(&self) -> bool {
        self.has_git
    }

    pub(crate) fn has_cargo_toml(&self) -> bool {
        self.has_cargo_toml
    }

    pub(crate) fn has_crates_dir(&self) -> bool {
        self.has_crates_dir
    }

    pub(crate) fn has_version(&self) -> bool {
        self.version.is_some()
    }

    pub(crate) fn child_crate_count(&self) -> usize {
        self.child_crate_count
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.has_git() && self.has_cargo_toml() && self.has_crates_dir() && self.has_version() {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn discover_root_repos(root: &Path) -> Vec<RootRepoEntry> {
    ROOT_REPOS
        .iter()
        .map(|name| discover_root_repo(root, name))
        .collect()
}

fn discover_root_repo(root: &Path, name: &'static str) -> RootRepoEntry {
    let path = root.join(name);

    RootRepoEntry {
        name,
        present: path.is_dir(),
        has_git: path.join(GIT_DIR_NAME).is_dir(),
    }
}

pub(crate) fn discover_facades(root: &Path) -> Result<Vec<FacadeEntry>> {
    let mut facades = Vec::new();

    for entry in fs::read_dir(root)
        .with_context(|| format!("failed to read root directory `{}`", root.display()))?
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

fn discover_facade_entry(path: &Path, name: &str) -> Result<FacadeEntry> {
    let crates_dir = path.join(CRATES_DIR_NAME);

    Ok(FacadeEntry {
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

/// Resolves `path` to a canonical, absolute path when possible.
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

/// Returns true when `path` looks like a `use-*` facade repository.
///
/// Shared by the `scan` and `report` commands so facade detection stays
/// consistent across workflows.
pub(crate) fn looks_like_facade(path: &Path) -> bool {
    let resolved = resolve_existing_path(path);

    has_facade_shape(&resolved)
        && (has_facade_directory_name(&resolved) || has_facade_package_name(&resolved))
}

fn has_facade_shape(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() && path.join("crates").is_dir()
}

fn has_facade_directory_name(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("use-"))
}

fn has_facade_package_name(path: &Path) -> bool {
    let manifest_path = path.join("Cargo.toml");

    let Ok(raw) = fs::read_to_string(manifest_path) else {
        return false;
    };

    let Ok(manifest) = toml::from_str::<toml::Value>(&raw) else {
        return false;
    };

    manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .is_some_and(|name| name.starts_with("use-"))
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
        .and_then(|version| version.as_str())
    {
        return Ok(Some(version.to_owned()));
    }

    let uses_workspace_version = facade_manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(|version| version.get("workspace"))
        .and_then(|workspace| workspace.as_bool())
        .unwrap_or(false);

    if uses_workspace_version && root_manifest_path.is_file() {
        let root_manifest = read_toml_value(&root_manifest_path)?;

        if let Some(version) = root_manifest
            .get("workspace")
            .and_then(|workspace| workspace.get("package"))
            .and_then(|package| package.get("version"))
            .and_then(|version| version.as_str())
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
