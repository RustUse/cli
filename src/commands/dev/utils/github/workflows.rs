//! Shared GitHub workflow policy checks for RustUse repositories.

use std::path::Path;

use crate::commands::dev::utils::report::PresenceCheck;

pub(crate) const REQUIRED_GITHUB_WORKFLOWS: &[&str] = &[
    "advisory-rust-quality.yml",
    "cargo-audit.yml",
    "cargo-deny.yml",
    "ci.yml",
    "codeql.yml",
    "facade-publish-readiness.yml",
    "mirror.yml",
    "publish-readiness.yml",
    "pull-request.yml",
    "release-plz-pr.yml",
    "release-plz-release.yml",
    "sbom.yml",
    "secrets.yml",
    "trivy.yml",
];

#[derive(Debug)]
pub(crate) struct GitHubWorkflowReport {
    pub(crate) required_surface: Vec<PresenceCheck>,
    pub(crate) required_workflows: Vec<PresenceCheck>,
}

impl GitHubWorkflowReport {
    pub(crate) fn status(&self) -> &'static str {
        if self.has_missing_required_paths() {
            "warning"
        } else {
            "ok"
        }
    }

    pub(crate) fn required_count(&self) -> usize {
        self.required_surface.len() + self.required_workflows.len()
    }

    pub(crate) fn present_required_count(&self) -> usize {
        self.required_surface
            .iter()
            .chain(self.required_workflows.iter())
            .filter(|check| check.present)
            .count()
    }

    pub(crate) fn workflow_count(&self) -> usize {
        self.required_workflows.len()
    }

    pub(crate) fn present_workflow_count(&self) -> usize {
        self.required_workflows
            .iter()
            .filter(|check| check.present)
            .count()
    }

    pub(crate) fn missing_required_count(&self) -> usize {
        self.required_surface
            .iter()
            .chain(self.required_workflows.iter())
            .filter(|check| !check.present)
            .count()
    }

    pub(crate) fn has_missing_required_paths(&self) -> bool {
        self.required_surface
            .iter()
            .chain(self.required_workflows.iter())
            .any(|check| !check.present)
    }

    pub(crate) fn missing_required_paths(&self) -> Vec<&str> {
        self.required_surface
            .iter()
            .chain(self.required_workflows.iter())
            .filter(|check| !check.present)
            .map(|check| check.path.as_str())
            .collect()
    }
}

pub(crate) fn inspect_github_workflows(root: &Path) -> GitHubWorkflowReport {
    let required_surface = vec![
        PresenceCheck::new(".github/", dir_exists(root, ".github")),
        PresenceCheck::new(".github/workflows/", dir_exists(root, ".github/workflows")),
        PresenceCheck::new(
            ".github/dependabot.yml",
            file_exists(root, ".github/dependabot.yml"),
        ),
    ];

    let required_workflows = REQUIRED_GITHUB_WORKFLOWS
        .iter()
        .map(|workflow| {
            let path = required_github_workflow_path(workflow);
            let present = file_exists(root, &path);

            PresenceCheck::new(path, present)
        })
        .collect();

    GitHubWorkflowReport {
        required_surface,
        required_workflows,
    }
}

fn required_github_workflow_path(file_name: &str) -> String {
    format!(".github/workflows/{file_name}")
}

fn file_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_file()
}

fn dir_exists(root: &Path, path: &str) -> bool {
    root.join(path).is_dir()
}
