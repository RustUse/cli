//! Cargo manifest analysis for one RustUse facade repository.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::rustuse::adapter::crates_io::category_slugs::{
    MAX_CRATES_IO_CATEGORIES, is_valid_category_slug,
};

use crate::rustuse::adapter::cargo::manifest::{
    has_string_or_workspace_true, read_manifest_with_diagnostics,
};
use crate::rustuse::facade::codes::FacadeIssueCode;

const EXPECTED_WORKSPACE_MEMBERS: &[&str] = &["crates/*"];
const RUSTUSE_GITHUB_ORG: &str = "https://github.com/RustUse";

const REQUIRED_WORKSPACE_PACKAGE_FIELDS: &[&str] = &[
    "authors",
    "categories",
    "edition",
    "license",
    "repository",
    "rust-version",
];

const REQUIRED_PACKAGE_FIELDS: &[&str] = &[
    "name",
    "version",
    "edition",
    "license",
    "repository",
    "homepage",
    "documentation",
    "readme",
    "rust-version",
];

const EXPECTED_WORKSPACE_INHERITED_PACKAGE_FIELDS: &[&str] =
    &["edition", "license", "repository", "rust-version"];

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
            .filter(|issue| issue.code == FacadeIssueCode::InvalidCategorySlug)
            .count()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ManifestIssue {
    pub(crate) severity: ManifestIssueSeverity,
    pub(crate) code: FacadeIssueCode,
    pub(crate) message: String,
}

impl ManifestIssue {
    pub(crate) fn error(code: FacadeIssueCode, message: impl Into<String>) -> Self {
        Self {
            severity: ManifestIssueSeverity::Error,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn warning(code: FacadeIssueCode, message: impl Into<String>) -> Self {
        Self {
            severity: ManifestIssueSeverity::Warning,
            code,
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub(crate) fn analyze_facade_repository_manifests(
    facade_root: &Path,
    facade_name: &str,
) -> Result<FacadeManifestReport> {
    let workspace_manifest_path = facade_root.join("Cargo.toml");
    let crates_dir = facade_root.join("crates");

    let crate_manifest_paths = discover_crate_manifest_paths(&crates_dir)?;
    let crate_infos = collect_crate_infos(&crate_manifest_paths);

    let child_crate_names = crate_infos
        .values()
        .filter(|info| info.dependency_name() != facade_name)
        .map(|info| info.dependency_name().to_owned())
        .collect::<BTreeSet<_>>();

    let mut manifests = Vec::new();

    let workspace_categories = analyze_workspace_root_manifest(
        facade_root,
        &workspace_manifest_path,
        facade_name,
        &child_crate_names,
        &crate_infos,
        &crate_manifest_paths,
        &mut manifests,
    );

    for manifest_path in crate_manifest_paths {
        analyze_package_manifest(
            facade_root,
            &manifest_path,
            ManifestKind::ChildPackage,
            facade_name,
            &child_crate_names,
            workspace_categories.as_deref(),
            &mut manifests,
        );
    }

    Ok(FacadeManifestReport {
        facade_name: facade_name.to_owned(),
        manifests,
    })
}

fn analyze_workspace_root_manifest(
    facade_root: &Path,
    manifest_path: &Path,
    facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    crate_manifest_paths: &[PathBuf],
    manifests: &mut Vec<ManifestFileReport>,
) -> Option<Vec<String>> {
    let mut report = ManifestFileReport {
        path: relative_path(facade_root, manifest_path),
        kind: ManifestKind::WorkspaceRoot,
        package_name: None,
        issues: Vec::new(),
    };

    if !manifest_path.is_file() {
        report.issues.push(ManifestIssue::error(
            FacadeIssueCode::MissingWorkspaceManifest,
            "missing workspace root Cargo.toml",
        ));
        manifests.push(report);
        return None;
    }

    let Some(manifest) = read_manifest(manifest_path, &mut report) else {
        manifests.push(report);
        return None;
    };

    validate_nested_facade_package(
        &manifest,
        facade_root,
        crate_manifest_paths,
        crate_infos,
        &mut report,
    );

    let workspace_categories =
        analyze_workspace_table(&manifest, facade_name, crate_infos, &mut report);

    analyze_package_table(
        &manifest,
        manifest_path,
        ManifestKind::FacadePackage,
        facade_name,
        child_crate_names,
        workspace_categories.as_deref(),
        &mut report,
    );

    manifests.push(report);

    workspace_categories
}

fn validate_nested_facade_package(
    root_manifest: &toml::Value,
    facade_root: &Path,
    crate_manifest_paths: &[PathBuf],
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    report: &mut ManifestFileReport,
) {
    let Some(root_package_name) = root_manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
    else {
        return;
    };

    let mut matching_directories = crate_manifest_paths
        .iter()
        .filter_map(|manifest_path| {
            let dir_name = crate_dir_name(manifest_path)?;
            let package_name = crate_infos
                .values()
                .find(|info| info.dir_name == dir_name)
                .and_then(|info| info.package_name.as_deref());

            (package_name == Some(root_package_name)).then(|| {
                relative_path(facade_root, manifest_path)
                    .parent()
                    .unwrap_or_else(|| Path::new("crates"))
                    .display()
                    .to_string()
            })
        })
        .collect::<Vec<_>>();

    if matching_directories.is_empty() {
        return;
    }

    matching_directories.sort();
    matching_directories.dedup();

    let directories = matching_directories
        .iter()
        .map(|directory| format!("`{directory}`"))
        .collect::<Vec<_>>()
        .join(", ");

    report.issues.push(ManifestIssue::warning(
        FacadeIssueCode::NestedFacadePackage,
        format!(
            "Facade `{root_package_name}` must not also exist as a child crate under `crates/` (matching child directory: {directories}). Move the Facade package to the repository root and remove the nested package after preserving any required source, metadata, README, examples, and feature wiring."
        ),
    ));
}

fn analyze_package_manifest(
    facade_root: &Path,
    manifest_path: &Path,
    kind: ManifestKind,
    facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    workspace_categories: Option<&[String]>,
    manifests: &mut Vec<ManifestFileReport>,
) {
    let mut report = ManifestFileReport {
        path: relative_path(facade_root, manifest_path),
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
        manifest_path,
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
    facade_name: &str,
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    report: &mut ManifestFileReport,
) -> Option<Vec<String>> {
    let Some(workspace) = manifest.get("workspace").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::error(
            FacadeIssueCode::MissingWorkspace,
            "workspace root manifest is missing [workspace]",
        ));
        return None;
    };

    match workspace.get("resolver").and_then(toml::Value::as_str) {
        Some("3") => {},
        Some(resolver) => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidWorkspaceResolver,
            format!("expected [workspace].resolver = \"3\", found \"{resolver}\""),
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceResolver,
            "missing [workspace].resolver",
        )),
    }

    match workspace.get("members") {
        Some(value) if value.as_array().is_some() => {},
        Some(_) => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidWorkspaceMembers,
            "expected [workspace].members to be an array",
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceMembers,
            "missing [workspace].members",
        )),
    }

    validate_workspace_members(workspace, report);
    validate_workspace_dependencies(workspace, crate_infos, report);
    validate_workspace_lints(workspace, report);

    let Some(workspace_package) = workspace.get("package").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspacePackage,
            "missing [workspace.package]",
        ));
        return None;
    };

    validate_workspace_repository(workspace_package, facade_name, report);

    for field in REQUIRED_WORKSPACE_PACKAGE_FIELDS {
        if !workspace_package.contains_key(*field) {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingWorkspacePackageField,
                format!("missing [workspace.package].{field}"),
            ));
        }
    }

    let Some(categories_value) = workspace_package.get("categories") else {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceCategories,
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
    manifest_path: &Path,
    kind: ManifestKind,
    facade_name: &str,
    child_crate_names: &BTreeSet<String>,
    workspace_categories: Option<&[String]>,
    report: &mut ManifestFileReport,
) {
    let Some(package) = manifest.get("package").and_then(toml::Value::as_table) else {
        report.issues.push(ManifestIssue::error(
            FacadeIssueCode::MissingPackage,
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
                missing_package_field_code(field),
                format!("missing [package].{field}"),
            ));
        }
    }

    validate_package_name(package, kind, facade_name, report);
    validate_package_version(package, report);
    validate_package_publish(package, report);
    validate_package_homepage_and_documentation(package, report);
    validate_package_readme(package, report);
    validate_workspace_inherited_package_fields(package, report);
    validate_docs_rs_configuration(manifest, report);
    validate_package_directory_name(manifest_path, package, kind, facade_name, report);
    analyze_package_categories(package, workspace_categories, report);

    if kind == ManifestKind::FacadePackage {
        validate_facade_dependency_and_feature_wiring(manifest, child_crate_names, report);
    }
}

fn validate_docs_rs_configuration(manifest: &toml::Value, report: &mut ManifestFileReport) {
    let docs_rs = manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("docs"))
        .and_then(|docs| docs.get("rs"))
        .and_then(toml::Value::as_table);

    let Some(docs_rs) = docs_rs else {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingDocsRsAllFeatures,
            "missing [package.metadata.docs.rs].all-features = true",
        ));
        return;
    };

    match docs_rs.get("all-features") {
        Some(value) if value.as_bool() == Some(true) => {},
        Some(_) => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidDocsRsAllFeatures,
            "expected [package.metadata.docs.rs].all-features = true",
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingDocsRsAllFeatures,
            "missing [package.metadata.docs.rs].all-features = true",
        )),
    }
}

fn validate_package_directory_name(
    manifest_path: &Path,
    package: &toml::Table,
    kind: ManifestKind,
    facade_name: &str,
    report: &mut ManifestFileReport,
) {
    let Some(package_name) = package.get("name").and_then(toml::Value::as_str) else {
        return;
    };

    let expected_name = match kind {
        ManifestKind::WorkspaceRoot | ManifestKind::FacadePackage => facade_name,
        ManifestKind::ChildPackage => {
            let Some(directory_name) = manifest_path
                .parent()
                .and_then(Path::file_name)
                .and_then(|name| name.to_str())
            else {
                return;
            };

            directory_name
        },
    };

    if package_name != expected_name {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::PackageNameDirectoryMismatch,
            format!(
                "package name `{package_name}` does not match its expected directory name `{expected_name}`"
            ),
        ));
    }
}

fn validate_package_name(
    package: &toml::Table,
    kind: ManifestKind,
    facade_name: &str,
    report: &mut ManifestFileReport,
) {
    let Some(name) = package.get("name").and_then(toml::Value::as_str) else {
        report.issues.push(ManifestIssue::error(
            FacadeIssueCode::InvalidPackageName,
            "expected [package].name to be a string",
        ));
        return;
    };

    match kind {
        ManifestKind::WorkspaceRoot => {},
        ManifestKind::FacadePackage => {
            if name != facade_name {
                report.issues.push(ManifestIssue::warning(
                    FacadeIssueCode::InvalidFacadePackageName,
                    format!("expected facade package name `{facade_name}`, found `{name}`"),
                ));
            }
        },
        ManifestKind::ChildPackage => {
            if !name.starts_with("use-") {
                report.issues.push(ManifestIssue::warning(
                    FacadeIssueCode::InvalidChildPackageName,
                    format!("expected child package name to start with `use-`, found `{name}`"),
                ));
            }
        },
    }
}

fn validate_package_version(package: &toml::Table, report: &mut ManifestFileReport) {
    if !has_string_or_workspace_true(package, "version") {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidPackageVersion,
            "expected [package].version to be a string or use version.workspace = true",
        ));
    }
}

fn validate_package_publish(package: &toml::Table, report: &mut ManifestFileReport) {
    match package.get("publish").and_then(toml::Value::as_bool) {
        Some(true) => {},
        Some(false) => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidPackagePublish,
            "expected [package].publish = true for publishable RustUse crates",
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingPackagePublish,
            "missing [package].publish",
        )),
    }
}

fn validate_package_homepage_and_documentation(
    package: &toml::Table,
    report: &mut ManifestFileReport,
) {
    if !has_string_or_workspace_true(package, "homepage") {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidPackageHomepage,
            "expected [package].homepage to be a string or use homepage.workspace = true",
        ));
    }

    if !has_string_or_workspace_true(package, "documentation") {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidPackageDocumentation,
            "expected [package].documentation to be a string or use documentation.workspace = true",
        ));
    }
}

fn validate_package_readme(package: &toml::Table, report: &mut ManifestFileReport) {
    if !has_string_or_workspace_true(package, "readme") {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingPackageReadmeFile,
            "expected [package].readme to be set or inherited from workspace",
        ));
    }
}

fn validate_workspace_inherited_package_fields(
    package: &toml::Table,
    report: &mut ManifestFileReport,
) {
    for field in EXPECTED_WORKSPACE_INHERITED_PACKAGE_FIELDS {
        check_workspace_inherited_package_field(package, field, report);
    }

    if package.get("lints").is_some_and(is_workspace_true) {
        return;
    }

    report.issues.push(ManifestIssue::warning(
        FacadeIssueCode::MissingLintsWorkspace,
        "expected [package].lints.workspace = true",
    ));
}

fn missing_package_field_code(field: &str) -> FacadeIssueCode {
    match field {
        "readme" => FacadeIssueCode::MissingPackageReadmeFile,
        "homepage" => FacadeIssueCode::InvalidPackageHomepage,
        "documentation" => FacadeIssueCode::InvalidPackageDocumentation,
        _ => FacadeIssueCode::MissingPackageField,
    }
}

fn analyze_package_categories(
    package: &toml::Table,
    workspace_categories: Option<&[String]>,
    report: &mut ManifestFileReport,
) {
    let Some(categories_value) = package.get("categories") else {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingPackageCategories,
            "missing [package].categories or categories.workspace = true",
        ));
        return;
    };

    if is_workspace_true(categories_value) {
        if workspace_categories.is_none() {
            report.issues.push(ManifestIssue::error(
                FacadeIssueCode::MissingInheritedCategories,
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
            FacadeIssueCode::MissingPackageInheritedField,
            format!("missing [package].{field}.workspace = true"),
        ));
        return;
    };

    if is_workspace_true(value) {
        return;
    }

    report.issues.push(ManifestIssue::warning(
        FacadeIssueCode::PackageFieldNotInherited,
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
            FacadeIssueCode::InvalidCategoriesShape,
            format!("expected {field_name} to be an array of strings"),
        ));
        return None;
    };

    let mut categories = Vec::new();

    for (index, item) in array.iter().enumerate() {
        let Some(category) = item.as_str() else {
            report.issues.push(ManifestIssue::error(
                FacadeIssueCode::InvalidCategoryValue,
                format!("expected {field_name}[{index}] to be a string"),
            ));
            continue;
        };

        categories.push(category.to_owned());
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
            FacadeIssueCode::TooManyCategories,
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
                FacadeIssueCode::DuplicateCategory,
                format!("{field_name} contains duplicate category `{category}`"),
            ));
        }

        if !is_valid_category_slug(category) {
            report.issues.push(ManifestIssue::error(
                FacadeIssueCode::InvalidCategorySlug,
                format!("`{category}` is not a valid crates.io category slug"),
            ));
        }
    }
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
                FacadeIssueCode::MissingStandardWorkspaceMember,
                format!("expected [workspace].members to include `{expected}`"),
            ));
        }
    }

    if members.len() != EXPECTED_WORKSPACE_MEMBERS.len() {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::NonStandardWorkspaceMembers,
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
            FacadeIssueCode::InvalidWorkspaceRepository,
            format!("expected [workspace.package].repository = `{expected}`, found `{actual}`"),
        )),
        None => {},
    }
}

fn validate_workspace_dependencies(
    workspace: &toml::Table,
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    report: &mut ManifestFileReport,
) {
    let Some(dependencies) = workspace
        .get("dependencies")
        .and_then(toml::Value::as_table)
    else {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceDependencies,
            "missing [workspace.dependencies]",
        ));
        return;
    };

    validate_all_workspace_dependency_versions(dependencies, report);
    validate_crate_workspace_dependencies(dependencies, crate_infos, report);
    validate_orphan_workspace_dependency_paths(dependencies, crate_infos, report);
}

fn validate_all_workspace_dependency_versions(
    dependencies: &toml::Table,
    report: &mut ManifestFileReport,
) {
    for (dependency_name, dependency) in dependencies {
        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidWorkspaceDependencyShape,
                format!(
                    "expected [workspace.dependencies].{dependency_name} to be an inline table with version"
                ),
            ));
            continue;
        };

        match dependency_table.get("version") {
            Some(version) if version.as_str().is_some_and(|value| !value.trim().is_empty()) => {},
            Some(_) => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidWorkspaceDependencyVersion,
                format!(
                    "expected [workspace.dependencies].{dependency_name}.version to be a non-empty string"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingWorkspaceDependencyVersion,
                format!("missing [workspace.dependencies].{dependency_name}.version"),
            )),
        }
    }
}

fn validate_crate_workspace_dependencies(
    dependencies: &toml::Table,
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    report: &mut ManifestFileReport,
) {
    for crate_info in crate_infos.values() {
        let dependency_name = crate_info.dependency_name();

        let Some(dependency) = dependencies.get(dependency_name) else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingWorkspaceDependency,
                format!("missing [workspace.dependencies].{dependency_name}"),
            ));
            continue;
        };

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidWorkspaceDependencyShape,
                format!(
                    "expected [workspace.dependencies].{dependency_name} to be an inline table"
                ),
            ));
            continue;
        };

        let expected_path = format!("crates/{}", crate_info.dir_name);

        match dependency_table.get("path").and_then(toml::Value::as_str) {
            Some(actual_path) if actual_path == expected_path => {},
            Some(actual_path) => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidWorkspaceDependencyPath,
                format!(
                    "expected [workspace.dependencies].{dependency_name}.path = `{expected_path}`, found `{actual_path}`"
                ),
            )),
            None => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingWorkspaceDependencyPath,
                format!("missing [workspace.dependencies].{dependency_name}.path"),
            )),
        }

        let actual_version = dependency_table
            .get("version")
            .and_then(toml::Value::as_str);

        if let (Some(expected_version), Some(actual_version)) =
            (crate_info.version.as_deref(), actual_version)
            && actual_version != expected_version
        {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MismatchedWorkspaceDependencyVersion,
                format!(
                    "expected [workspace.dependencies].{dependency_name}.version = `{expected_version}`, found `{actual_version}`"
                ),
            ));
        }
    }
}

fn validate_orphan_workspace_dependency_paths(
    dependencies: &toml::Table,
    crate_infos: &BTreeMap<String, CrateManifestInfo>,
    report: &mut ManifestFileReport,
) {
    let valid_paths = crate_infos
        .values()
        .map(|info| format!("crates/{}", info.dir_name))
        .collect::<BTreeSet<_>>();

    for (dependency_name, dependency) in dependencies {
        let Some(dependency_table) = dependency.as_table() else {
            continue;
        };

        let Some(path) = dependency_table.get("path").and_then(toml::Value::as_str) else {
            continue;
        };

        if path.starts_with("crates/") && !valid_paths.contains(path) {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::OrphanWorkspaceDependencyPath,
                format!(
                    "[workspace.dependencies].{dependency_name}.path points to `{path}`, but no matching crate manifest was found"
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
            FacadeIssueCode::InvalidWorkspaceUnsafeCodePolicy,
            format!("expected [workspace.lints.rust].unsafe_code = \"forbid\", found `{actual}`"),
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceUnsafeCodePolicy,
            "missing [workspace.lints.rust].unsafe_code = \"forbid\"",
        )),
    }

    let clippy = workspace
        .get("lints")
        .and_then(|lints| lints.get("clippy"))
        .and_then(toml::Value::as_table);

    if clippy.is_none() {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingWorkspaceClippyLints,
            "missing [workspace.lints.clippy]",
        ));
    }
}

fn validate_facade_dependency_and_feature_wiring(
    manifest: &toml::Value,
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
            FacadeIssueCode::MissingFacadeDependencies,
            "facade crate has child crates but no [dependencies]",
        ));
        return;
    };

    for child_crate in child_crate_names {
        let Some(dependency) = dependencies.get(child_crate) else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingFacadeChildDependency,
                format!("missing [dependencies].{child_crate}"),
            ));
            continue;
        };

        let Some(dependency_table) = dependency.as_table() else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidFacadeChildDependency,
                format!("expected [dependencies].{child_crate} to be an inline table"),
            ));
            continue;
        };

        if !is_workspace_true(dependency) {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidFacadeChildDependency,
                format!("expected [dependencies].{child_crate}.workspace = true"),
            ));
        }

        match dependency_table
            .get("optional")
            .and_then(toml::Value::as_bool)
        {
            Some(true) => {},
            Some(false) => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidFacadeChildDependency,
                format!("expected [dependencies].{child_crate}.optional = true"),
            )),
            None => report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingFacadeChildDependencyOptional,
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
            FacadeIssueCode::MissingFacadeFeatures,
            "facade crate has child crates but no [features]",
        ));
        return;
    };

    match features.get("default").and_then(toml::Value::as_array) {
        Some(default_features) if default_features.is_empty() => {},
        Some(_) => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::InvalidFacadeDefaultFeatures,
            "expected [features].default = []",
        )),
        None => report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingFacadeDefaultFeatures,
            "missing [features].default = []",
        )),
    }

    let full_features = features
        .get("full")
        .and_then(toml::Value::as_array)
        .map(|values| array_string_set(values));

    if full_features.is_none() {
        report.issues.push(ManifestIssue::warning(
            FacadeIssueCode::MissingFacadeFullFeature,
            "missing [features].full",
        ));
    }

    for child_crate in child_crate_names {
        let feature_name = feature_name_for_child_crate(child_crate);

        /* if let Some(full_features) = &full_features {
            if !full_features.contains(feature_name.as_str()) {
                report.issues.push(ManifestIssue::warning(
                    FacadeIssueCode::MissingFullFeatureMember,
                    format!("expected [features].full to include `{feature_name}`"),
                ));
            }
        } */

        if let Some(full_features) = &full_features
            && !full_features.contains(feature_name.as_str())
        {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingFullFeatureMember,
                format!("expected [features].full to include `{feature_name}`"),
            ));
        }

        let Some(feature_values) = features
            .get(feature_name.as_str())
            .and_then(toml::Value::as_array)
        else {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::MissingFacadeChildFeature,
                format!("missing [features].{feature_name}"),
            ));
            continue;
        };

        let feature_values = array_string_set(feature_values);
        let expected_dep = format!("dep:{child_crate}");

        if !feature_values.contains(expected_dep.as_str()) {
            report.issues.push(ManifestIssue::warning(
                FacadeIssueCode::InvalidFacadeChildFeature,
                format!("expected [features].{feature_name} to include `{expected_dep}`"),
            ));
        }
    }
}

/* fn has_string_or_workspace_true(package: &toml::Table, field: &'static str) -> bool {
    let Some(value) = package.get(field) else {
        return false;
    };

    value.as_str().is_some() || is_workspace_true(value)
} */

fn is_workspace_true(value: &toml::Value) -> bool {
    value
        .as_table()
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct CrateManifestInfo {
    dir_name: String,
    package_name: Option<String>,
    version: Option<String>,
}

impl CrateManifestInfo {
    fn dependency_name(&self) -> &str {
        self.package_name.as_deref().unwrap_or(&self.dir_name)
    }
}

fn collect_crate_infos(crate_manifest_paths: &[PathBuf]) -> BTreeMap<String, CrateManifestInfo> {
    let mut crate_infos = BTreeMap::new();

    for manifest_path in crate_manifest_paths {
        let Some(dir_name) = crate_dir_name(manifest_path) else {
            continue;
        };

        let raw = fs::read_to_string(manifest_path).ok();
        let manifest = raw
            .as_deref()
            .and_then(|raw| toml::from_str::<toml::Value>(raw).ok());

        let package = manifest
            .as_ref()
            .and_then(|manifest| manifest.get("package"))
            .and_then(toml::Value::as_table);

        let package_name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .map(ToOwned::to_owned);

        let version = package
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
            .map(ToOwned::to_owned);

        let info = CrateManifestInfo {
            dir_name,
            package_name,
            version,
        };

        crate_infos.insert(info.dependency_name().to_owned(), info);
    }

    crate_infos
}

fn feature_name_for_child_crate(crate_name: &str) -> String {
    crate_name
        .strip_prefix("use-")
        .unwrap_or(crate_name)
        .to_owned()
}

fn array_string_set(array: &[toml::Value]) -> BTreeSet<String> {
    array
        .iter()
        .filter_map(toml::Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
}

fn manifest_diagnostic_code(code: &str) -> FacadeIssueCode {
    match code {
        "read-manifest" => FacadeIssueCode::ReadManifest,
        "parse-manifest" => FacadeIssueCode::ParseManifest,
        _ => FacadeIssueCode::InvalidManifest,
    }
}

fn read_manifest(path: &Path, report: &mut ManifestFileReport) -> Option<toml::Value> {
    let read = read_manifest_with_diagnostics(path);

    if let Some(diagnostic) = read.diagnostic() {
        report.issues.push(ManifestIssue::error(
            manifest_diagnostic_code(diagnostic.code),
            diagnostic.message.clone(),
        ));
    }

    read.into_value()
}
