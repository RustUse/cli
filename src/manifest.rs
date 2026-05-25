use std::path::{Path, PathBuf};

pub const LOCK_FILE: &str = "rustuse.lock";

#[must_use]
pub fn lock_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join(LOCK_FILE)
}

#[must_use]
pub fn lock_exists(root: impl AsRef<Path>) -> bool {
    lock_path(root).is_file()
}
