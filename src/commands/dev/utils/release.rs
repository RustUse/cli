//! Shared release surface checks for RustUse repositories.

use std::path::Path;

use crate::commands::dev::utils::report::PresenceCheck;

#[derive(Debug)]
pub(crate) struct ReleaseSurfaceReport {
    pub(crate) surface: Vec<PresenceCheck>,
    pub(crate) ci_surface: Vec<PresenceCheck>,
}

impl ReleaseSurfaceReport {
    pub(crate) fn total_count(&self) -> usize {
        self.surface.len()
    }

    pub(crate) fn present_count(&self) -> usize {
        self.surface.iter().filter(|check| check.present).count()
    }

    pub(crate) fn ci_total_count(&self) -> usize {
        self.ci_surface.len()
    }

    pub(crate) fn ci_present_count(&self) -> usize {
        self.ci_surface.iter().filter(|check| check.present).count()
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.surface.iter().all(|check| check.present) {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn inspect_release_surface(root: &Path) -> ReleaseSurfaceReport {
    let surface = vec![
        PresenceCheck::new("release-plz.toml", file_exists(root, "release-plz.toml")),
        PresenceCheck::new("RELEASE.md", file_exists(root, "RELEASE.md")),
        PresenceCheck::new("RELEASING.md", file_exists(root, "RELEASING.md")),
        PresenceCheck::new("CHANGELOG.md", file_exists(root, "CHANGELOG.md")),
        PresenceCheck::new("Cargo.lock", file_exists(root, "Cargo.lock")),
    ];

    let ci_surface = vec![PresenceCheck::new(
        "release-plz.toml",
        file_exists(root, "release-plz.toml"),
    )];

    ReleaseSurfaceReport {
        surface,
        ci_surface,
    }
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}
