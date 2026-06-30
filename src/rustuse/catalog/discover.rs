use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use super::model::{CatalogEntry, DistributionMode};

pub fn discover_catalog() -> Result<Vec<CatalogEntry>> {
    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    let root = discover_rustuse_root(&current_dir)
        .context("failed to find RustUse root with use-* facade repositories")?;

    discover_catalog_from_root(&root)
}

pub fn discover_catalog_from_root(root: &Path) -> Result<Vec<CatalogEntry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(root).with_context(|| format!("failed to read {}", root.display()))? {
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

        entries.push(facade_entry(name));

        let crates_dir = path.join("crates");

        if crates_dir.is_dir() {
            discover_child_crates(name, &crates_dir, &mut entries)?;
        }
    }

    entries.sort_by(|left, right| left.name.cmp(&right.name));
    entries.dedup_by(|left, right| left.name == right.name);

    Ok(entries)
}

fn discover_rustuse_root(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        if looks_like_rustuse_root(ancestor) {
            return Some(ancestor.to_path_buf());
        }
    }

    None
}

fn looks_like_rustuse_root(path: &Path) -> bool {
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    let use_repo_count = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with("use-"))
        })
        .take(3)
        .count();

    use_repo_count >= 3
}

fn discover_child_crates(
    facade_name: &str,
    crates_dir: &Path,
    entries: &mut Vec<CatalogEntry>,
) -> Result<()> {
    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read {}", crates_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(crate_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !crate_name.starts_with("use-") {
            continue;
        }

        entries.push(child_entry(facade_name, crate_name));
    }

    Ok(())
}

fn facade_entry(name: &str) -> CatalogEntry {
    CatalogEntry {
        name: name.to_owned(),
        kind: "facade crate".to_owned(),
        set: name.to_owned(),
        docs_url: format!("https://rustuse.org/{name}/"),
        api_docs_url: format!("https://rustuse.org/api/{name}/"),
        workspace_docs_url: Some(format!("https://rustuse.org/api/workspaces/{name}/")),
        modes: vec![DistributionMode::Cargo, DistributionMode::Cli],
    }
}

fn child_entry(facade_name: &str, crate_name: &str) -> CatalogEntry {
    CatalogEntry {
        name: crate_name.to_owned(),
        kind: "child crate".to_owned(),
        set: facade_name.to_owned(),
        docs_url: format!("https://rustuse.org/{facade_name}/{crate_name}/"),
        api_docs_url: format!("https://rustuse.org/api/{crate_name}/"),
        workspace_docs_url: Some(format!("https://rustuse.org/api/workspaces/{facade_name}/")),
        modes: vec![
            DistributionMode::Cargo,
            DistributionMode::Copy,
            DistributionMode::Cli,
        ],
    }
}
