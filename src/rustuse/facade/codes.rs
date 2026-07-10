pub(crate) const SHAPE_WORKSPACE: &str = "Workspace shape";
pub(crate) const SHAPE_FACADE_WIRING: &str = "Facade wiring";
pub(crate) const SHAPE_PACKAGE: &str = "Package shape";
pub(crate) const SHAPE_CATEGORY_METADATA: &str = "Category metadata";
pub(crate) const SHAPE_GENERAL_METADATA: &str = "General metadata";

pub(crate) const MISSING_STANDARD_WORKSPACE_MEMBER: &str = "missing-standard-workspace-member";
pub(crate) const NON_STANDARD_WORKSPACE_MEMBERS: &str = "non-standard-workspace-members";
pub(crate) const MISSING_WORKSPACE: &str = "missing-workspace";
pub(crate) const MISSING_WORKSPACE_MEMBERS: &str = "missing-workspace-members";
pub(crate) const INVALID_WORKSPACE_MEMBERS: &str = "invalid-workspace-members";
pub(crate) const MISSING_WORKSPACE_RESOLVER: &str = "missing-workspace-resolver";
pub(crate) const INVALID_WORKSPACE_RESOLVER: &str = "workspace-resolver";
pub(crate) const MISSING_WORKSPACE_PACKAGE: &str = "missing-workspace-package";
pub(crate) const MISSING_WORKSPACE_PACKAGE_FIELD: &str = "missing-workspace-package-field";
pub(crate) const INVALID_WORKSPACE_REPOSITORY: &str = "invalid-workspace-repository";
pub(crate) const MISSING_WORKSPACE_DEPENDENCIES: &str = "missing-workspace-dependencies";
pub(crate) const MISSING_WORKSPACE_DEPENDENCY: &str = "missing-workspace-dependency";
pub(crate) const INVALID_WORKSPACE_DEPENDENCY: &str = "invalid-workspace-dependency";
pub(crate) const INVALID_WORKSPACE_DEPENDENCY_SHAPE: &str = "invalid-workspace-dependency-shape";
pub(crate) const INVALID_WORKSPACE_DEPENDENCY_PATH: &str = "invalid-workspace-dependency-path";
pub(crate) const MISSING_WORKSPACE_DEPENDENCY_PATH: &str = "missing-workspace-dependency-path";
pub(crate) const MISSING_WORKSPACE_DEPENDENCY_VERSION: &str =
    "missing-workspace-dependency-version";
pub(crate) const INVALID_WORKSPACE_DEPENDENCY_VERSION: &str =
    "invalid-workspace-dependency-version";
pub(crate) const MISMATCHED_WORKSPACE_DEPENDENCY_VERSION: &str =
    "mismatched-workspace-dependency-version";
pub(crate) const ORPHAN_WORKSPACE_DEPENDENCY_PATH: &str = "orphan-workspace-dependency-path";
pub(crate) const MISSING_WORKSPACE_UNSAFE_CODE_POLICY: &str =
    "missing-workspace-unsafe-code-policy";
pub(crate) const INVALID_WORKSPACE_UNSAFE_CODE_POLICY: &str =
    "invalid-workspace-unsafe-code-policy";
pub(crate) const MISSING_WORKSPACE_CLIPPY_LINTS: &str = "missing-workspace-clippy-lints";

pub(crate) const MISSING_FACADE_DEPENDENCIES: &str = "missing-facade-dependencies";
pub(crate) const MISSING_FACADE_CHILD_DEPENDENCY: &str = "missing-facade-child-dependency";
pub(crate) const INVALID_FACADE_CHILD_DEPENDENCY: &str = "invalid-facade-child-dependency";
pub(crate) const MISSING_FACADE_CHILD_DEPENDENCY_OPTIONAL: &str =
    "missing-facade-child-dependency-optional";
pub(crate) const MISSING_FACADE_FEATURES: &str = "missing-facade-features";
pub(crate) const INVALID_FACADE_DEFAULT_FEATURES: &str = "invalid-facade-default-features";
pub(crate) const MISSING_FACADE_DEFAULT_FEATURES: &str = "missing-facade-default-features";
pub(crate) const MISSING_FACADE_FULL_FEATURE: &str = "missing-facade-full-feature";
pub(crate) const MISSING_FULL_FEATURE_MEMBER: &str = "missing-full-feature-member";
pub(crate) const MISSING_FACADE_CHILD_FEATURE: &str = "missing-facade-child-feature";
pub(crate) const INVALID_FACADE_CHILD_FEATURE: &str = "invalid-facade-child-feature";

pub(crate) const INVALID_PACKAGE_HOMEPAGE: &str = "invalid-package-homepage";
pub(crate) const INVALID_PACKAGE_DOCUMENTATION: &str = "invalid-package-documentation";
pub(crate) const MISSING_PACKAGE_README_FILE: &str = "missing-package-readme-file";
pub(crate) const MISSING_DOCS_RS_ALL_FEATURES: &str = "missing-docs-rs-all-features";
pub(crate) const INVALID_DOCS_RS_ALL_FEATURES: &str = "invalid-docs-rs-all-features";
pub(crate) const MISSING_LINTS_WORKSPACE: &str = "missing-lints-workspace";
pub(crate) const PACKAGE_NAME_DIRECTORY_MISMATCH: &str = "package-name-directory-mismatch";
pub(crate) const INVALID_FACADE_PACKAGE_NAME: &str = "invalid-facade-package-name";
pub(crate) const INVALID_CHILD_PACKAGE_NAME: &str = "invalid-child-package-name";

pub(crate) const INVALID_CATEGORY_SLUG: &str = "invalid-category-slug";
pub(crate) const TOO_MANY_CATEGORIES: &str = "too-many-categories";
pub(crate) const DUPLICATE_CATEGORY: &str = "duplicate-category";
pub(crate) const INVALID_CATEGORIES_SHAPE: &str = "invalid-categories-shape";
pub(crate) const INVALID_CATEGORY_VALUE: &str = "invalid-category-value";
pub(crate) const MISSING_WORKSPACE_CATEGORIES: &str = "missing-workspace-categories";
pub(crate) const MISSING_PACKAGE_CATEGORIES: &str = "missing-package-categories";
pub(crate) const MISSING_INHERITED_CATEGORIES: &str = "missing-inherited-categories";

pub(crate) const WORKSPACE_SHAPE_CODES: &[&str] = &[
    MISSING_STANDARD_WORKSPACE_MEMBER,
    NON_STANDARD_WORKSPACE_MEMBERS,
    MISSING_WORKSPACE,
    MISSING_WORKSPACE_MEMBERS,
    INVALID_WORKSPACE_MEMBERS,
    MISSING_WORKSPACE_RESOLVER,
    INVALID_WORKSPACE_RESOLVER,
    MISSING_WORKSPACE_PACKAGE,
    MISSING_WORKSPACE_PACKAGE_FIELD,
    INVALID_WORKSPACE_REPOSITORY,
    MISSING_WORKSPACE_DEPENDENCIES,
    MISSING_WORKSPACE_DEPENDENCY,
    INVALID_WORKSPACE_DEPENDENCY,
    INVALID_WORKSPACE_DEPENDENCY_SHAPE,
    INVALID_WORKSPACE_DEPENDENCY_PATH,
    MISSING_WORKSPACE_DEPENDENCY_PATH,
    MISSING_WORKSPACE_DEPENDENCY_VERSION,
    INVALID_WORKSPACE_DEPENDENCY_VERSION,
    MISMATCHED_WORKSPACE_DEPENDENCY_VERSION,
    ORPHAN_WORKSPACE_DEPENDENCY_PATH,
    MISSING_WORKSPACE_UNSAFE_CODE_POLICY,
    INVALID_WORKSPACE_UNSAFE_CODE_POLICY,
    MISSING_WORKSPACE_CLIPPY_LINTS,
];

pub(crate) const FACADE_WIRING_CODES: &[&str] = &[
    MISSING_FACADE_DEPENDENCIES,
    MISSING_FACADE_CHILD_DEPENDENCY,
    INVALID_FACADE_CHILD_DEPENDENCY,
    MISSING_FACADE_CHILD_DEPENDENCY_OPTIONAL,
    MISSING_FACADE_FEATURES,
    INVALID_FACADE_DEFAULT_FEATURES,
    MISSING_FACADE_DEFAULT_FEATURES,
    MISSING_FACADE_FULL_FEATURE,
    MISSING_FULL_FEATURE_MEMBER,
    MISSING_FACADE_CHILD_FEATURE,
    INVALID_FACADE_CHILD_FEATURE,
];

pub(crate) const PACKAGE_SHAPE_CODES: &[&str] = &[
    INVALID_PACKAGE_HOMEPAGE,
    INVALID_PACKAGE_DOCUMENTATION,
    MISSING_PACKAGE_README_FILE,
    MISSING_DOCS_RS_ALL_FEATURES,
    INVALID_DOCS_RS_ALL_FEATURES,
    MISSING_LINTS_WORKSPACE,
    PACKAGE_NAME_DIRECTORY_MISMATCH,
    INVALID_FACADE_PACKAGE_NAME,
    INVALID_CHILD_PACKAGE_NAME,
];

pub(crate) const CATEGORY_METADATA_CODES: &[&str] = &[
    INVALID_CATEGORY_SLUG,
    TOO_MANY_CATEGORIES,
    DUPLICATE_CATEGORY,
    INVALID_CATEGORIES_SHAPE,
    INVALID_CATEGORY_VALUE,
    MISSING_WORKSPACE_CATEGORIES,
    MISSING_PACKAGE_CATEGORIES,
    MISSING_INHERITED_CATEGORIES,
];

/* pub(crate) const WORKSPACE_DEPENDENCY_VERSION_CODES: &[&str] = &[
    MISSING_WORKSPACE_DEPENDENCY_VERSION,
    INVALID_WORKSPACE_DEPENDENCY_VERSION,
    MISMATCHED_WORKSPACE_DEPENDENCY_VERSION,
]; */

pub(crate) fn manifest_shape_bucket(code: &str) -> &'static str {
    if WORKSPACE_SHAPE_CODES.contains(&code) {
        SHAPE_WORKSPACE
    } else if FACADE_WIRING_CODES.contains(&code) {
        SHAPE_FACADE_WIRING
    } else if PACKAGE_SHAPE_CODES.contains(&code) {
        SHAPE_PACKAGE
    } else if CATEGORY_METADATA_CODES.contains(&code) {
        SHAPE_CATEGORY_METADATA
    } else {
        SHAPE_GENERAL_METADATA
    }
}

/* pub(crate) fn is_workspace_dependency_version_code(code: &str) -> bool {
    WORKSPACE_DEPENDENCY_VERSION_CODES.contains(&code)
} */

pub(crate) const MISSING_GIT_REPOSITORY: &str = "missing-git-repository";
pub(crate) const MISSING_ROOT_MANIFEST: &str = "missing-root-manifest";
pub(crate) const MISSING_CRATES_DIRECTORY: &str = "missing-crates-directory";
pub(crate) const MISSING_CHILD_CRATES: &str = "missing-child-crates";
pub(crate) const MISSING_REQUIRED_FILE: &str = "missing-required-file";
pub(crate) const MISSING_REQUIRED_DIRECTORY: &str = "missing-required-directory";
pub(crate) const MISSING_CRATE_LIB: &str = "missing-crate-lib";
pub(crate) const MISSING_FACADE_PRELUDE: &str = "missing-facade-prelude";
pub(crate) const MISSING_TOOLING_SURFACE: &str = "missing-tooling-surface";
pub(crate) const MISSING_GITHUB_CI_CD_SURFACE: &str = "missing-github-ci-cd-surface";
pub(crate) const MISSING_RELEASE_SURFACE: &str = "missing-release-surface";
pub(crate) const MISSING_RELEASE_CI_SURFACE: &str = "missing-release-ci-surface";
pub(crate) const NON_STANDARD_PATH: &str = "non-standard-path";
