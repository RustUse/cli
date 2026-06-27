//! Shared filesystem surface scanning for RustUse development reports.

use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum SurfaceCheckStatus {
    Ok,
    Warning,
}

impl SurfaceCheckStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
        }
    }

    fn combine(self, other: Self) -> Self {
        self.max(other)
    }
}

#[derive(Debug)]
pub(crate) struct SurfaceProfile {
    pub(crate) required_files: &'static [(&'static str, &'static str)],
    pub(crate) optional_files: &'static [(&'static str, &'static str)],
    pub(crate) required_directories: &'static [(&'static str, &'static str)],
    pub(crate) optional_directories: &'static [(&'static str, &'static str)],
}

#[derive(Debug)]
pub(crate) struct RepositorySurfaceReport {
    pub(crate) files: Vec<FileSurfaceCheck>,
    pub(crate) directories: Vec<DirectorySurfaceCheck>,
}

impl RepositorySurfaceReport {
    pub(crate) fn status(&self) -> SurfaceCheckStatus {
        self.files
            .iter()
            .map(|check| check.status)
            .chain(self.directories.iter().map(|check| check.status))
            .fold(SurfaceCheckStatus::Ok, SurfaceCheckStatus::combine)
    }

    pub(crate) fn missing_required_files(&self) -> Vec<&FileSurfaceCheck> {
        self.files
            .iter()
            .filter(|check| check.required && !check.present)
            .collect()
    }

    pub(crate) fn missing_required_directories(&self) -> Vec<&DirectorySurfaceCheck> {
        self.directories
            .iter()
            .filter(|check| check.required && !check.present)
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct FileSurfaceCheck {
    pub(crate) path: &'static str,
    pub(crate) label: &'static str,
    pub(crate) required: bool,
    pub(crate) present: bool,
    pub(crate) status: SurfaceCheckStatus,
}

#[derive(Debug)]
pub(crate) struct DirectorySurfaceCheck {
    pub(crate) path: &'static str,
    pub(crate) label: &'static str,
    pub(crate) required: bool,
    pub(crate) present: bool,
    pub(crate) status: SurfaceCheckStatus,
}

pub(crate) fn inspect_repository_surface(
    root: &Path,
    profile: &SurfaceProfile,
) -> RepositorySurfaceReport {
    let files = profile
        .required_files
        .iter()
        .map(|(path, label)| inspect_file(root, path, label, true))
        .chain(
            profile
                .optional_files
                .iter()
                .map(|(path, label)| inspect_file(root, path, label, false)),
        )
        .collect();

    let directories = profile
        .required_directories
        .iter()
        .map(|(path, label)| inspect_directory(root, path, label, true))
        .chain(
            profile
                .optional_directories
                .iter()
                .map(|(path, label)| inspect_directory(root, path, label, false)),
        )
        .collect();

    RepositorySurfaceReport { files, directories }
}

fn inspect_file(
    root: &Path,
    path: &'static str,
    label: &'static str,
    required: bool,
) -> FileSurfaceCheck {
    let present = root.join(path).is_file();

    let status = if required && !present {
        SurfaceCheckStatus::Warning
    } else {
        SurfaceCheckStatus::Ok
    };

    FileSurfaceCheck {
        path,
        label,
        required,
        present,
        status,
    }
}

fn inspect_directory(
    root: &Path,
    path: &'static str,
    label: &'static str,
    required: bool,
) -> DirectorySurfaceCheck {
    let present = root.join(path).is_dir();

    let status = if required && !present {
        SurfaceCheckStatus::Warning
    } else {
        SurfaceCheckStatus::Ok
    };

    DirectorySurfaceCheck {
        path,
        label,
        required,
        present,
        status,
    }
}
