//! Builds a repair plan for one RustUse facade repository.
//!
//! Planning is read-only. This module discovers child crates, combines the
//! selected repair groups, asks the manifest module to produce repaired
//! contents, and records only files whose contents would change.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::manifest::{self, FacadeManifestRepairs};
use super::model::{FacadeFixOptions, FacadeFixPlan, PlannedFileChange};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ManifestFixGroup {
    All,
    FacadeWiring,
    WorkspaceShape,
    WorkspaceDependencies,
    PackageMetadata,
}

impl ManifestFixGroup {
    fn from_code(code: &str) -> Result<Self> {
        match code {
            "all" => Ok(Self::All),

            "facade-wiring"
            | "missing-facade-dependencies"
            | "missing-facade-child-dependency"
            | "invalid-facade-child-dependency"
            | "missing-facade-child-dependency-optional"
            | "missing-facade-features"
            | "invalid-facade-default-features"
            | "missing-facade-default-features"
            | "missing-facade-full-feature"
            | "missing-full-feature-member"
            | "missing-facade-child-feature"
            | "invalid-facade-child-feature" => Ok(Self::FacadeWiring),

            "workspace-shape"
            | "missing-standard-workspace-member"
            | "non-standard-workspace-members"
            | "missing-workspace"
            | "missing-workspace-members"
            | "invalid-workspace-members"
            | "missing-workspace-resolver"
            | "workspace-resolver"
            | "missing-workspace-package"
            | "missing-workspace-package-field"
            | "invalid-workspace-repository"
            | "missing-workspace-categories"
            | "missing-workspace-unsafe-code-policy"
            | "invalid-workspace-unsafe-code-policy"
            | "missing-workspace-clippy-lints" => Ok(Self::WorkspaceShape),

            "workspace-dependencies"
            | "missing-workspace-dependencies"
            | "missing-workspace-dependency"
            | "invalid-workspace-dependency"
            | "invalid-workspace-dependency-path"
            | "missing-workspace-dependency-path"
            | "missing-workspace-dependency-version" => Ok(Self::WorkspaceDependencies),

            "package-metadata"
            | "missing-package-field"
            | "missing-package-publish"
            | "package-publish"
            | "missing-package-categories"
            | "missing-inherited-categories"
            | "missing-package-inherited-field"
            | "package-field-not-inherited"
            | "invalid-package-homepage"
            | "invalid-package-documentation"
            | "missing-package-readme-file"
            | "missing-docs-rs-all-features"
            | "invalid-docs-rs-all-features"
            | "missing-lints-workspace" => Ok(Self::PackageMetadata),

            other => bail!("unknown manifest fix code or group `{other}`"),
        }
    }

    const fn repairs(self) -> FacadeManifestRepairs {
        FacadeManifestRepairs {
            workspace_shape: matches!(self, Self::All | Self::WorkspaceShape),
            workspace_dependencies: matches!(self, Self::All | Self::WorkspaceDependencies),
            facade_wiring: matches!(self, Self::All | Self::FacadeWiring),
            package_metadata: matches!(self, Self::All | Self::PackageMetadata),
        }
    }

    const fn repairs_child_manifests(self) -> bool {
        matches!(self, Self::All | Self::PackageMetadata)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ManifestFixSelection {
    repairs: FacadeManifestRepairs,
    repair_child_manifests: bool,
}

impl ManifestFixSelection {
    fn from_codes(codes: &[String]) -> Result<Self> {
        if codes.is_empty() {
            return Ok(Self::from_group(ManifestFixGroup::All));
        }

        let mut selection = Self::default();

        for code in codes {
            selection.include(ManifestFixGroup::from_code(code)?);
        }

        Ok(selection)
    }

    const fn from_group(group: ManifestFixGroup) -> Self {
        Self {
            repairs: group.repairs(),
            repair_child_manifests: group.repairs_child_manifests(),
        }
    }

    fn include(&mut self, group: ManifestFixGroup) {
        let repairs = group.repairs();

        self.repairs.workspace_shape |= repairs.workspace_shape;
        self.repairs.workspace_dependencies |= repairs.workspace_dependencies;
        self.repairs.facade_wiring |= repairs.facade_wiring;
        self.repairs.package_metadata |= repairs.package_metadata;

        self.repair_child_manifests |= group.repairs_child_manifests();
    }
}

/// Builds a read-only repair plan for one facade repository.
///
/// The facade package is expected at the repository root. Directories below
/// `crates/` are treated only as child crates.
pub(crate) fn build_plan(root: &Path, options: &FacadeFixOptions) -> Result<FacadeFixPlan> {
    let root = canonical_facade_root(root)?;
    let facade_name = facade_name(&root)?;
    let selection = ManifestFixSelection::from_codes(&options.codes)?;

    let child_crates = manifest::discover_child_crates(&root.join("crates"), &facade_name)?;

    let mut plan = FacadeFixPlan {
        root: root.clone(),
        files_inspected: 0,
        files_unchanged: 0,
        changes: Vec::new(),
    };

    plan_facade_manifest(
        &mut plan,
        &root,
        &facade_name,
        &child_crates,
        selection.repairs,
    )?;

    if selection.repair_child_manifests {
        for child in &child_crates {
            plan_child_manifest(&mut plan, &root, &facade_name, child)?;
        }
    }

    plan.changes
        .sort_by(|left, right| left.path.cmp(&right.path));

    Ok(plan)
}

fn plan_facade_manifest(
    plan: &mut FacadeFixPlan,
    root: &Path,
    facade_name: &str,
    child_crates: &[manifest::CrateInfo],
    repairs: FacadeManifestRepairs,
) -> Result<()> {
    let manifest_path = root.join("Cargo.toml");
    let original = read_optional_file(&manifest_path)?;
    let created = original.is_none();
    let original = original.unwrap_or_default();

    plan.files_inspected += 1;

    let rendered = manifest::repair_facade_manifest(
        &manifest_path,
        &original,
        facade_name,
        child_crates,
        repairs,
    )?;

    record_file_result(plan, root, &manifest_path, created, &original, rendered)
}

fn plan_child_manifest(
    plan: &mut FacadeFixPlan,
    root: &Path,
    facade_name: &str,
    child: &manifest::CrateInfo,
) -> Result<()> {
    let manifest_path = &child.manifest_path;
    let original = read_required_file(manifest_path)?;

    plan.files_inspected += 1;

    let rendered = manifest::repair_child_manifest(manifest_path, &original, facade_name, child)?;

    record_file_result(plan, root, manifest_path, false, &original, rendered)
}

fn record_file_result(
    plan: &mut FacadeFixPlan,
    root: &Path,
    path: &Path,
    created: bool,
    original: &str,
    rendered: String,
) -> Result<()> {
    let rendered = ensure_trailing_newline(rendered);

    if rendered == original {
        plan.files_unchanged += 1;
        return Ok(());
    }

    plan.changes.push(PlannedFileChange {
        path: relative_path(root, path)?,
        contents: rendered,
        created,
    });

    Ok(())
}

fn canonical_facade_root(root: &Path) -> Result<PathBuf> {
    let canonical = fs::canonicalize(root)
        .with_context(|| format!("failed to resolve facade root `{}`", root.display()))?;

    if !canonical.is_dir() {
        bail!("facade root `{}` is not a directory", canonical.display());
    }

    Ok(canonical)
}

fn facade_name(root: &Path) -> Result<String> {
    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| {
            format!(
                "facade root `{}` does not have a valid UTF-8 directory name",
                root.display()
            )
        })?;

    if !name.starts_with("use-") {
        bail!("facade directory name `{name}` must start with `use-`");
    }

    Ok(name.to_owned())
}

fn read_optional_file(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("failed to read `{}`", path.display())),
    }
}

fn read_required_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read `{}`", path.display()))
}

fn ensure_trailing_newline(mut contents: String) -> String {
    while contents.ends_with("\n\n") {
        contents.pop();
    }

    if !contents.ends_with('\n') {
        contents.push('\n');
    }

    contents
}

fn relative_path(root: &Path, path: &Path) -> Result<PathBuf> {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .with_context(|| {
            format!(
                "planned path `{}` is outside facade root `{}`",
                path.display(),
                root.display()
            )
        })
}
