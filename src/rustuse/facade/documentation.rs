//! Shared documentation surface checks for RustUse repositories.

use std::path::Path;

use crate::rustuse::report::destination::PresenceCheck;

#[derive(Debug)]
pub(crate) struct DocumentationSurfaceReport {
    pub(crate) surface: Vec<PresenceCheck>,
}

impl DocumentationSurfaceReport {
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

pub(crate) fn inspect_documentation_surface(root: &Path) -> DocumentationSurfaceReport {
    let surface = vec![
        PresenceCheck::new("README.md", file_exists(root, "README.md")),
        PresenceCheck::new("CHANGELOG.md", file_exists(root, "CHANGELOG.md")),
        PresenceCheck::new("CONTRIBUTING.md", file_exists(root, "CONTRIBUTING.md")),
        PresenceCheck::new("GOVERNANCE.md", file_exists(root, "GOVERNANCE.md")),
        PresenceCheck::new("MAINTAINERS.md", file_exists(root, "MAINTAINERS.md")),
    ];

    DocumentationSurfaceReport { surface }
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}
