//! Shared generated/local artifact checks for RustUse repositories.

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct GeneratedArtifactReport {
    pub(crate) artifacts: Vec<GeneratedArtifactCheck>,
}

impl GeneratedArtifactReport {
    pub(crate) fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedArtifactCheck {
    pub(crate) path: PathBuf,
    pub(crate) label: &'static str,
}

pub(crate) fn inspect_generated_artifacts(root: &Path) -> GeneratedArtifactReport {
    let mut artifacts = Vec::new();
    let target = root.join("target");

    if !target.is_dir() {
        return GeneratedArtifactReport { artifacts };
    }

    artifacts.push(GeneratedArtifactCheck {
        path: PathBuf::from("target"),
        label: "Cargo build output",
    });

    if let Ok(entries) = target.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            if name.starts_with("flycheck") {
                artifacts.push(GeneratedArtifactCheck {
                    path: PathBuf::from("target").join(name),
                    label: "rust-analyzer flycheck output",
                });
            }
        }
    }

    GeneratedArtifactReport { artifacts }
}
