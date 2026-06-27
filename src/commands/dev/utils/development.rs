//! Shared development environment surface checks for RustUse repositories.

use std::path::Path;

use crate::commands::dev::utils::report::PresenceCheck;

#[derive(Debug)]
pub(crate) struct DevelopmentSurfaceReport {
    pub(crate) surface: Vec<PresenceCheck>,
}

impl DevelopmentSurfaceReport {
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

pub(crate) fn inspect_development_surface(root: &Path) -> DevelopmentSurfaceReport {
    let surface = vec![
        PresenceCheck::new(
            ".cargo/config.toml",
            file_exists(root, ".cargo/config.toml"),
        ),
        PresenceCheck::new(
            ".devcontainer/devcontainer.json",
            file_exists(root, ".devcontainer/devcontainer.json"),
        ),
        PresenceCheck::new(
            ".devcontainer/post-create.sh",
            file_exists(root, ".devcontainer/post-create.sh"),
        ),
        PresenceCheck::new(
            "scripts/bootstrap-dev-tools.ps1",
            file_exists(root, "scripts/bootstrap-dev-tools.ps1"),
        ),
        PresenceCheck::new(
            "scripts/bootstrap-dev-tools.sh",
            file_exists(root, "scripts/bootstrap-dev-tools.sh"),
        ),
        PresenceCheck::new(
            "scripts/sync-mirrors.sh",
            file_exists(root, "scripts/sync-mirrors.sh"),
        ),
    ];

    DevelopmentSurfaceReport { surface }
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}
