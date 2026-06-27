//! Shared tooling configuration surface checks for RustUse repositories.

use std::path::Path;

use crate::commands::dev::utils::report::PresenceCheck;

#[derive(Debug)]
pub(crate) struct ToolingSurfaceReport {
    pub(crate) surface: Vec<PresenceCheck>,
}

impl ToolingSurfaceReport {
    pub(crate) fn total_count(&self) -> usize {
        self.surface.len()
    }

    pub(crate) fn present_count(&self) -> usize {
        self.surface.iter().filter(|check| check.present).count()
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.surface.iter().all(|check| check.present) {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn inspect_tooling_surface(root: &Path) -> ToolingSurfaceReport {
    let surface = vec![
        PresenceCheck::new(".cargo", dir_exists(root, ".cargo")),
        PresenceCheck::new(".clippy.toml", file_exists(root, ".clippy.toml")),
        PresenceCheck::new(".rustfmt.toml", file_exists(root, ".rustfmt.toml")),
        PresenceCheck::new(".taplo.toml", file_exists(root, ".taplo.toml")),
        PresenceCheck::new("deny.toml", file_exists(root, "deny.toml")),
        PresenceCheck::new(".gitleaks.toml", file_exists(root, ".gitleaks.toml")),
        PresenceCheck::new(".trivyignore", file_exists(root, ".trivyignore")),
        PresenceCheck::new(
            "rust-toolchain.toml",
            file_exists(root, "rust-toolchain.toml"),
        ),
    ];

    ToolingSurfaceReport { surface }
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}

fn dir_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_dir()
}
