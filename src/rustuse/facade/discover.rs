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
