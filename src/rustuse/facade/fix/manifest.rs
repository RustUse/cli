//! Cargo manifest discovery and repair for RustUse facades.
//!
//! This module performs TOML parsing and transformation only. It does not
//! write files or produce terminal output.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::rustuse::config::{
    crate_documentation_url, facade_documentation_url, facade_repository_url,
};

const EXPECTED_WORKSPACE_MEMBERS: &[&str] = &["crates/*"];
const EXPECTED_DEFAULT_MEMBERS: &[&str] = &["."];

const DEFAULT_WORKSPACE_AUTHORS: &[&str] = &["RustUse Contributors"];
const DEFAULT_WORKSPACE_CATEGORIES: &[&str] = &["data-structures", "development-tools"];

const DEFAULT_EDITION: &str = "2024";
const DEFAULT_LICENSE: &str = "MIT OR Apache-2.0";
const DEFAULT_RUST_VERSION: &str = "1.95.0";
const DEFAULT_PACKAGE_VERSION: &str = "0.1.0";
const DEFAULT_RESOLVER: &str = "3";

const INHERITED_PACKAGE_FIELDS: &[&str] = &[
    "authors",
    "edition",
    "rust-version",
    "license",
    "repository",
];

const WORKSPACE_DEPENDENCY_SOURCE_FIELDS: &[&str] = &[
    "version",
    "path",
    "git",
    "branch",
    "tag",
    "rev",
    "registry",
    "registry-index",
    "package",
];

/// Selects which parts of a facade manifest should be repaired.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct FacadeManifestRepairs {
    pub(crate) workspace_shape: bool,
    pub(crate) workspace_dependencies: bool,
    pub(crate) facade_wiring: bool,
    pub(crate) package_metadata: bool,
}

/// Manifest information discovered for one child crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CrateInfo {
    pub(crate) dir_name: String,
    pub(crate) package_name: String,
    pub(crate) version: Option<String>,
    pub(crate) manifest_path: PathBuf,
}

impl CrateInfo {
    pub(crate) fn dependency_name(&self) -> &str {
        &self.package_name
    }

    pub(crate) fn feature_name(&self) -> String {
        self.package_name
            .strip_prefix("use-")
            .unwrap_or(&self.package_name)
            .to_owned()
    }

    fn effective_version(&self) -> &str {
        self.version.as_deref().unwrap_or(DEFAULT_PACKAGE_VERSION)
    }
}

/// Discovers child crates beneath a facade's `crates/` directory.
///
/// A crate whose directory or package name matches the facade name is
/// intentionally excluded. The facade package belongs at the repository root
/// and must not also exist under `crates/`.
pub(crate) fn discover_child_crates(
    crates_dir: &Path,
    facade_name: &str,
) -> Result<Vec<CrateInfo>> {
    if !crates_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut child_crates = Vec::new();

    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry = entry.with_context(|| {
            format!("failed to read an entry beneath `{}`", crates_dir.display())
        })?;

        let crate_dir = entry.path();

        if !crate_dir.is_dir() {
            continue;
        }

        let manifest_path = crate_dir.join("Cargo.toml");

        if !manifest_path.is_file() {
            continue;
        }

        let dir_name = crate_dir
            .file_name()
            .and_then(|name| name.to_str())
            .with_context(|| {
                format!(
                    "crate directory `{}` does not have a valid UTF-8 name",
                    crate_dir.display()
                )
            })?
            .to_owned();

        let original = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read `{}`", manifest_path.display()))?;

        let manifest = parse_manifest(&manifest_path, &original)?;
        let package = manifest.get("package").and_then(toml::Value::as_table);

        let package_name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or(&dir_name)
            .to_owned();

        if dir_name == facade_name || package_name == facade_name {
            continue;
        }

        let version = package
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
            .map(ToOwned::to_owned);

        child_crates.push(CrateInfo {
            dir_name,
            package_name,
            version,
            manifest_path,
        });
    }

    child_crates.sort_by(|left, right| {
        left.package_name
            .cmp(&right.package_name)
            .then_with(|| left.dir_name.cmp(&right.dir_name))
    });

    Ok(child_crates)
}

/// Repairs the combined package/workspace manifest at a facade repository root.
pub(crate) fn repair_facade_manifest(
    manifest_path: &Path,
    original: &str,
    facade_name: &str,
    child_crates: &[CrateInfo],
    repairs: FacadeManifestRepairs,
) -> Result<String> {
    let mut manifest = parse_manifest(manifest_path, original)?;

    {
        let root = manifest
            .as_table_mut()
            .context("expected Cargo.toml document to be a TOML table")?;

        if repairs.workspace_shape {
            repair_workspace_shape(root, facade_name);
        }

        if repairs.workspace_dependencies || repairs.facade_wiring {
            repair_workspace_dependencies(root, child_crates);
        }

        if repairs.package_metadata {
            repair_package_metadata(root, facade_name, facade_name, true);
        }

        if repairs.facade_wiring {
            repair_facade_wiring(root, child_crates);
        }
    }

    render_manifest(manifest_path, &manifest)
}

/// Repairs package metadata for one child crate manifest.
pub(crate) fn repair_child_manifest(
    manifest_path: &Path,
    original: &str,
    facade_name: &str,
    child: &CrateInfo,
) -> Result<String> {
    let mut manifest = parse_manifest(manifest_path, original)?;

    {
        let root = manifest
            .as_table_mut()
            .context("expected Cargo.toml document to be a TOML table")?;

        repair_package_metadata(root, facade_name, &child.package_name, false);
    }

    render_manifest(manifest_path, &manifest)
}

fn repair_workspace_shape(root: &mut toml::Table, facade_name: &str) {
    let workspace = ensure_table(root, "workspace");

    set_string_array(workspace, "members", EXPECTED_WORKSPACE_MEMBERS);

    set_string_array(workspace, "default-members", EXPECTED_DEFAULT_MEMBERS);

    set_string(workspace, "resolver", DEFAULT_RESOLVER);

    let package = ensure_table(workspace, "package");

    set_string_array_if_missing(package, "authors", DEFAULT_WORKSPACE_AUTHORS);

    set_string_if_missing(package, "edition", DEFAULT_EDITION);
    set_string_if_missing(package, "license", DEFAULT_LICENSE);
    set_string_if_missing(package, "rust-version", DEFAULT_RUST_VERSION);

    set_string(package, "repository", &facade_repository_url(facade_name));

    set_string_array_if_missing(package, "categories", DEFAULT_WORKSPACE_CATEGORIES);

    let lints = ensure_table(workspace, "lints");

    let rust_lints = ensure_table(lints, "rust");
    set_string(rust_lints, "unsafe_code", "forbid");

    let clippy_lints = ensure_table(lints, "clippy");
    repair_workspace_clippy_lints(clippy_lints);
}

fn repair_workspace_dependencies(root: &mut toml::Table, child_crates: &[CrateInfo]) {
    let workspace = ensure_table(root, "workspace");
    let dependencies = ensure_table(workspace, "dependencies");

    for child in child_crates {
        let mut dependency = toml::Table::new();

        dependency.insert(
            "version".to_owned(),
            toml::Value::String(child.effective_version().to_owned()),
        );

        dependency.insert(
            "path".to_owned(),
            toml::Value::String(format!("crates/{}", child.dir_name)),
        );

        dependencies.insert(
            child.dependency_name().to_owned(),
            toml::Value::Table(dependency),
        );
    }
}

fn repair_package_metadata(
    root: &mut toml::Table,
    facade_name: &str,
    package_name: &str,
    is_facade_package: bool,
) {
    let package = ensure_table(root, "package");

    set_string_if_missing(package, "name", package_name);
    set_string_if_missing(package, "version", DEFAULT_PACKAGE_VERSION);

    set_bool_if_missing(package, "publish", true);
    set_string_if_missing(package, "readme", "README.md");

    set_string(
        package,
        "documentation",
        &format!("https://docs.rs/{package_name}"),
    );

    let homepage = if is_facade_package {
        facade_documentation_url(facade_name)
    } else {
        crate_documentation_url(facade_name, package_name)
    };

    set_string(package, "homepage", &homepage);

    for field in INHERITED_PACKAGE_FIELDS {
        set_workspace_true(package, field);
    }

    set_workspace_true(package, "categories");

    let docs_rs = ensure_dotted_table(root, &["package", "metadata", "docs", "rs"]);

    docs_rs.insert("all-features".to_owned(), toml::Value::Boolean(true));

    let lints = ensure_table(root, "lints");
    set_workspace_true(lints, "workspace");
}

fn repair_facade_wiring(root: &mut toml::Table, child_crates: &[CrateInfo]) {
    repair_facade_dependencies(root, child_crates);
    repair_facade_features(root, child_crates);
}

fn repair_facade_dependencies(root: &mut toml::Table, child_crates: &[CrateInfo]) {
    let dependencies = ensure_table(root, "dependencies");

    for child in child_crates {
        let dependency_name = child.dependency_name();

        let mut dependency = dependencies
            .get(dependency_name)
            .and_then(toml::Value::as_table)
            .cloned()
            .unwrap_or_default();

        for source_field in WORKSPACE_DEPENDENCY_SOURCE_FIELDS {
            dependency.remove(*source_field);
        }

        dependency.insert("workspace".to_owned(), toml::Value::Boolean(true));

        dependency.insert("optional".to_owned(), toml::Value::Boolean(true));

        dependencies.insert(dependency_name.to_owned(), toml::Value::Table(dependency));
    }
}

fn repair_facade_features(root: &mut toml::Table, child_crates: &[CrateInfo]) {
    let features = ensure_table(root, "features");

    features.insert("default".to_owned(), toml::Value::Array(Vec::new()));

    features.insert(
        "full".to_owned(),
        toml::Value::Array(
            child_crates
                .iter()
                .map(CrateInfo::feature_name)
                .map(toml::Value::String)
                .collect(),
        ),
    );

    for child in child_crates {
        features.insert(
            child.feature_name(),
            toml::Value::Array(vec![toml::Value::String(format!(
                "dep:{}",
                child.dependency_name()
            ))]),
        );
    }
}

fn repair_workspace_clippy_lints(clippy: &mut toml::Table) {
    set_lint_table_if_missing(clippy, "all", "warn", -1);

    set_lint_table_if_missing(clippy, "cargo", "warn", -1);

    set_lint_table_if_missing(clippy, "nursery", "warn", -1);

    set_lint_table_if_missing(clippy, "pedantic", "warn", -1);

    set_string_if_missing(clippy, "derivable_impls", "allow");

    set_string_if_missing(clippy, "doc_markdown", "allow");

    set_string_if_missing(clippy, "expect_used", "warn");

    set_string_if_missing(clippy, "missing_const_for_fn", "allow");

    set_string_if_missing(clippy, "missing_errors_doc", "allow");

    set_string_if_missing(clippy, "module_name_repetitions", "allow");

    set_string_if_missing(clippy, "multiple_crate_versions", "allow");

    set_string_if_missing(clippy, "must_use_candidate", "allow");

    set_string_if_missing(clippy, "return_self_not_must_use", "allow");

    set_string_if_missing(clippy, "todo", "deny");
    set_string_if_missing(clippy, "unimplemented", "deny");

    set_string_if_missing(clippy, "unwrap_used", "warn");
}

fn parse_manifest(manifest_path: &Path, original: &str) -> Result<toml::Value> {
    if original.trim().is_empty() {
        return Ok(toml::Value::Table(toml::Table::new()));
    }

    toml::from_str(original)
        .with_context(|| format!("failed to parse `{}`", manifest_path.display()))
}

fn render_manifest(manifest_path: &Path, manifest: &toml::Value) -> Result<String> {
    let mut rendered = toml::to_string_pretty(manifest)
        .with_context(|| format!("failed to render `{}`", manifest_path.display()))?;

    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }

    Ok(rendered)
}

fn ensure_table<'a>(table: &'a mut toml::Table, key: &str) -> &'a mut toml::Table {
    let is_table = table.get(key).and_then(toml::Value::as_table).is_some();

    if !is_table {
        table.insert(key.to_owned(), toml::Value::Table(toml::Table::new()));
    }

    table
        .get_mut(key)
        .and_then(toml::Value::as_table_mut)
        .expect("table was inserted immediately above")
}

fn ensure_dotted_table<'a>(table: &'a mut toml::Table, path: &[&str]) -> &'a mut toml::Table {
    let mut current = table;

    for key in path {
        current = ensure_table(current, key);
    }

    current
}

fn set_string(table: &mut toml::Table, key: &str, value: &str) {
    table.insert(key.to_owned(), toml::Value::String(value.to_owned()));
}

fn set_string_if_missing(table: &mut toml::Table, key: &str, value: &str) {
    if table.contains_key(key) {
        return;
    }

    set_string(table, key, value);
}

fn set_bool_if_missing(table: &mut toml::Table, key: &str, value: bool) {
    if table.contains_key(key) {
        return;
    }

    table.insert(key.to_owned(), toml::Value::Boolean(value));
}

fn set_string_array(table: &mut toml::Table, key: &str, values: &[&str]) {
    table.insert(
        key.to_owned(),
        toml::Value::Array(
            values
                .iter()
                .map(|value| toml::Value::String((*value).to_owned()))
                .collect(),
        ),
    );
}

fn set_string_array_if_missing(table: &mut toml::Table, key: &str, values: &[&str]) {
    if table.contains_key(key) {
        return;
    }

    set_string_array(table, key, values);
}

fn set_workspace_true(table: &mut toml::Table, key: &str) {
    let mut workspace = toml::Table::new();

    workspace.insert("workspace".to_owned(), toml::Value::Boolean(true));

    table.insert(key.to_owned(), toml::Value::Table(workspace));
}

fn set_lint_table_if_missing(table: &mut toml::Table, key: &str, level: &str, priority: i64) {
    if table.contains_key(key) {
        return;
    }

    let mut lint = toml::Table::new();

    lint.insert("level".to_owned(), toml::Value::String(level.to_owned()));

    lint.insert("priority".to_owned(), toml::Value::Integer(priority));

    table.insert(key.to_owned(), toml::Value::Table(lint));
}
