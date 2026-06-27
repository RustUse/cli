//! Shared Cargo manifest analysis for RustUse development commands.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::commands::dev::root::discover::FacadeEntry;
use crate::commands::dev::utils::crates_io::category_slugs::{
    MAX_CRATES_IO_CATEGORIES, is_valid_category_slug,
};

const EXPECTED_WORKSPACE_MEMBERS: &[&str] = &["crates/*"];

const RUSTUSE_GITHUB_ORG: &str = "https://github.com/RustUse";

const REQUIRED_WORKSPACE_PACKAGE_FIELDS: &[&str] = &[
    "authors",
    "edition",
    "license",
    "repository",
    "rust-version",
];

const REQUIRED_PACKAGE_FIELDS: &[&str] = &[
    "name",
    "version",
    "publish",
    "keywords",
    "description",
    "homepage",
    "documentation",
    "readme",
];

const EXPECTED_WORKSPACE_INHERITED_PACKAGE_FIELDS: &[&str] = &[
    "authors",
    "edition",
    "rust-version",
    "license",
    "repository",
];

/* fn manifest_shape_bucket(code: &str) -> &'static str {
    match code {
        "missing-standard-workspace-member"
        | "non-standard-workspace-members"
        | "missing-workspace"
        | "missing-workspace-members"
        | "invalid-workspace-members"
        | "missing-workspace-resolver"
        | "workspace-resolver"
        | "missing-workspace-package"
        | "missing-workspace-package-field"
        | "invalid-workspace-repository"
        | "missing-workspace-dependencies"
        | "missing-workspace-dependency"
        | "invalid-workspace-dependency"
        | "invalid-workspace-dependency-path"
        | "missing-workspace-dependency-path"
        | "missing-workspace-dependency-version"
        | "missing-workspace-unsafe-code-policy"
        | "invalid-workspace-unsafe-code-policy"
        | "missing-workspace-clippy-lints" => "Workspace shape",

        "missing-facade-dependencies"
        | "missing-facade-child-dependency"
        | "invalid-facade-child-dependency"
        | "missing-facade-child-dependency-optional"
        | "missing-facade-features"
        | "invalid-facade-default-features"
        | "missing-facade-default-features"
        | "missing-facade-full-feature"
        | "missing-full-feature-member"
        | "missing-facade-child-feature"
        | "invalid-facade-child-feature" => "Facade wiring",

        "invalid-package-homepage"
        | "invalid-package-documentation"
        | "missing-package-readme-file"
        | "missing-docs-rs-all-features"
        | "invalid-docs-rs-all-features"
        | "missing-lints-workspace"
        | "package-name-directory-mismatch"
        | "invalid-facade-package-name"
        | "invalid-child-package-name" => "Package shape",

        "invalid-category-slug"
        | "too-many-categories"
        | "duplicate-category"
        | "invalid-categories-shape"
        | "invalid-category-value"
        | "missing-workspace-categories"
        | "missing-package-categories"
        | "missing-inherited-categories" => "Category metadata",

        _ => "General metadata",
    }
} */

#[derive(Debug)]
pub(crate) struct FacadeManifestReport {
    pub(crate) facade_name: String,
    pub(crate) manifests: Vec<ManifestFileReport>,
}

impl FacadeManifestReport {
    pub(crate) fn status(&self) -> &'static str {
        if self.error_count() > 0 {
            "error"
        } else if self.warning_count() > 0 {
            "warning"
        } else {
            "ok"
        }
    }

    pub(crate) fn manifest_count(&self) -> usize {
        self.manifests.len()
    }

    pub(crate) fn issue_count(&self) -> usize {
        self.manifests
            .iter()
            .map(ManifestFileReport::issue_count)
            .sum()
    }

    pub(crate) fn error_count(&self) -> usize {
        self.manifests
            .iter()
            .map(ManifestFileReport::error_count)
            .sum()
    }

    pub(crate) fn warning_count(&self) -> usize {
        self.manifests
            .iter()
            .map(ManifestFileReport::warning_count)
            .sum()
    }

    pub(crate) fn invalid_category_count(&self) -> usize {
        self.manifests
            .iter()
            .map(ManifestFileReport::invalid_category_count)
            .sum()
    }
}

#[derive(Debug)]
pub(crate) struct ManifestFileReport {
    pub(crate) path: PathBuf,
    pub(crate) kind: ManifestKind,
    pub(crate) package_name: Option<String>,
    pub(crate) issues: Vec<ManifestIssue>,
}

impl ManifestFileReport {
    pub(crate) fn status(&self) -> &'static str {
        if self.error_count() > 0 {
            "error"
        } else if self.warning_count() > 0 {
            "warning"
        } else {
            "ok"
        }
    }

    pub(crate) fn issue_count(&self) -> usize {
        self.issues.len()
    }

    pub(crate) fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ManifestIssueSeverity::Error)
            .count()
    }

    pub(crate) fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ManifestIssueSeverity::Warning)
            .count()
    }

    pub(crate) fn invalid_category_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.code == "invalid-category-slug")
            .count()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ManifestKind {
    WorkspaceRoot,
    FacadePackage,
    ChildPackage,
}

impl ManifestKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace-root",
            Self::FacadePackage => "facade-package",
            Self::ChildPackage => "child-package",
        }
    }
}

#[derive(Debug)]
pub(crate) struct ManifestIssue {
    pub(crate) severity: ManifestIssueSeverity,
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

impl ManifestIssue {
    pub(crate) fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: ManifestIssueSeverity::Error,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: ManifestIssueSeverity::Warning,
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ManifestIssueSeverity {
    Error,
    Warning,
}

impl ManifestIssueSeverity {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

pub(crate) fn analyze_manifests(
    root: &Path,
    facades: &[FacadeEntry],
) -> Result<Vec<FacadeManifestReport>> {
    facades
        .iter()
        .map(|facade| analyze_facade_manifests(root, facade))
        .collect()
}

pub(crate) fn analyze_facade_manifests(
    root: &Path,
    facade: &FacadeEntry,
) -> Result<FacadeManifestReport> {
    let facade_root = root.join(&facade.name);
    let workspace_manifest_path = facade_root.join("Cargo.toml");
    let crates_dir = facade_root.join("crates");

    let mut manifests = Vec::new();

    let crate_manifest_paths = discover_crate_manifest_paths(&crates_dir)?;

    let crate_dir_names = crate_manifest_paths
        .iter()
        .filter_map(|path| crate_dir_name(path))
        .collect::<BTreeSet<_>>();

    /* let child_crate_names = crate_dir_names
        .iter()
        .filter(|name| name.as_str() != facade.name.as_str())
        .cloned()
        .collect::<BTreeSet<_>>();

    let workspace_categories = analyze_workspace_root_manifest(
        root,
        &workspace_manifest_path,
        facade,
        &child_crate_names,
        &mut manifests,
    ); */

    let child_crate_names = crate_dir_names
        .iter()
        .filter(|name| name.as_str() != facade.name.as_str())
        .cloned()
        .collect::<BTreeSet<_>>();

    let child_crates = collect_child_crate_info(&crate_manifest_paths, &facade.name);

    let workspace_categories = analyze_workspace_root_manifest(
        root,
        &workspace_manifest_path,
        facade,
        &child_crates,
        &mut manifests,
    );

    for manifest_path in crate_manifest_paths {
        let crate_dir_name = manifest_path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");

        let kind = if crate_dir_name == facade.name {
            ManifestKind::FacadePackage
        } else {
            ManifestKind::ChildPackage
        };

        analyze_package_manifest(
            root,
            &manifest_path,
            kind,
            &facade.name,
            &child_crate_names,
            workspace_categories.as_deref(),
            &mut manifests,
        );
    }

    Ok(FacadeManifestReport {
        facade_name: facade.name.clone(),
        manifests,
    })
}

pub(crate) fn analyze_facade_repository_manifests(
    facade_root: &Path,
    facade_name: &str,
) -> Result<FacadeManifestReport> {
    let workspace_manifest_path = facade_root.join("Cargo.toml");
    let crates_dir = facade_root.join("crates");

    let mut manifests = Vec::new();

    let crate_manifest_paths = discover_crate_manifest_paths(&crates_dir)?;

    let crate_dir_names = crate_manifest_paths
        .iter()
        .filter_map(|path| crate_dir_name(path))
        .collect::<BTreeSet<_>>();

    let child_crate_names = crate_dir_names
        .iter()
        .filter(|name| name.as_str() != facade_name)
        .cloned()
        .collect::<BTreeSet<_>>();

    let child_crates = collect_child_crate_info(&crate_manifest_paths, facade_name);

    let facade = FacadeEntry {
        name: facade_name.to_string(),
        version: None,
        has_git: facade_root.join(".git").is_dir(),
        has_cargo_toml: workspace_manifest_path.is_file(),
        has_crates_dir: crates_dir.is_dir(),
        child_crate_count: crate_manifest_paths.len(),
    };

    let workspace_categories = analyze_workspace_root_manifest(
        facade_root,
        &workspace_manifest_path,
        &facade,
        &child_crates,
        &mut manifests,
    );

    for manifest_path in crate_manifest_paths {
        let crate_dir_name = manifest_path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");

        let kind = if crate_dir_name == facade_name {
            ManifestKind::FacadePackage
        } else {
            ManifestKind::ChildPackage
        };

        analyze_package_manifest(
            facade_root,
            &manifest_path,
            kind,
            facade_name,
            &child_crate_names,
            workspace_categories.as_deref(),
            &mut manifests,
        );
    }

    Ok(FacadeManifestReport {
        facade_name: facade_name.to_string(),
        manifests,
    })
}

/* fn analyze_workspace_root_manifest(
    root: &Path,
    manifest_path: &Path,
    facade: &FacadeEntry,
    child_crate_names: &BTreeSet<String>,
    manifests: &mut Vec<ManifestFileReport>,
) -> Option<Vec<String>> { */
fn analyze_workspace_root_manifest(
    root: &Path,
    manifest_path: &Path,
    facade: &FacadeEntry,
    child_crates: &BTreeMap<String, ChildCrateInfo>,
    manifests: &mut Vec<ManifestFileReport>,
) -> Option<Vec<String>> {
    let mut report = ManifestFileReport {
        path: relative_path(root, manifest_path),
        kind: ManifestKind::WorkspaceRoot,
        package_name: None,
        issues: Vec::new(),
    };

    if !manifest_path.is_file() {
        report.issues.push(ManifestIssue::error(
            "missing-workspace-manifest",
            "missing workspace root Cargo.toml",
        ));
        manifests.push(report);
        return None;
    }

    let Some(manifest) = read_manifest(manifest_path, &mut report) else {
        manifests.push(report);
        return None;
    };

    let workspace_categories =
        analyze_workspace_table(&manifest, facade, &mut report, child_crates);

    manifests.push(report);

    workspace_categories
}

fn analyze_package_manifest(
    root: &Path,
    manifest_path: &Path,
    kind: ManifestKind,
    facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    workspace_categories: Option<&[String]>,
    manifests: &mut Vec<ManifestFileReport>,
) {
    let mut report = ManifestFileReport {
        path: relative_path(root, manifest_path),
        kind,
        package_name: None,
        issues: Vec::new(),
    };

    let Some(manifest) = read_manifest(manifest_path, &mut report) else {
        manifests.push(report);
        return;
    };

    analyze_package_table(
        &manifest,
        kind,
        facade_name,
        child_crate_names,
        workspace_categories,
        &mut report,
    );

    manifests.push(report);
}

fn analyze_workspace_table(
    manifest: &toml::Value,
    facade: &FacadeEntry,
    report: &mut ManifestFileReport,
    child_crates: &BTreeMap<String, ChildCrateInfo>,
) -> Option<Vec<String>> {
    let Some(workspace) = manifest.get("workspace").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::error(
            "missing-workspace",
            "workspace root manifest is missing [workspace]",
        ));
        return None;
    };

    match workspace.get("resolver").and_then(toml::Value::as_str) {
        Some("3") => {},
        Some(resolver) => report.issues.push(ManifestIssue::warning(
            "workspace-resolver",
            format!("expected [workspace].resolver = \"3\", found \"{resolver}\""),
        )),
        None => report.issues.push(ManifestIssue::warning(
            "missing-workspace-resolver",
            "missing [workspace].resolver",
        )),
    }

    match workspace.get("members") {
        Some(value) if value.as_array().is_some() => {},
        Some(_) => report.issues.push(ManifestIssue::warning(
            "invalid-workspace-members",
            "expected [workspace].members to be an array",
        )),
        None => report.issues.push(ManifestIssue::warning(
            "missing-workspace-members",
            "missing [workspace].members",
        )),
    }

    validate_workspace_members(workspace, report);
    // validate_workspace_dependencies(workspace, child_crate_names, report);
    validate_workspace_dependencies(workspace, child_crates, report);
    validate_workspace_lints(workspace, report);

    let Some(workspace_package) = workspace.get("package").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::warning(
            "missing-workspace-package",
            "missing [workspace.package]",
        ));
        return None;
    };

    validate_workspace_repository(workspace_package, &facade.name, report);

    for field in REQUIRED_WORKSPACE_PACKAGE_FIELDS {
        if !workspace_package.contains_key(*field) {
            report.issues.push(ManifestIssue::warning(
                "missing-workspace-package-field",
                format!("missing [workspace.package].{field}"),
            ));
        }
    }

    let Some(categories_value) = workspace_package.get("categories") else {
        report.issues.push(ManifestIssue::warning(
            "missing-workspace-categories",
            "missing [workspace.package].categories",
        ));
        return None;
    };

    let categories =
        collect_category_strings(categories_value, "[workspace.package].categories", report);

    if let Some(categories) = categories.as_ref() {
        validate_categories(categories, "[workspace.package].categories", report);
    }

    categories
}

fn analyze_package_table(
    manifest: &toml::Value,
    kind: ManifestKind,
    facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    workspace_categories: Option<&[String]>,
    report: &mut ManifestFileReport,
) {
    let Some(package) = manifest.get("package").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::error(
            "missing-package",
            "crate manifest is missing [package]",
        ));
        return;
    };

    report.package_name = package
        .get("name")
        .and_then(toml::Value::as_str)
        .map(ToOwned::to_owned);

    for field in REQUIRED_PACKAGE_FIELDS {
        if !package.contains_key(*field) {
            report.issues.push(ManifestIssue::warning(
                "missing-package-field",
                format!("missing [package].{field}"),
            ));
        }
    }

    if package.get("name").and_then(toml::Value::as_str).is_none() {
        report.issues.push(ManifestIssue::error(
            "invalid-package-name",
            "expected [package].name to be a string",
        ));
    }

    if !has_string_or_workspace_true(package, "version") {
        report.issues.push(ManifestIssue::warning(
            "invalid-package-version",
            "expected [package].version to be a string or use version.workspace = true",
        ));
    }

    match package.get("publish").and_then(toml::Value::as_bool) {
        Some(true) => {},
        Some(false) => report.issues.push(ManifestIssue::warning(
            "package-publish",
            "expected [package].publish = true for publishable RustUse crates",
        )),
        None => report.issues.push(ManifestIssue::warning(
            "missing-package-publish",
            "missing [package].publish",
        )),
    }

    for field in EXPECTED_WORKSPACE_INHERITED_PACKAGE_FIELDS {
        check_workspace_inherited_package_field(package, field, report);
    }

    analyze_package_categories(package, workspace_categories, report);

    if kind == ManifestKind::FacadePackage {
        validate_facade_dependency_and_feature_wiring(
            manifest,
            facade_name,
            child_crate_names,
            report,
        );
    }
}

fn analyze_package_categories(
    package: &toml::Table,
    workspace_categories: Option<&[String]>,
    report: &mut ManifestFileReport,
) {
    let Some(categories_value) = package.get("categories") else {
        report.issues.push(ManifestIssue::warning(
            "missing-package-categories",
            "missing [package].categories or categories.workspace = true",
        ));
        return;
    };

    if is_workspace_true(categories_value) {
        if workspace_categories.is_none() {
            report.issues.push(ManifestIssue::error(
                "missing-inherited-categories",
                "package uses categories.workspace = true, but [workspace.package].categories is missing",
            ));
        }

        return;
    }

    let Some(categories) =
        collect_category_strings(categories_value, "[package].categories", report)
    else {
        return;
    };

    validate_categories(&categories, "[package].categories", report);
}

fn check_workspace_inherited_package_field(
    package: &toml::Table,
    field: &'static str,
    report: &mut ManifestFileReport,
) {
    let Some(value) = package.get(field) else {
        report.issues.push(ManifestIssue::warning(
            "missing-package-inherited-field",
            format!("missing [package].{field}.workspace = true"),
        ));
        return;
    };

    if is_workspace_true(value) {
        return;
    }

    report.issues.push(ManifestIssue::warning(
        "package-field-not-inherited",
        format!("expected [package].{field}.workspace = true"),
    ));
}

fn collect_category_strings(
    value: &toml::Value,
    field_name: &'static str,
    report: &mut ManifestFileReport,
) -> Option<Vec<String>> {
    let Some(array) = value.as_array() else {
        report.issues.push(ManifestIssue::error(
            "invalid-categories-shape",
            format!("expected {field_name} to be an array of strings"),
        ));
        return None;
    };

    let mut categories = Vec::new();

    for (index, item) in array.iter().enumerate() {
        let Some(category) = item.as_str() else {
            report.issues.push(ManifestIssue::error(
                "invalid-category-value",
                format!("expected {field_name}[{index}] to be a string"),
            ));
            continue;
        };

        categories.push(category.to_string());
    }

    Some(categories)
}

fn validate_categories(
    categories: &[String],
    field_name: &'static str,
    report: &mut ManifestFileReport,
) {
    if categories.len() > MAX_CRATES_IO_CATEGORIES {
        report.issues.push(ManifestIssue::error(
            "too-many-categories",
            format!(
                "{field_name} has {} categories; crates.io allows at most {}",
                categories.len(),
                MAX_CRATES_IO_CATEGORIES
            ),
        ));
    }

    let mut seen = BTreeSet::new();

    for category in categories {
        if !seen.insert(category.as_str()) {
            report.issues.push(ManifestIssue::warning(
                "duplicate-category",
                format!("{field_name} contains duplicate category `{category}`"),
            ));
        }

        if !is_valid_category_slug(category) {
            report.issues.push(ManifestIssue::error(
                "invalid-category-slug",
                format!("`{category}` is not a valid crates.io category slug"),
            ));
        }
    }
}

fn has_string_or_workspace_true(package: &toml::Table, field: &'static str) -> bool {
    let Some(value) = package.get(field) else {
        return false;
    };

    value.as_str().is_some() || is_workspace_true(value)
}

fn is_workspace_true(value: &toml::Value) -> bool {
    value
        .as_table()
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
}

fn read_manifest(path: &Path, report: &mut ManifestFileReport) -> Option<toml::Value> {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            report.issues.push(ManifestIssue::error(
                "read-manifest",
                format!("failed to read `{}`: {error}", path.display()),
            ));
            return None;
        },
    };

    match toml::from_str(&raw) {
        Ok(value) => Some(value),
        Err(error) => {
            report.issues.push(ManifestIssue::error(
                "parse-manifest",
                format!("failed to parse `{}`: {error}", path.display()),
            ));
            None
        },
    }
}

fn discover_crate_manifest_paths(crates_dir: &Path) -> Result<Vec<PathBuf>> {
    if !crates_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();

    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let manifest = path.join("Cargo.toml");

        if manifest.is_file() {
            manifests.push(manifest);
        }
    }

    manifests.sort();

    Ok(manifests)
}

fn relative_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}

fn crate_dir_name(manifest_path: &Path) -> Option<String> {
    manifest_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
}

#[derive(Debug, Clone)]
pub struct ChildCrateInfo {
    // dir_name: String,
    // package_name: Option<String>,
    version: Option<String>,
}

fn collect_child_crate_info(
    crate_manifest_paths: &[PathBuf],
    facade_name: &str,
) -> BTreeMap<String, ChildCrateInfo> {
    let mut child_crates = BTreeMap::new();

    for manifest_path in crate_manifest_paths {
        let Some(dir_name) = crate_dir_name(manifest_path) else {
            continue;
        };

        if dir_name == facade_name {
            continue;
        }

        let raw = fs::read_to_string(manifest_path).ok();
        let manifest = raw
            .as_deref()
            .and_then(|raw| toml::from_str::<toml::Value>(raw).ok());

        let package = manifest
            .as_ref()
            .and_then(|manifest| manifest.get("package"))
            .and_then(toml::Value::as_table);

        /* let package_name = package
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(ToOwned::to_owned); */

        let version = package
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
            .map(ToOwned::to_owned);

        child_crates.insert(
            dir_name.clone(),
            ChildCrateInfo {
                // dir_name,
                // package_name,
                version,
            },
        );
    }

    child_crates
}

fn validate_workspace_members(workspace: &toml::Table, report: &mut ManifestFileReport) {
    let Some(members) = workspace.get("members").and_then(toml::Value::as_array) else {
        return;
    };

    let members = members
        .iter()
        .filter_map(toml::Value::as_str)
        .collect::<BTreeSet<_>>();

    for expected in EXPECTED_WORKSPACE_MEMBERS {
        if !members.contains(expected) {
            report.issues.push(ManifestIssue::warning(
                "missing-standard-workspace-member",
                format!("expected [workspace].members to include `{expected}`"),
            ));
        }
    }

    if members.len() != EXPECTED_WORKSPACE_MEMBERS.len() {
        report.issues.push(ManifestIssue::warning(
            "non-standard-workspace-members",
            "expected [workspace].members to be exactly [\"crates/*\"]",
        ));
    }
}

fn validate_workspace_repository(
    workspace_package: &toml::Table,
    facade_name: &str,
    report: &mut ManifestFileReport,
) {
    let expected = format!("{RUSTUSE_GITHUB_ORG}/{facade_name}");

    match workspace_package
        .get("repository")
        .and_then(toml::Value::as_str)
    {
        Some(actual) if actual == expected => {},
        Some(actual) => report.issues.push(ManifestIssue::warning(
            "invalid-workspace-repository",
            format!("expected [workspace.package].repository = `{expected}`, found `{actual}`"),
        )),
        None => {},
    }
}

/* fn validate_workspace_dependencies(
    workspace: &toml::Table,
    child_crate_names: &BTreeSet<String>,
    report: &mut ManifestFileReport,
) {
    let Some(dependencies) = workspace
        .get("dependencies")
        .and_then(toml::Value::as_table)
    else {
        report.issues.push(ManifestIssue::warning(
            "missing-workspace-dependencies",
            "missing [workspace.dependencies]",
        ));
        return;
    };

    for crate_name in child_crate_names {
        let Some(dependency) = dependencies.get(crate_name) else {
            report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency",
                format!("missing [workspace.dependencies].{crate_name}"),
            ));
            continue;
        };

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency",
                format!("expected [workspace.dependencies].{crate_name} to be an inline table"),
            ));
            continue;
        };

        let expected_path = format!("crates/{crate_name}");

        match dependency_table.get("path").and_then(toml::Value::as_str) {
            Some(actual_path) if actual_path == expected_path => {}
            Some(actual_path) => report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-path",
                format!(
                    "expected [workspace.dependencies].{crate_name}.path = `{expected_path}`, found `{actual_path}`"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency-path",
                format!("missing [workspace.dependencies].{crate_name}.path"),
            )),
        }

        if dependency_table
            .get("version")
            .and_then(toml::Value::as_str)
            .is_none()
        {
            report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency-version",
                format!("missing [workspace.dependencies].{crate_name}.version"),
            ));
        }
    }
} */

fn validate_workspace_dependencies(
    workspace: &toml::Table,
    child_crates: &BTreeMap<String, ChildCrateInfo>,
    report: &mut ManifestFileReport,
) {
    let Some(dependencies) = workspace
        .get("dependencies")
        .and_then(toml::Value::as_table)
    else {
        report.issues.push(ManifestIssue::warning(
            "missing-workspace-dependencies",
            "missing [workspace.dependencies]",
        ));
        return;
    };

    validate_all_workspace_dependency_versions(dependencies, report);
    validate_child_workspace_dependencies(dependencies, child_crates, report);
    validate_orphan_workspace_dependency_paths(dependencies, child_crates, report);
}

/* fn validate_all_workspace_dependency_versions(
    dependencies: &toml::Table,
    report: &mut ManifestFileReport,
) {
    for (dependency_name, dependency) in dependencies {
        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-shape",
                format!(
                    "expected [workspace.dependencies].{dependency_name} to be an inline table with version"
                ),
            ));
            continue;
        };

        match dependency_table.get("version") {
            Some(version) if version.as_str().is_some_and(|value| !value.trim().is_empty()) => {},
            Some(_) => report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-version",
                format!(
                    "expected [workspace.dependencies].{dependency_name}.version to be a non-empty string"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency-version",
                format!("missing [workspace.dependencies].{dependency_name}.version"),
            )),
        }
    }
} */

fn validate_all_workspace_dependency_versions(
    dependencies: &toml::Table,
    report: &mut ManifestFileReport,
) {
    for (dependency_name, dependency) in dependencies {
        if let Some(version) = dependency.as_str() {
            if version.trim().is_empty() {
                report.issues.push(ManifestIssue::warning(
                    "invalid-workspace-dependency-version",
                    format!(
                        "expected [workspace.dependencies].{dependency_name} to use a non-empty version string"
                    ),
                ));
            }

            continue;
        }

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-shape",
                format!(
                    "expected [workspace.dependencies].{dependency_name} to be a version string or an inline table with version"
                ),
            ));
            continue;
        };

        match dependency_table.get("version") {
            Some(version) if version.as_str().is_some_and(|value| !value.trim().is_empty()) => {},
            Some(_) => report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-version",
                format!(
                    "expected [workspace.dependencies].{dependency_name}.version to be a non-empty string"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency-version",
                format!("missing [workspace.dependencies].{dependency_name}.version"),
            )),
        }
    }
}

fn validate_child_workspace_dependencies(
    dependencies: &toml::Table,
    child_crates: &BTreeMap<String, ChildCrateInfo>,
    report: &mut ManifestFileReport,
) {
    for (crate_name, child_crate) in child_crates {
        let Some(dependency) = dependencies.get(crate_name) else {
            report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency",
                format!("missing [workspace.dependencies].{crate_name}"),
            ));
            continue;
        };

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-shape",
                format!("expected [workspace.dependencies].{crate_name} to be an inline table"),
            ));
            continue;
        };

        let expected_path = format!("crates/{crate_name}");

        match dependency_table.get("path").and_then(toml::Value::as_str) {
            Some(actual_path) if actual_path == expected_path => {},
            Some(actual_path) => report.issues.push(ManifestIssue::warning(
                "invalid-workspace-dependency-path",
                format!(
                    "expected [workspace.dependencies].{crate_name}.path = `{expected_path}`, found `{actual_path}`"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                "missing-workspace-dependency-path",
                format!("missing [workspace.dependencies].{crate_name}.path"),
            )),
        }

        let actual_version = dependency_table
            .get("version")
            .and_then(toml::Value::as_str);

        if let (Some(expected_version), Some(actual_version)) =
            (child_crate.version.as_deref(), actual_version)
        {
            if actual_version != expected_version {
                report.issues.push(ManifestIssue::warning(
                    "mismatched-workspace-dependency-version",
                    format!(
                        "expected [workspace.dependencies].{crate_name}.version = `{expected_version}`, found `{actual_version}`"
                    ),
                ));
            }
        }
    }
}

fn validate_orphan_workspace_dependency_paths(
    dependencies: &toml::Table,
    child_crates: &BTreeMap<String, ChildCrateInfo>,
    report: &mut ManifestFileReport,
) {
    for (dependency_name, dependency) in dependencies {
        let Some(dependency_table) = dependency.as_table() else {
            continue;
        };

        let Some(path) = dependency_table.get("path").and_then(toml::Value::as_str) else {
            continue;
        };

        let Some(crate_name) = path.strip_prefix("crates/") else {
            continue;
        };

        if !child_crates.contains_key(crate_name) {
            report.issues.push(ManifestIssue::warning(
                "orphan-workspace-dependency-path",
                format!(
                    "[workspace.dependencies].{dependency_name}.path points to `{path}`, but no matching child crate manifest was found"
                ),
            ));
        }
    }
}

fn validate_workspace_lints(workspace: &toml::Table, report: &mut ManifestFileReport) {
    let unsafe_code = workspace
        .get("lints")
        .and_then(|lints| lints.get("rust"))
        .and_then(|rust| rust.get("unsafe_code"))
        .and_then(toml::Value::as_str);

    match unsafe_code {
        Some("forbid") => {},
        Some(actual) => report.issues.push(ManifestIssue::warning(
            "invalid-workspace-unsafe-code-policy",
            format!("expected [workspace.lints.rust].unsafe_code = \"forbid\", found `{actual}`"),
        )),
        None => report.issues.push(ManifestIssue::warning(
            "missing-workspace-unsafe-code-policy",
            "missing [workspace.lints.rust].unsafe_code = \"forbid\"",
        )),
    }

    let clippy = workspace
        .get("lints")
        .and_then(|lints| lints.get("clippy"))
        .and_then(toml::Value::as_table);

    if clippy.is_none() {
        report.issues.push(ManifestIssue::warning(
            "missing-workspace-clippy-lints",
            "missing [workspace.lints.clippy]",
        ));
    }
}

fn validate_facade_dependency_and_feature_wiring(
    manifest: &toml::Value,
    _facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    report: &mut ManifestFileReport,
) {
    validate_facade_child_dependencies(manifest, child_crate_names, report);
    validate_facade_features(manifest, child_crate_names, report);
}

fn validate_facade_child_dependencies(
    manifest: &toml::Value,
    child_crate_names: &BTreeSet<String>,
    report: &mut ManifestFileReport,
) {
    if child_crate_names.is_empty() {
        return;
    }

    let Some(dependencies) = manifest.get("dependencies").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::warning(
            "missing-facade-dependencies",
            "facade crate has child crates but no [dependencies]",
        ));
        return;
    };

    for child_crate in child_crate_names {
        let Some(dependency) = dependencies.get(child_crate) else {
            report.issues.push(ManifestIssue::warning(
                "missing-facade-child-dependency",
                format!("missing [dependencies].{child_crate}"),
            ));
            continue;
        };

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                "invalid-facade-child-dependency",
                format!("expected [dependencies].{child_crate} to be an inline table"),
            ));
            continue;
        };

        if !is_workspace_true(dependency) {
            report.issues.push(ManifestIssue::warning(
                "invalid-facade-child-dependency",
                format!("expected [dependencies].{child_crate}.workspace = true"),
            ));
        }

        match dependency_table
            .get("optional")
            .and_then(toml::Value::as_bool)
        {
            Some(true) => {},
            Some(false) => report.issues.push(ManifestIssue::warning(
                "invalid-facade-child-dependency",
                format!("expected [dependencies].{child_crate}.optional = true"),
            )),
            None => report.issues.push(ManifestIssue::warning(
                "missing-facade-child-dependency-optional",
                format!("missing [dependencies].{child_crate}.optional = true"),
            )),
        }
    }
}

fn validate_facade_features(
    manifest: &toml::Value,
    child_crate_names: &BTreeSet<String>,
    report: &mut ManifestFileReport,
) {
    if child_crate_names.is_empty() {
        return;
    }

    let Some(features) = manifest.get("features").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::warning(
            "missing-facade-features",
            "facade crate has child crates but no [features]",
        ));
        return;
    };

    match features.get("default").and_then(toml::Value::as_array) {
        Some(default_features) if default_features.is_empty() => {},
        Some(_) => report.issues.push(ManifestIssue::warning(
            "invalid-facade-default-features",
            "expected [features].default = []",
        )),
        None => report.issues.push(ManifestIssue::warning(
            "missing-facade-default-features",
            "missing [features].default = []",
        )),
    }

    let full_features = features
        .get("full")
        .and_then(toml::Value::as_array)
        .map(|values| array_string_set(values));

    if full_features.is_none() {
        report.issues.push(ManifestIssue::warning(
            "missing-facade-full-feature",
            "missing [features].full",
        ));
    }

    for child_crate in child_crate_names {
        let feature_name = feature_name_for_child_crate(child_crate);

        if let Some(full_features) = &full_features {
            if !full_features.contains(feature_name.as_str()) {
                report.issues.push(ManifestIssue::warning(
                    "missing-full-feature-member",
                    format!("expected [features].full to include `{feature_name}`"),
                ));
            }
        }

        let Some(feature_values) = features
            .get(feature_name.as_str())
            .and_then(toml::Value::as_array)
        else {
            report.issues.push(ManifestIssue::warning(
                "missing-facade-child-feature",
                format!("missing [features].{feature_name}"),
            ));
            continue;
        };

        let feature_values = array_string_set(feature_values);
        let expected_dep = format!("dep:{child_crate}");

        if !feature_values.contains(expected_dep.as_str()) {
            report.issues.push(ManifestIssue::warning(
                "invalid-facade-child-feature",
                format!("expected [features].{feature_name} to include `{expected_dep}`"),
            ));
        }
    }
}

fn feature_name_for_child_crate(crate_name: &str) -> String {
    crate_name
        .strip_prefix("use-")
        .unwrap_or(crate_name)
        .to_string()
}

fn array_string_set(array: &[toml::Value]) -> BTreeSet<String> {
    array
        .iter()
        .filter_map(toml::Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
}
