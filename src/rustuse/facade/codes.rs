use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum FacadeIssueBucket {
    WorkspaceShape,
    FacadeWiring,
    PackageShape,
    CategoryMetadata,
    RepositoryShape,
    ToolingSurface,
    ReleaseSurface,
}

impl FacadeIssueBucket {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceShape => "Workspace shape",
            Self::FacadeWiring => "Facade wiring",
            Self::PackageShape => "Package shape",
            Self::CategoryMetadata => "Category metadata",
            Self::RepositoryShape => "Repository shape",
            Self::ToolingSurface => "Tooling surface",
            Self::ReleaseSurface => "Release surface",
        }
    }
}

impl fmt::Display for FacadeIssueBucket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! define_facade_issue_codes {
    (
        $(
            $variant:ident => {
                id: $id:literal,
                bucket: $bucket:ident,
            };
        )+
    ) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub(crate) enum FacadeIssueCode {
            $(
                $variant,
            )+
        }

        impl FacadeIssueCode {
            pub(crate) const fn as_str(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $id,
                    )+
                }
            }

            pub(crate) fn from_id(id: &str) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            pub(crate) const fn bucket(self) -> FacadeIssueBucket {
                match self {
                    $(
                        Self::$variant => FacadeIssueBucket::$bucket,
                    )+
                }
            }
        }

        impl fmt::Display for FacadeIssueCode {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        #[cfg(test)]
        const ALL_FACADE_ISSUE_CODES: &[FacadeIssueCode] = &[
            $(
                FacadeIssueCode::$variant,
            )+
        ];
    };
}

define_facade_issue_codes! {
    MissingStandardWorkspaceMember => {
        id: "missing-standard-workspace-member",
        bucket: WorkspaceShape,
    };
    NonStandardWorkspaceMembers => {
        id: "non-standard-workspace-members",
        bucket: WorkspaceShape,
    };
    MissingWorkspace => {
        id: "missing-workspace",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceMembers => {
        id: "missing-workspace-members",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceMembers => {
        id: "invalid-workspace-members",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceResolver => {
        id: "missing-workspace-resolver",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceResolver => {
        id: "invalid-workspace-resolver",
        bucket: WorkspaceShape,
    };
    MissingWorkspacePackage => {
        id: "missing-workspace-package",
        bucket: WorkspaceShape,
    };
    MissingWorkspacePackageField => {
        id: "missing-workspace-package-field",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceRepository => {
        id: "invalid-workspace-repository",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceDependencies => {
        id: "missing-workspace-dependencies",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceDependency => {
        id: "missing-workspace-dependency",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceDependencyShape => {
        id: "invalid-workspace-dependency-shape",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceDependencyPath => {
        id: "invalid-workspace-dependency-path",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceDependencyPath => {
        id: "missing-workspace-dependency-path",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceDependencyVersion => {
        id: "missing-workspace-dependency-version",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceDependencyVersion => {
        id: "invalid-workspace-dependency-version",
        bucket: WorkspaceShape,
    };
    MismatchedWorkspaceDependencyVersion => {
        id: "mismatched-workspace-dependency-version",
        bucket: WorkspaceShape,
    };
    OrphanWorkspaceDependencyPath => {
        id: "orphan-workspace-dependency-path",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceUnsafeCodePolicy => {
        id: "missing-workspace-unsafe-code-policy",
        bucket: WorkspaceShape,
    };
    InvalidWorkspaceUnsafeCodePolicy => {
        id: "invalid-workspace-unsafe-code-policy",
        bucket: WorkspaceShape,
    };
    MissingWorkspaceClippyLints => {
        id: "missing-workspace-clippy-lints",
        bucket: WorkspaceShape,
    };

    MissingFacadeDependencies => {
        id: "missing-facade-dependencies",
        bucket: FacadeWiring,
    };
    MissingFacadeChildDependency => {
        id: "missing-facade-child-dependency",
        bucket: FacadeWiring,
    };
    InvalidFacadeChildDependency => {
        id: "invalid-facade-child-dependency",
        bucket: FacadeWiring,
    };
    MissingFacadeChildDependencyOptional => {
        id: "missing-facade-child-dependency-optional",
        bucket: FacadeWiring,
    };
    MissingFacadeFeatures => {
        id: "missing-facade-features",
        bucket: FacadeWiring,
    };
    InvalidFacadeDefaultFeatures => {
        id: "invalid-facade-default-features",
        bucket: FacadeWiring,
    };
    MissingFacadeDefaultFeatures => {
        id: "missing-facade-default-features",
        bucket: FacadeWiring,
    };
    MissingFacadeFullFeature => {
        id: "missing-facade-full-feature",
        bucket: FacadeWiring,
    };
    MissingFullFeatureMember => {
        id: "missing-full-feature-member",
        bucket: FacadeWiring,
    };
    MissingFacadeChildFeature => {
        id: "missing-facade-child-feature",
        bucket: FacadeWiring,
    };
    InvalidFacadeChildFeature => {
        id: "invalid-facade-child-feature",
        bucket: FacadeWiring,
    };

    InvalidPackageHomepage => {
        id: "invalid-package-homepage",
        bucket: PackageShape,
    };
    InvalidPackageDocumentation => {
        id: "invalid-package-documentation",
        bucket: PackageShape,
    };
    MissingPackageReadmeFile => {
        id: "missing-package-readme-file",
        bucket: PackageShape,
    };
    MissingDocsRsAllFeatures => {
        id: "missing-docs-rs-all-features",
        bucket: PackageShape,
    };
    InvalidDocsRsAllFeatures => {
        id: "invalid-docs-rs-all-features",
        bucket: PackageShape,
    };
    MissingLintsWorkspace => {
        id: "missing-lints-workspace",
        bucket: PackageShape,
    };
    PackageNameDirectoryMismatch => {
        id: "package-name-directory-mismatch",
        bucket: PackageShape,
    };
    InvalidFacadePackageName => {
        id: "invalid-facade-package-name",
        bucket: PackageShape,
    };
    InvalidChildPackageName => {
        id: "invalid-child-package-name",
        bucket: PackageShape,
    };
    MissingCrateLib => {
        id: "missing-crate-lib",
        bucket: PackageShape,
    };
    MissingFacadePrelude => {
        id: "missing-facade-prelude",
        bucket: PackageShape,
    };

    InvalidCategorySlug => {
        id: "invalid-category-slug",
        bucket: CategoryMetadata,
    };
    TooManyCategories => {
        id: "too-many-categories",
        bucket: CategoryMetadata,
    };
    DuplicateCategory => {
        id: "duplicate-category",
        bucket: CategoryMetadata,
    };
    InvalidCategoriesShape => {
        id: "invalid-categories-shape",
        bucket: CategoryMetadata,
    };
    InvalidCategoryValue => {
        id: "invalid-category-value",
        bucket: CategoryMetadata,
    };
    MissingWorkspaceCategories => {
        id: "missing-workspace-categories",
        bucket: CategoryMetadata,
    };
    MissingPackageCategories => {
        id: "missing-package-categories",
        bucket: CategoryMetadata,
    };
    MissingInheritedCategories => {
        id: "missing-inherited-categories",
        bucket: CategoryMetadata,
    };

    NestedFacadePackage => {
        id: "nested-facade-package",
        bucket: RepositoryShape,
    };
    MissingGitRepository => {
        id: "missing-git-repository",
        bucket: RepositoryShape,
    };
    MissingRootManifest => {
        id: "missing-root-manifest",
        bucket: RepositoryShape,
    };
    MissingCratesDirectory => {
        id: "missing-crates-directory",
        bucket: RepositoryShape,
    };
    MissingChildCrates => {
        id: "missing-child-crates",
        bucket: RepositoryShape,
    };
    MissingRequiredFile => {
        id: "missing-required-file",
        bucket: RepositoryShape,
    };
    MissingRequiredDirectory => {
        id: "missing-required-directory",
        bucket: RepositoryShape,
    };
    NonStandardPath => {
        id: "non-standard-path",
        bucket: RepositoryShape,
    };

    MissingToolingSurface => {
        id: "missing-tooling-surface",
        bucket: ToolingSurface,
    };
    MissingGithubCiCdSurface => {
        id: "missing-github-ci-cd-surface",
        bucket: ToolingSurface,
    };

    MissingReleaseSurface => {
        id: "missing-release-surface",
        bucket: ReleaseSurface,
    };
    MissingReleaseCiSurface => {
        id: "missing-release-ci-surface",
        bucket: ReleaseSurface,
    };

    MissingWorkspaceManifest => {
        id: "missing-workspace-manifest",
        bucket: WorkspaceShape,
    };

    ReadManifest => {
        id: "read-manifest",
        bucket: PackageShape,
    };
    ParseManifest => {
        id: "parse-manifest",
        bucket: PackageShape,
    };
    InvalidManifest => {
        id: "invalid-manifest",
        bucket: PackageShape,
    };

    MissingPackage => {
        id: "missing-package",
        bucket: PackageShape,
    };
    MissingPackageField => {
        id: "missing-package-field",
        bucket: PackageShape,
    };
    InvalidPackageName => {
        id: "invalid-package-name",
        bucket: PackageShape,
    };
    InvalidPackageVersion => {
        id: "invalid-package-version",
        bucket: PackageShape,
    };
    InvalidPackagePublish => {
        id: "invalid-package-publish",
        bucket: PackageShape,
    };
    MissingPackagePublish => {
        id: "missing-package-publish",
        bucket: PackageShape,
    };
    MissingPackageInheritedField => {
        id: "missing-package-inherited-field",
        bucket: PackageShape,
    };
    PackageFieldNotInherited => {
        id: "package-field-not-inherited",
        bucket: PackageShape,
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParseFacadeIssueCodeError {
    value: String,
}

impl fmt::Display for ParseFacadeIssueCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown facade issue code: {}", self.value)
    }
}

impl std::error::Error for ParseFacadeIssueCodeError {}

impl FromStr for FacadeIssueCode {
    type Err = ParseFacadeIssueCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_id(value).ok_or_else(|| ParseFacadeIssueCodeError {
            value: value.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::ALL_FACADE_ISSUE_CODES;

    #[test]
    fn facade_issue_code_ids_are_unique() {
        let mut ids = BTreeSet::new();

        for code in ALL_FACADE_ISSUE_CODES {
            assert!(
                ids.insert(code.as_str()),
                "duplicate facade issue code: {code}"
            );
        }
    }

    #[test]
    fn facade_issue_code_ids_use_kebab_case() {
        for code in ALL_FACADE_ISSUE_CODES {
            let id = code.as_str();

            assert!(!id.is_empty(), "facade issue code cannot be empty");

            assert!(
                !id.starts_with('-') && !id.ends_with('-'),
                "facade issue code has an outer hyphen: {id}"
            );

            assert!(
                !id.contains("--"),
                "facade issue code has consecutive hyphens: {id}"
            );

            assert!(
                id.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
                }),
                "facade issue code is not lowercase kebab-case: {id}"
            );
        }
    }
}
