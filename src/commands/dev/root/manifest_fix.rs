use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::discover::{FacadeEntry, discover_facades};
use crate::output::Output;

const EXPECTED_WORKSPACE_MEMBERS: &[&str] = &["crates/*"];
const RUSTUSE_GITHUB_ORG: &str = "https://github.com/RustUse";
const RUSTUSE_DOCS_ORIGIN: &str = "https://rustuse.org";

const DEFAULT_WORKSPACE_AUTHORS: &[&str] = &["RustUse Contributors"];
const DEFAULT_EDITION: &str = "2024";
const DEFAULT_LICENSE: &str = "MIT OR Apache-2.0";
const DEFAULT_RUST_VERSION: &str = "1.95.0";
const DEFAULT_WORKSPACE_CATEGORIES: &[&str] = &["data-structures", "development-tools"];

const INHERITED_PACKAGE_FIELDS: &[&str] = &[
    "authors",
    "edition",
    "rust-version",
    "license",
    "repository",
];

#[derive(Debug, Clone)]
pub(crate) struct ManifestFixOptions {
    pub(crate) root: PathBuf,
    pub(crate) facade: Option<String>,
    pub(crate) code: Option<String>,
    pub(crate) write: bool,
}

impl ManifestFixOptions {
    pub(crate) fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            facade: None,
            code: None,
            write: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ManifestFixGroup {
    All,
    FacadeWiring,
    WorkspaceShape,
    WorkspaceDependencies,
    PackageMetadata,
}

impl ManifestFixGroup {
    fn from_code(code: Option<&str>) -> Result<Self> {
        let Some(code) = code else {
            return Ok(Self::All);
        };

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

    fn fixes_facade_wiring(self) -> bool {
        matches!(self, Self::All | Self::FacadeWiring)
    }

    fn fixes_workspace_shape(self) -> bool {
        matches!(self, Self::All | Self::WorkspaceShape)
    }

    fn fixes_workspace_dependencies(self) -> bool {
        matches!(self, Self::All | Self::WorkspaceDependencies)
    }

    fn fixes_package_metadata(self) -> bool {
        matches!(self, Self::All | Self::PackageMetadata)
    }
}

#[derive(Debug, Default)]
pub(crate) struct ManifestFixSummary {
    pub(crate) facades_inspected: usize,
    pub(crate) files_changed: usize,
    pub(crate) files_unchanged: usize,
    pub(crate) files_created: usize,
    pub(crate) skipped_facades: usize,
    pub(crate) changes: Vec<ManifestFileChange>,
}

#[derive(Debug, Clone)]
pub(crate) struct ManifestFileChange {
    pub(crate) path: PathBuf,
    pub(crate) created: bool,
    pub(crate) wrote: bool,
}

pub(crate) fn run(options: ManifestFixOptions, output: Output) -> Result<ManifestFixSummary> {
    let summary = fix_manifests(&options)?;

    output.line(format!(
        "RustUse dev root manifests fix - root: {}",
        options.root.display()
    ));

    output.line(format!(
        "mode: {}",
        if options.write { "write" } else { "dry-run" }
    ));
    output.line(format!("facades inspected: {}", summary.facades_inspected));
    output.line(format!("skipped facades: {}", summary.skipped_facades));
    output.line(format!("files changed: {}", summary.files_changed));
    output.line(format!("files created: {}", summary.files_created));
    output.line(format!("files unchanged: {}", summary.files_unchanged));

    if !summary.changes.is_empty() {
        output.line("");
        output.line("changed files:");

        for change in &summary.changes {
            let action = if change.wrote { "wrote" } else { "would write" };
            let created = if change.created { " created" } else { "" };

            output.line(format!("- {action}{created}: {}", change.path.display()));
        }
    }

    Ok(summary)
}

pub(crate) fn fix_manifests(options: &ManifestFixOptions) -> Result<ManifestFixSummary> {
    let root = fs::canonicalize(&options.root).unwrap_or_else(|_| options.root.clone());
    let facades = discover_facades(&root)?;
    let group = ManifestFixGroup::from_code(options.code.as_deref())?;

    let mut summary = ManifestFixSummary::default();

    for facade in facades {
        if !matches_facade_filter(&facade, options.facade.as_deref()) {
            summary.skipped_facades += 1;
            continue;
        }

        summary.facades_inspected += 1;
        fix_facade_manifests(&root, &facade, group, options.write, &mut summary)?;
    }

    Ok(summary)
}

fn fix_facade_manifests(
    root: &Path,
    facade: &FacadeEntry,
    group: ManifestFixGroup,
    write: bool,
    summary: &mut ManifestFixSummary,
) -> Result<()> {
    let facade_root = root.join(&facade.name);
    let crates_dir = facade_root.join("crates");
    let workspace_manifest_path = facade_root.join("Cargo.toml");

    let crate_infos = discover_crate_infos(&crates_dir)?;

    let Some(facade_crate) = find_facade_crate(&crate_infos, &facade.name).cloned() else {
        /* if group.fixes_workspace_shape() || group.fixes_workspace_dependencies() {
            let child_crates = child_crates(&crate_infos, &facade.name);
            fix_workspace_manifest(
                root,
                &workspace_manifest_path,
                &facade.name,
                &child_crates,
                group,
                write,
                summary,
            )?;
        } */

        if group.fixes_workspace_shape()
            || group.fixes_workspace_dependencies()
            || group.fixes_facade_wiring()
        {
            let child_crates = child_crates(&crate_infos, &facade.name);
            fix_workspace_manifest(
                root,
                &workspace_manifest_path,
                &facade.name,
                &child_crates,
                group,
                write,
                summary,
            )?;
        }

        return Ok(());
    };

    let child_crates = child_crates(&crate_infos, &facade.name);

    /* if group.fixes_workspace_shape() || group.fixes_workspace_dependencies() {
        fix_workspace_manifest(
            root,
            &workspace_manifest_path,
            &facade.name,
            &child_crates,
            group,
            write,
            summary,
        )?;
    } */

    if group.fixes_workspace_shape()
        || group.fixes_workspace_dependencies()
        || group.fixes_facade_wiring()
    {
        fix_workspace_manifest(
            root,
            &workspace_manifest_path,
            &facade.name,
            &child_crates,
            group,
            write,
            summary,
        )?;
    }

    if group.fixes_facade_wiring() || group.fixes_package_metadata() {
        fix_package_manifest(
            root,
            &facade_crate,
            &facade.name,
            &child_crates,
            group,
            true,
            write,
            summary,
        )?;
    }

    if group.fixes_package_metadata() {
        for child in &child_crates {
            fix_package_manifest(root, child, &facade.name, &[], group, false, write, summary)?;
        }
    }

    Ok(())
}

fn fix_workspace_manifest(
    root: &Path,
    manifest_path: &Path,
    facade_name: &str,
    child_crates: &[CrateInfo],
    group: ManifestFixGroup,
    write: bool,
    summary: &mut ManifestFixSummary,
) -> Result<()> {
    let created = !manifest_path.exists();
    let original = fs::read_to_string(manifest_path).unwrap_or_default();

    let mut manifest = if original.trim().is_empty() {
        toml::Value::Table(toml::Table::new())
    } else {
        toml::from_str::<toml::Value>(&original)
            .with_context(|| format!("failed to parse `{}`", manifest_path.display()))?
    };

    {
        let root_table = manifest
            .as_table_mut()
            .context("expected Cargo.toml document to be a TOML table")?;

        let workspace = ensure_table(root_table, "workspace");

        if group.fixes_workspace_shape() {
            workspace.insert(
                "members".to_string(),
                toml::Value::Array(
                    EXPECTED_WORKSPACE_MEMBERS
                        .iter()
                        .map(|member| toml::Value::String((*member).to_string()))
                        .collect(),
                ),
            );

            workspace.insert("resolver".to_string(), toml::Value::String("3".to_string()));

            let package = ensure_table(workspace, "package");

            set_string_array_if_missing(package, "authors", DEFAULT_WORKSPACE_AUTHORS);
            set_string_if_missing(package, "edition", DEFAULT_EDITION);
            set_string_if_missing(package, "license", DEFAULT_LICENSE);
            set_string_if_missing(
                package,
                "repository",
                &format!("{RUSTUSE_GITHUB_ORG}/{facade_name}"),
            );
            set_string_if_missing(package, "rust-version", DEFAULT_RUST_VERSION);

            if !package.contains_key("categories") {
                package.insert(
                    "categories".to_string(),
                    toml::Value::Array(
                        DEFAULT_WORKSPACE_CATEGORIES
                            .iter()
                            .map(|category| toml::Value::String((*category).to_string()))
                            .collect(),
                    ),
                );
            }

            let lints = ensure_table(workspace, "lints");
            let rust_lints = ensure_table(lints, "rust");
            rust_lints.insert(
                "unsafe_code".to_string(),
                toml::Value::String("forbid".to_string()),
            );

            let clippy_lints = ensure_table(lints, "clippy");
            set_string_if_missing(clippy_lints, "pedantic", "warn");
            set_string_if_missing(clippy_lints, "nursery", "warn");
        }

        /* if group.fixes_workspace_dependencies() {
            let workspace_dependencies = ensure_table(workspace, "dependencies");

            for child in child_crates {
                let mut dependency = toml::Table::new();

                if let Some(version) = &child.version {
                    dependency.insert("version".to_string(), toml::Value::String(version.clone()));
                }

                dependency.insert(
                    "path".to_string(),
                    toml::Value::String(format!("crates/{}", child.dir_name)),
                );

                workspace_dependencies.insert(
                    child.dependency_name().to_string(),
                    toml::Value::Table(dependency),
                );
            }
        } */

        if group.fixes_workspace_dependencies() || group.fixes_facade_wiring() {
            let workspace_dependencies = ensure_table(workspace, "dependencies");

            for child in child_crates {
                let mut dependency = toml::Table::new();

                if let Some(version) = &child.version {
                    dependency.insert("version".to_string(), toml::Value::String(version.clone()));
                }

                dependency.insert(
                    "path".to_string(),
                    toml::Value::String(format!("crates/{}", child.dir_name)),
                );

                workspace_dependencies.insert(
                    child.dependency_name().to_string(),
                    toml::Value::Table(dependency),
                );
            }
        }
    }

    /* write_if_changed(
        root,
        manifest_path,
        created,
        &original,
        &manifest,
        write,
        summary,
    ) */

    let rendered = render_workspace_manifest(&manifest, facade_name, child_crates);

    write_rendered_if_changed(
        root,
        manifest_path,
        created,
        &original,
        rendered,
        write,
        summary,
    )
}

fn fix_package_manifest(
    root: &Path,
    crate_info: &CrateInfo,
    facade_name: &str,
    child_crates: &[CrateInfo],
    group: ManifestFixGroup,
    is_facade_package: bool,
    write: bool,
    summary: &mut ManifestFixSummary,
) -> Result<()> {
    let manifest_path = &crate_info.manifest_path;
    let created = !manifest_path.exists();
    let original = fs::read_to_string(manifest_path).unwrap_or_default();

    let mut manifest = if original.trim().is_empty() {
        toml::Value::Table(toml::Table::new())
    } else {
        toml::from_str::<toml::Value>(&original)
            .with_context(|| format!("failed to parse `{}`", manifest_path.display()))?
    };

    {
        let root_table = manifest
            .as_table_mut()
            .context("expected Cargo.toml document to be a TOML table")?;

        if group.fixes_package_metadata() {
            let package = ensure_table(root_table, "package");

            set_string_if_missing(package, "name", &crate_info.package_name);

            if crate_info.version.is_none() {
                set_string_if_missing(package, "version", "0.1.0");
            }

            if !package.contains_key("publish") {
                package.insert("publish".to_string(), toml::Value::Boolean(true));
            }

            set_string_if_missing(package, "readme", "README.md");
            set_string_if_missing(
                package,
                "documentation",
                &format!("https://docs.rs/{}", crate_info.package_name),
            );

            let homepage = if is_facade_package {
                format!("{RUSTUSE_DOCS_ORIGIN}/{facade_name}")
            } else {
                format!(
                    "{RUSTUSE_DOCS_ORIGIN}/{facade_name}/{}",
                    crate_info.package_name
                )
            };

            set_string_if_missing(package, "homepage", &homepage);

            for field in INHERITED_PACKAGE_FIELDS {
                set_workspace_true(package, field);
            }

            if !package.contains_key("categories") {
                set_workspace_true(package, "categories");
            }

            let docs_rs = ensure_dotted_table(root_table, &["package", "metadata", "docs", "rs"]);
            docs_rs.insert("all-features".to_string(), toml::Value::Boolean(true));

            let lints = ensure_table(root_table, "lints");
            set_workspace_true(lints, "workspace");
        }

        if is_facade_package && group.fixes_facade_wiring() {
            let dependencies = ensure_table(root_table, "dependencies");

            for child in child_crates {
                let mut dependency = toml::Table::new();
                dependency.insert("workspace".to_string(), toml::Value::Boolean(true));
                dependency.insert("optional".to_string(), toml::Value::Boolean(true));

                dependencies.insert(
                    child.dependency_name().to_string(),
                    toml::Value::Table(dependency),
                );
            }

            let features = ensure_table(root_table, "features");

            features.insert("default".to_string(), toml::Value::Array(Vec::new()));

            let full_features = child_crates
                .iter()
                .map(|child| toml::Value::String(child.feature_name()))
                .collect::<Vec<_>>();

            features.insert("full".to_string(), toml::Value::Array(full_features));

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
    }

    if is_facade_package && (group.fixes_facade_wiring() || group.fixes_package_metadata()) {
        let rendered = render_facade_package_manifest(&manifest, facade_name, child_crates);

        return write_rendered_if_changed(
            root,
            manifest_path,
            created,
            &original,
            rendered,
            write,
            summary,
        );
    }

    write_if_changed(
        root,
        manifest_path,
        created,
        &original,
        &manifest,
        write,
        summary,
    )
}

fn render_facade_package_manifest(
    manifest: &toml::Value,
    facade_name: &str,
    child_crates: &[CrateInfo],
) -> String {
    let package = manifest.get("package").and_then(toml::Value::as_table);

    let name = string_field(package, "name").unwrap_or(facade_name);
    let version = string_field(package, "version").unwrap_or("0.1.0");
    let publish = bool_field(package, "publish").unwrap_or(true);
    let keywords =
        array_string_field(package, "keywords").unwrap_or_else(|| vec!["rustuse".to_string()]);
    let description =
        string_field(package, "description").unwrap_or("Facade crate for RustUse primitives");
    let homepage = format!("{RUSTUSE_DOCS_ORIGIN}/{facade_name}");
    let documentation = format!("https://docs.rs/{name}");
    let readme = string_field(package, "readme").unwrap_or("README.md");

    let mut out = String::new();

    out.push_str("[package]\n");
    out.push_str(&format!("name = {}\n", render_toml_string(name)));
    out.push_str(&format!("version = {}\n", render_toml_string(version)));
    out.push_str(&format!("publish = {publish}\n"));
    out.push_str(&format!("keywords = {}\n", render_inline_array(&keywords)));
    out.push_str(&format!(
        "description = {}\n",
        render_toml_string(description)
    ));
    out.push_str(&format!("homepage = {}\n", render_toml_string(&homepage)));
    out.push_str(&format!(
        "documentation = {}\n",
        render_toml_string(&documentation)
    ));
    out.push_str(&format!("readme = {}\n", render_toml_string(readme)));
    out.push_str("authors.workspace = true\n");
    out.push_str("edition.workspace = true\n");
    out.push_str("rust-version.workspace = true\n");
    out.push_str("license.workspace = true\n");
    out.push_str("repository.workspace = true\n");
    out.push_str("categories.workspace = true\n\n");

    out.push_str("[package.metadata.docs.rs]\n");
    out.push_str("all-features = true\n\n");

    out.push_str("[features]\n");
    out.push_str("default = []\n\n");

    out.push_str("full = [\n");
    for child in child_crates {
        out.push_str(&format!(
            "    {},\n",
            render_toml_string(&child.feature_name())
        ));
    }
    out.push_str("]\n\n");

    for child in child_crates {
        out.push_str(&format!(
            "{} = [{}]\n",
            child.feature_name(),
            render_toml_string(&format!("dep:{}", child.dependency_name()))
        ));
    }

    out.push('\n');
    out.push_str("[dependencies]\n");

    for child in child_crates {
        out.push_str(&format!(
            "{} = {{ workspace = true, optional = true }}\n",
            child.dependency_name()
        ));
    }

    append_existing_dependency_table(manifest, "dev-dependencies", &mut out);
    append_existing_dependency_table(manifest, "build-dependencies", &mut out);
    append_existing_array_of_tables(manifest, "example", "example", &mut out);
    append_existing_array_of_tables(manifest, "bin", "bin", &mut out);
    append_existing_array_of_tables(manifest, "test", "test", &mut out);
    append_existing_array_of_tables(manifest, "bench", "bench", &mut out);
    append_unhandled_top_level_sections(manifest, &mut out);

    out.push('\n');
    out.push_str("[lints]\n");
    out.push_str("workspace = true\n");

    if !out.ends_with('\n') {
        out.push('\n');
    }

    out
}

fn append_existing_dependency_table(manifest: &toml::Value, key: &str, out: &mut String) {
    let Some(table) = manifest.get(key).and_then(toml::Value::as_table) else {
        return;
    };

    if table.is_empty() {
        return;
    }

    out.push('\n');
    out.push_str(&format!("[{key}]\n"));

    for (name, value) in table {
        out.push_str(&format!("{name} = {}\n", render_dependency_value(value)));
    }
}

fn append_existing_array_of_tables(
    manifest: &toml::Value,
    source_key: &str,
    rendered_key: &str,
    out: &mut String,
) {
    let Some(items) = manifest.get(source_key).and_then(toml::Value::as_array) else {
        return;
    };

    for item in items {
        let Some(table) = item.as_table() else {
            continue;
        };

        out.push('\n');
        out.push_str(&format!("[[{rendered_key}]]\n"));

        if let Some(name) = table.get("name").and_then(toml::Value::as_str) {
            out.push_str(&format!("name = {}\n", render_toml_string(name)));
        }

        if let Some(path) = table.get("path").and_then(toml::Value::as_str) {
            out.push_str(&format!("path = {}\n", render_toml_string(path)));
        }

        if let Some(required_features) = table
            .get("required-features")
            .and_then(toml::Value::as_array)
        {
            let required_features = required_features
                .iter()
                .filter_map(toml::Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();

            out.push_str(&format!(
                "required-features = {}\n",
                render_inline_array(&required_features)
            ));
        }

        for (key, value) in table {
            if matches!(key.as_str(), "name" | "path" | "required-features") {
                continue;
            }

            out.push_str(&format!("{key} = {}\n", render_toml_value(value)));
        }
    }
}

fn append_unhandled_top_level_sections(manifest: &toml::Value, out: &mut String) {
    let Some(table) = manifest.as_table() else {
        return;
    };

    for (key, value) in table {
        if is_canonical_facade_top_level_key(key) {
            continue;
        }

        let mut single = toml::Table::new();
        single.insert(key.clone(), value.clone());

        let Ok(rendered) = toml::to_string_pretty(&toml::Value::Table(single)) else {
            continue;
        };

        let rendered = rendered.trim();

        if rendered.is_empty() {
            continue;
        }

        out.push('\n');
        out.push_str(rendered);
        out.push('\n');
    }
}

fn is_canonical_facade_top_level_key(key: &str) -> bool {
    matches!(
        key,
        "package"
            | "features"
            | "dependencies"
            | "dev-dependencies"
            | "build-dependencies"
            | "example"
            | "bin"
            | "test"
            | "bench"
            | "lints"
    )
}

fn render_dependency_value(value: &toml::Value) -> String {
    let Some(table) = value.as_table() else {
        return render_toml_value(value);
    };

    let preferred_order = [
        "version",
        "path",
        "package",
        "workspace",
        "optional",
        "default-features",
        "features",
    ];

    let mut parts = Vec::new();

    for key in preferred_order {
        if let Some(value) = table.get(key) {
            parts.push(format!("{key} = {}", render_toml_value(value)));
        }
    }

    for (key, value) in table {
        if preferred_order.contains(&key.as_str()) {
            continue;
        }

        parts.push(format!("{key} = {}", render_toml_value(value)));
    }

    format!("{{ {} }}", parts.join(", "))
}

fn render_toml_value(value: &toml::Value) -> String {
    if let Some(value) = value.as_str() {
        return render_toml_string(value);
    }

    if let Some(values) = value.as_array() {
        let strings = values
            .iter()
            .filter_map(toml::Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        if strings.len() == values.len() {
            return render_inline_array(&strings);
        }
    }

    value.to_string()
}

fn render_inline_array(values: &[String]) -> String {
    let rendered = values
        .iter()
        .map(|value| render_toml_string(value))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{rendered}]")
}

fn render_toml_string(value: &str) -> String {
    format!("{value:?}")
}

fn string_field<'a>(table: Option<&'a toml::Table>, key: &str) -> Option<&'a str> {
    table
        .and_then(|table| table.get(key))
        .and_then(toml::Value::as_str)
}

fn bool_field(table: Option<&toml::Table>, key: &str) -> Option<bool> {
    table
        .and_then(|table| table.get(key))
        .and_then(toml::Value::as_bool)
}

fn array_string_field(table: Option<&toml::Table>, key: &str) -> Option<Vec<String>> {
    table
        .and_then(|table| table.get(key))
        .and_then(toml::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(toml::Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .filter(|values| !values.is_empty())
}

fn write_if_changed(
    root: &Path,
    manifest_path: &Path,
    created: bool,
    original: &str,
    manifest: &toml::Value,
    write: bool,
    summary: &mut ManifestFixSummary,
) -> Result<()> {
    let rendered = toml::to_string_pretty(manifest)
        .with_context(|| format!("failed to render `{}`", manifest_path.display()))?;

    write_rendered_if_changed(
        root,
        manifest_path,
        created,
        original,
        rendered,
        write,
        summary,
    )
}

fn write_rendered_if_changed(
    root: &Path,
    manifest_path: &Path,
    created: bool,
    original: &str,
    rendered: String,
    write: bool,
    summary: &mut ManifestFixSummary,
) -> Result<()> {
    let rendered = if rendered.ends_with('\n') {
        rendered
    } else {
        format!("{rendered}\n")
    };

    if rendered == original {
        summary.files_unchanged += 1;
        return Ok(());
    }

    summary.files_changed += 1;

    if created {
        summary.files_created += 1;
    }

    if write {
        if let Some(parent) = manifest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create `{}`", parent.display()))?;
        }

        fs::write(manifest_path, rendered)
            .with_context(|| format!("failed to write `{}`", manifest_path.display()))?;
    }

    summary.changes.push(ManifestFileChange {
        path: relative_path(root, manifest_path),
        created,
        wrote: write,
    });

    Ok(())
}

#[derive(Debug, Clone)]
struct CrateInfo {
    dir_name: String,
    package_name: String,
    version: Option<String>,
    manifest_path: PathBuf,
}

impl CrateInfo {
    fn dependency_name(&self) -> &str {
        &self.package_name
    }

    fn feature_name(&self) -> String {
        self.dependency_name()
            .strip_prefix("use-")
            .unwrap_or(self.dependency_name())
            .to_string()
    }
}

fn discover_crate_infos(crates_dir: &Path) -> Result<Vec<CrateInfo>> {
    if !crates_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut crates = Vec::new();

    for entry in fs::read_dir(crates_dir)
        .with_context(|| format!("failed to read `{}`", crates_dir.display()))?
    {
        let entry = entry?;
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
            .context("crate directory name is not valid UTF-8")?
            .to_string();

        let raw = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read `{}`", manifest_path.display()))?;

        let manifest = toml::from_str::<toml::Value>(&raw)
            .with_context(|| format!("failed to parse `{}`", manifest_path.display()))?;

        let package = manifest.get("package").and_then(toml::Value::as_table);

        let package_name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or(&dir_name)
            .to_string();

        let version = package
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
            .map(ToOwned::to_owned);

        crates.push(CrateInfo {
            dir_name,
            package_name,
            version,
            manifest_path,
        });
    }

    crates.sort_by(|left, right| left.package_name.cmp(&right.package_name));

    Ok(crates)
}

fn find_facade_crate<'a>(crates: &'a [CrateInfo], facade_name: &str) -> Option<&'a CrateInfo> {
    crates
        .iter()
        .find(|crate_info| crate_info.package_name == facade_name)
        .or_else(|| {
            crates
                .iter()
                .find(|crate_info| crate_info.dir_name == facade_name)
        })
}

fn child_crates(crates: &[CrateInfo], facade_name: &str) -> Vec<CrateInfo> {
    crates
        .iter()
        .filter(|crate_info| {
            crate_info.dir_name != facade_name && crate_info.package_name != facade_name
        })
        .cloned()
        .collect()
}

fn ensure_table<'a>(table: &'a mut toml::Table, key: &str) -> &'a mut toml::Table {
    let needs_insert = table.get(key).and_then(toml::Value::as_table).is_none();

    if needs_insert {
        table.insert(key.to_string(), toml::Value::Table(toml::Table::new()));
    }

    table
        .get_mut(key)
        .and_then(toml::Value::as_table_mut)
        .expect("table was just inserted")
}

fn ensure_dotted_table<'a>(table: &'a mut toml::Table, path: &[&str]) -> &'a mut toml::Table {
    let mut current = table;

    for key in path {
        current = ensure_table(current, key);
    }

    current
}

fn set_string_if_missing(table: &mut toml::Table, key: &str, value: &str) {
    if !table.contains_key(key) {
        table.insert(key.to_string(), toml::Value::String(value.to_string()));
    }
}

fn set_string_array_if_missing(table: &mut toml::Table, key: &str, values: &[&str]) {
    if !table.contains_key(key) {
        table.insert(
            key.to_string(),
            toml::Value::Array(
                values
                    .iter()
                    .map(|value| toml::Value::String((*value).to_string()))
                    .collect(),
            ),
        );
    }
}

fn set_workspace_true(table: &mut toml::Table, key: &str) {
    let mut workspace = toml::Table::new();
    workspace.insert("workspace".to_string(), toml::Value::Boolean(true));

    table.insert(key.to_string(), toml::Value::Table(workspace));
}

fn matches_facade_filter(facade: &FacadeEntry, facade_filter: Option<&str>) -> bool {
    match facade_filter {
        Some(filter) => facade.name == filter,
        None => true,
    }
}

fn relative_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}

fn render_workspace_manifest(
    manifest: &toml::Value,
    facade_name: &str,
    child_crates: &[CrateInfo],
) -> String {
    let workspace = manifest.get("workspace").and_then(toml::Value::as_table);

    let package = workspace
        .and_then(|workspace| workspace.get("package"))
        .and_then(toml::Value::as_table);

    let categories = array_string_field(package, "categories").unwrap_or_else(|| {
        DEFAULT_WORKSPACE_CATEGORIES
            .iter()
            .map(|value| (*value).to_string())
            .collect()
    });

    let mut out = String::new();

    out.push_str("[workspace]\n");
    out.push_str("members = [\"crates/*\"]\n");
    out.push_str("resolver = \"3\"\n\n");

    out.push_str("[workspace.package]\n");
    out.push_str("authors = [\"RustUse Contributors\"]\n");
    out.push_str(&format!(
        "categories = {}\n",
        render_inline_array(&categories)
    ));
    out.push_str("edition = \"2024\"\n");
    out.push_str("license = \"MIT OR Apache-2.0\"\n");
    out.push_str(&format!(
        "repository = \"{RUSTUSE_GITHUB_ORG}/{facade_name}\"\n"
    ));
    out.push_str("rust-version = \"1.95.0\"\n\n");

    out.push_str("[workspace.dependencies]\n");

    let dependencies = workspace
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table);

    append_workspace_dependency_entries(dependencies, child_crates, &mut out);

    out.push('\n');
    out.push_str("[workspace.lints.rust]\n");
    out.push_str("unsafe_code = \"forbid\"\n\n");

    out.push_str("[workspace.lints.clippy]\n");
    append_workspace_clippy_lints(workspace, &mut out);

    if !out.ends_with('\n') {
        out.push('\n');
    }

    out
}

fn append_workspace_dependency_entries(
    existing_dependencies: Option<&toml::Table>,
    child_crates: &[CrateInfo],
    out: &mut String,
) {
    let mut entries = Vec::new();

    if let Some(existing_dependencies) = existing_dependencies {
        for (name, value) in existing_dependencies {
            if child_crates
                .iter()
                .any(|child| child.dependency_name() == name)
            {
                continue;
            }

            entries.push((name.clone(), render_dependency_value(value)));
        }
    }

    for child in child_crates {
        let path = format!("crates/{}", child.dir_name);

        let value = match &child.version {
            Some(version) => format!(
                "{{ version = {}, path = {} }}",
                render_toml_string(version),
                render_toml_string(&path)
            ),
            None => format!("{{ path = {} }}", render_toml_string(&path)),
        };

        entries.push((child.dependency_name().to_string(), value));
    }

    entries.sort_by(|left, right| dependency_sort_key(&left.0).cmp(&dependency_sort_key(&right.0)));

    for (name, value) in entries {
        out.push_str(&format!("{name} = {value}\n"));
    }
}

fn dependency_sort_key(name: &str) -> (u8, String) {
    if name.starts_with("use-") {
        (1, name.to_string())
    } else {
        (0, name.to_string())
    }
}

fn append_workspace_clippy_lints(workspace: Option<&toml::Table>, out: &mut String) {
    let existing = workspace
        .and_then(|workspace| workspace.get("lints"))
        .and_then(|lints| lints.get("clippy"))
        .and_then(toml::Value::as_table);

    let canonical_keys = [
        "all",
        "cargo",
        "derivable_impls",
        "doc_markdown",
        "expect_used",
        "missing_const_for_fn",
        "missing_errors_doc",
        "module_name_repetitions",
        "multiple_crate_versions",
        "must_use_candidate",
        "nursery",
        "pedantic",
        "return_self_not_must_use",
        "todo",
        "unimplemented",
        "unwrap_used",
    ];

    let defaults = [
        ("all", "{ level = \"warn\", priority = -1 }"),
        ("cargo", "{ level = \"warn\", priority = -1 }"),
        ("derivable_impls", "\"allow\""),
        ("doc_markdown", "\"allow\""),
        ("expect_used", "\"warn\""),
        ("missing_const_for_fn", "\"allow\""),
        ("missing_errors_doc", "\"allow\""),
        ("module_name_repetitions", "\"allow\""),
        ("multiple_crate_versions", "\"allow\""),
        ("must_use_candidate", "\"allow\""),
        ("nursery", "{ level = \"warn\", priority = -1 }"),
        ("pedantic", "{ level = \"warn\", priority = -1 }"),
        ("return_self_not_must_use", "\"allow\""),
        ("todo", "\"deny\""),
        ("unimplemented", "\"deny\""),
        ("unwrap_used", "\"warn\""),
    ];

    for (key, default_value) in defaults {
        let value = existing
            .and_then(|existing| existing.get(key))
            .map(render_workspace_lint_value)
            .unwrap_or_else(|| default_value.to_string());

        out.push_str(&format!("{key} = {value}\n"));
    }

    if let Some(existing) = existing {
        let mut extras = existing
            .iter()
            .filter(|(key, _)| !canonical_keys.contains(&key.as_str()))
            .map(|(key, value)| (key.clone(), render_workspace_lint_value(value)))
            .collect::<Vec<_>>();

        extras.sort_by(|left, right| left.0.cmp(&right.0));

        for (key, value) in extras {
            out.push_str(&format!("{key} = {value}\n"));
        }
    }
}

fn render_workspace_lint_value(value: &toml::Value) -> String {
    if let Some(table) = value.as_table() {
        let level = table
            .get("level")
            .and_then(toml::Value::as_str)
            .unwrap_or("warn");

        let priority = table
            .get("priority")
            .and_then(toml::Value::as_integer)
            .unwrap_or(-1);

        return format!(
            "{{ level = {}, priority = {priority} }}",
            render_toml_string(level)
        );
    }

    render_toml_value(value)
}
