//! Shared GitLab CI surface checks for RustUse repositories.

use std::path::Path;

use crate::rustuse::report::destination::PresenceCheck;

#[derive(Debug)]
pub(crate) struct GitLabReport {
    pub(crate) surface: Vec<PresenceCheck>,
}

impl GitLabReport {
    pub(crate) fn total_count(&self) -> usize {
        self.surface.len()
    }

    pub(crate) fn present_count(&self) -> usize {
        self.surface.iter().filter(|check| check.present).count()
    }
}

pub(crate) fn inspect_gitlab(root: &Path) -> GitLabReport {
    let surface = vec![
        PresenceCheck::new(".gitlab/", dir_exists(root, ".gitlab")),
        PresenceCheck::new(".gitlab-ci.yml", file_exists(root, ".gitlab-ci.yml")),
    ];

    GitLabReport { surface }
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}

fn dir_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_dir()
}
