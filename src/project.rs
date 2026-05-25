use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::manifest;

pub const CONFIG_FILE: &str = "rustuse.toml";
pub const CARGO_FILE: &str = "Cargo.toml";
pub const STATE_DIR: &str = ".rustuse";
pub const CACHE_DIR: &str = ".rustuse/cache";
pub const SNAPSHOTS_DIR: &str = ".rustuse/snapshots";

#[derive(Clone, Debug)]
pub struct ProjectState {
    pub cargo_toml_path: PathBuf,
    pub config_path: PathBuf,
    pub lock_path: PathBuf,
    pub state_dir_path: PathBuf,
    pub cache_dir_path: PathBuf,
    pub snapshots_dir_path: PathBuf,
    pub has_cargo_toml: bool,
    pub has_config: bool,
    pub has_lock: bool,
    pub has_state_dir: bool,
    pub has_cache_dir: bool,
    pub has_snapshots_dir: bool,
}

pub fn current_state() -> Result<ProjectState> {
    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    Ok(detect(current_dir))
}

pub fn detect(root: impl AsRef<Path>) -> ProjectState {
    let root = root.as_ref();
    let cargo_toml_path = root.join(CARGO_FILE);
    let config_path = config_path(root);
    let lock_path = manifest::lock_path(root);
    let state_dir_path = root.join(STATE_DIR);
    let cache_dir_path = root.join(CACHE_DIR);
    let snapshots_dir_path = root.join(SNAPSHOTS_DIR);

    ProjectState {
        has_cargo_toml: cargo_toml_path.is_file(),
        has_config: config_path.is_file(),
        has_lock: manifest::lock_exists(root),
        has_state_dir: state_dir_path.is_dir(),
        has_cache_dir: cache_dir_path.is_dir(),
        has_snapshots_dir: snapshots_dir_path.is_dir(),
        cargo_toml_path,
        config_path,
        lock_path,
        state_dir_path,
        cache_dir_path,
        snapshots_dir_path,
    }
}

#[must_use]
pub fn config_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join(CONFIG_FILE)
}

pub fn create_tracking_dirs(root: impl AsRef<Path>) -> Result<()> {
    let root = root.as_ref();
    fs::create_dir_all(root.join(CACHE_DIR))
        .with_context(|| format!("failed to create {}", root.join(CACHE_DIR).display()))?;
    fs::create_dir_all(root.join(SNAPSHOTS_DIR))
        .with_context(|| format!("failed to create {}", root.join(SNAPSHOTS_DIR).display()))?;
    Ok(())
}

pub fn write_config_new(root: impl AsRef<Path>, contents: &str) -> Result<()> {
    let path = config_path(root);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(contents.as_bytes())
        .with_context(|| format!("failed to write {}", path.display()))
}
