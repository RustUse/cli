use std::path::{Path, PathBuf};

use crate::rustuse::report::destination::report_path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum FacadeStatus {
    Ok,
    Warning,
    Error,
}

impl FacadeStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    pub(crate) const fn from_error_warning_counts(errors: usize, warnings: usize) -> Self {
        if errors > 0 {
            Self::Error
        } else if warnings > 0 {
            Self::Warning
        } else {
            Self::Ok
        }
    }

    pub(crate) const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Error, _) | (_, Self::Error) => Self::Error,
            (Self::Warning, _) | (_, Self::Warning) => Self::Warning,
            (Self::Ok, Self::Ok) => Self::Ok,
        }
    }
}

impl std::fmt::Display for FacadeStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FacadeInfo {
    pub(crate) root: PathBuf,
    pub(crate) name: String,
    pub(crate) git_dir: PathBuf,
    pub(crate) manifest_path: PathBuf,
    pub(crate) crates_dir: PathBuf,
    pub(crate) crate_manifest_paths: Vec<PathBuf>,
}

impl FacadeInfo {
    pub(crate) fn new(
        root: impl Into<PathBuf>,
        name: impl Into<String>,
        crate_manifest_paths: Vec<PathBuf>,
    ) -> Self {
        let root = root.into();

        Self {
            git_dir: root.join(".git"),
            manifest_path: root.join("Cargo.toml"),
            crates_dir: root.join("crates"),
            root,
            name: name.into(),
            crate_manifest_paths,
        }
    }

    pub(crate) fn has_git(&self) -> bool {
        self.git_dir.is_dir()
    }

    pub(crate) fn has_manifest(&self) -> bool {
        self.manifest_path.is_file()
    }

    pub(crate) fn has_crates_dir(&self) -> bool {
        self.crates_dir.is_dir()
    }

    pub(crate) fn crate_count(&self) -> usize {
        self.crate_manifest_paths.len()
    }

    pub(crate) fn has_child_crates(&self) -> bool {
        self.crate_count() > 0
    }

    pub(crate) fn status(&self) -> &'static str {
        self.facade_status().as_str()
    }

    pub(crate) fn facade_status(&self) -> FacadeStatus {
        if !self.has_manifest() {
            FacadeStatus::Error
        } else if !self.has_git() || !self.has_crates_dir() || !self.has_child_crates() {
            FacadeStatus::Warning
        } else {
            FacadeStatus::Ok
        }
    }

    pub(crate) fn relative_path<'a>(&self, path: &'a Path) -> &'a Path {
        path.strip_prefix(&self.root).unwrap_or(path)
    }

    pub(crate) fn display_path(&self, path: &Path) -> String {
        report_path(self.relative_path(path))
    }

    pub(crate) fn root_manifest_display_path(&self) -> String {
        self.display_path(&self.manifest_path)
    }

    pub(crate) fn crates_dir_display_path(&self) -> String {
        self.display_path(&self.crates_dir)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FacadeCrateInfo {
    pub(crate) kind: FacadeCrateKind,
    pub(crate) name: String,
    pub(crate) manifest_path: PathBuf,
    pub(crate) readme_path: PathBuf,
    pub(crate) lib_path: PathBuf,
    pub(crate) prelude_path: PathBuf,
}

impl FacadeCrateInfo {
    pub(crate) fn from_manifest(facade: &FacadeInfo, manifest_path: &Path) -> Self {
        let root = manifest_path.parent().unwrap_or(&facade.root).to_path_buf();

        let name = root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>")
            .to_owned();

        let kind = if name == facade.name {
            FacadeCrateKind::Facade
        } else {
            FacadeCrateKind::Child
        };

        Self {
            kind,
            name,
            readme_path: root.join("README.md"),
            lib_path: root.join("src/lib.rs"),
            prelude_path: root.join("src/prelude.rs"),
            manifest_path: manifest_path.to_path_buf(),
        }
    }

    pub(crate) fn readme_present(&self) -> bool {
        self.readme_path.is_file()
    }

    pub(crate) fn lib_present(&self) -> bool {
        self.lib_path.is_file()
    }

    pub(crate) fn prelude_present(&self) -> bool {
        self.prelude_path.is_file()
    }

    pub(crate) const fn requires_prelude(&self) -> bool {
        matches!(self.kind, FacadeCrateKind::Facade)
    }

    pub(crate) fn documentation_status(&self) -> FacadeStatus {
        if self.readme_present()
            && self.lib_present()
            && (!self.requires_prelude() || self.prelude_present())
        {
            FacadeStatus::Ok
        } else {
            FacadeStatus::Warning
        }
    }

    pub(crate) fn documentation_notes(&self) -> String {
        let mut notes = Vec::new();

        if !self.readme_present() {
            notes.push("missing README.md");
        }

        if !self.lib_present() {
            notes.push("missing src/lib.rs");
        }

        if self.requires_prelude() && !self.prelude_present() {
            notes.push("missing facade src/prelude.rs");
        }

        if notes.is_empty() {
            "ok".to_owned()
        } else {
            notes.join("; ")
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum FacadeCrateKind {
    Facade,
    Child,
}

impl FacadeCrateKind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Facade => "facade",
            Self::Child => "child",
        }
    }
}

impl std::fmt::Display for FacadeCrateKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
