//! Facade repository discovery helpers.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub(crate) use super::model::FacadeInfo;

pub(crate) fn discover_facade(root: impl AsRef<Path>) -> Result<FacadeInfo> {
    let input_root = root.as_ref();
    let root = fs::canonicalize(input_root).unwrap_or_else(|_| input_root.to_path_buf());

    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
        .to_owned();

    let crates_dir = root.join("crates");
    let crate_manifest_paths = discover_crate_manifest_paths(&crates_dir)?;

    Ok(FacadeInfo::new(root, name, crate_manifest_paths))
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
