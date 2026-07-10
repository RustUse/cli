use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::rustuse::adapter::github::workflows::{GitHubWorkflowReport, inspect_github_workflows};
use crate::rustuse::adapter::gitlab::{GitLabReport, inspect_gitlab};
use crate::rustuse::facade::codes;
use crate::rustuse::facade::discover::{FacadeInfo, discover_facade};
use crate::rustuse::facade::inspect::{FacadeRepositoryReport, inspect_facade_repository};
use crate::rustuse::facade::issue::{FacadeFixKind, FacadeIssue, FacadeIssueSeverity};
use crate::rustuse::facade::layout::MAKEFILE;
use crate::rustuse::facade::manifest::{FacadeManifestReport, analyze_facade_repository_manifests};
use crate::rustuse::facade::non_standard::{
    NonStandardPathKind, NonStandardPathReport, NonStandardPathRule, inspect_non_standard_paths,
};
use crate::rustuse::facade::release::{ReleaseSurfaceReport, inspect_release_surface};
use crate::rustuse::utils::tooling::{ToolingSurfaceReport, inspect_tooling_surface};

const BUCKET_FACADE_SHAPE: &str = "Facade shape";
const BUCKET_REPOSITORY_SURFACE: &str = "Repository surface";
const BUCKET_DOCUMENTATION: &str = "Documentation";
const BUCKET_CI_CD: &str = "CI/CD surface";
const BUCKET_RELEASE: &str = "Release surface";
const BUCKET_TOOLING: &str = "Tooling configuration";
const BUCKET_NON_STANDARD_PATHS: &str = "Non-standard paths";

const FACADE_NON_STANDARD_PATHS: &[NonStandardPathRule] = &[
    NonStandardPathRule {
        path: "docs/",
        kind: NonStandardPathKind::Directory,
        recommendation: "Move facade documentation to the central docs repository.",
    },
    NonStandardPathRule {
        path: MAKEFILE,
        kind: NonStandardPathKind::File,
        recommendation: "Remove the Makefile; use Cargo commands or repository scripts instead.",
    },
];

#[derive(Debug)]
pub(crate) struct FacadeDiagnostics {
    pub(crate) facade: FacadeInfo,
    pub(crate) manifest: FacadeManifestReport,
    pub(crate) repository: FacadeRepositoryReport,
    pub(crate) github_workflows: GitHubWorkflowReport,
    pub(crate) gitlab: GitLabReport,
    pub(crate) release: ReleaseSurfaceReport,
    pub(crate) tooling: ToolingSurfaceReport,
    pub(crate) non_standard_paths: NonStandardPathReport,
    pub(crate) issues: Vec<FacadeIssue>,
}

#[derive(Clone, Debug)]
pub(crate) struct FacadeChildCrateRow {
    pub(crate) kind: &'static str,
    pub(crate) crate_name: String,
    pub(crate) manifest_path: PathBuf,
    pub(crate) readme_present: bool,
    pub(crate) lib_present: bool,
    pub(crate) prelude_present: bool,
    pub(crate) manifest_status: &'static str,
    pub(crate) manifest_issue_count: usize,
}

impl FacadeDiagnostics {
    pub(crate) fn inspect(root: &Path) -> Result<Self> {
        let facade = discover_facade(root)?;
        let manifest = analyze_facade_repository_manifests(&facade.root, &facade.name)?;
        let repository = inspect_facade_repository(&facade);
        let github_workflows = inspect_github_workflows(&facade.root);
        let gitlab = inspect_gitlab(&facade.root);
        let release = inspect_release_surface(&facade.root);
        let tooling = inspect_tooling_surface(&facade.root);
        let non_standard_paths =
            inspect_non_standard_paths(&facade.root, FACADE_NON_STANDARD_PATHS);

        let mut diagnostics = Self {
            facade,
            manifest,
            repository,
            github_workflows,
            gitlab,
            release,
            tooling,
            non_standard_paths,
            issues: Vec::new(),
        };

        diagnostics.collect_issues();

        Ok(diagnostics)
    }

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
            .filter(|issue| issue.severity == FacadeIssueSeverity::Error)
            .count()
    }

    pub(crate) fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == FacadeIssueSeverity::Warning)
            .count()
    }

    pub(crate) fn fixable_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.fix.is_some())
            .count()
    }

    pub(crate) fn fixable_issues(&self) -> impl Iterator<Item = &FacadeIssue> {
        self.issues.iter().filter(|issue| issue.fix.is_some())
    }

    pub(crate) fn child_crate_rows(&self) -> Vec<FacadeChildCrateRow> {
        self.facade
            .crate_manifest_paths
            .iter()
            .map(|manifest_path| {
                let crate_dir = manifest_path.parent().unwrap_or(&self.facade.root);
                let crate_name = crate_name(crate_dir);
                let kind = if crate_name == self.facade.name.as_str() {
                    "facade"
                } else {
                    "child"
                };
                let relative_manifest_path = manifest_path
                    .strip_prefix(&self.facade.root)
                    .unwrap_or(manifest_path)
                    .to_path_buf();
                let manifest_report = self
                    .manifest
                    .manifests
                    .iter()
                    .find(|manifest| manifest.path == relative_manifest_path);

                FacadeChildCrateRow {
                    kind,
                    crate_name,
                    manifest_path: relative_manifest_path,
                    readme_present: crate_dir.join("README.md").is_file(),
                    lib_present: crate_dir.join("src/lib.rs").is_file(),
                    prelude_present: crate_dir.join("src/prelude.rs").is_file(),
                    manifest_status: manifest_report
                        .map(|manifest| manifest.status())
                        .unwrap_or("unknown"),
                    manifest_issue_count: manifest_report
                        .map(|manifest| manifest.issue_count())
                        .unwrap_or(0),
                }
            })
            .collect()
    }

    fn collect_issues(&mut self) {
        self.collect_facade_shape_issues();
        self.collect_repository_surface_issues();
        self.collect_manifest_issues();
        self.collect_crate_documentation_issues();
        self.collect_tooling_issues();
        self.collect_ci_cd_issues();
        self.collect_release_issues();
        self.collect_non_standard_path_issues();
    }

    fn collect_facade_shape_issues(&mut self) {
        if !self.facade.has_git() {
            self.push_warning(
                codes::MISSING_GIT_REPOSITORY,
                BUCKET_FACADE_SHAPE,
                Some(self.facade.root.join(".git")),
                "Initialize or restore the facade `.git` repository.",
                None,
            );
        }

        if !self.facade.has_manifest() {
            self.push_error(
                codes::MISSING_ROOT_MANIFEST,
                BUCKET_FACADE_SHAPE,
                Some(self.facade.root.join("Cargo.toml")),
                "Add the facade root `Cargo.toml`.",
                None,
            );
        }

        if !self.facade.has_crates_dir() {
            self.push_warning(
                codes::MISSING_CRATES_DIRECTORY,
                BUCKET_FACADE_SHAPE,
                Some(self.facade.root.join("crates")),
                "Add the facade `crates/` directory.",
                Some(FacadeFixKind::RestoreStandardDirectory),
            );
        }

        if self.facade.has_crates_dir() && self.facade.crate_count() == 0 {
            self.push_warning(
                codes::MISSING_CHILD_CRATES,
                BUCKET_FACADE_SHAPE,
                Some(self.facade.root.join("crates")),
                "Add child crates under `crates/`, each with its own `Cargo.toml`.",
                None,
            );
        }
    }

    fn collect_repository_surface_issues(&mut self) {
        let missing_files = self
            .repository
            .missing_required_files()
            .into_iter()
            .map(|check| {
                (
                    self.facade.root.join(check.path),
                    format!(
                        "Restore missing required standard file `{}` ({})",
                        check.path, check.label
                    ),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing_files {
            self.push_warning(
                codes::MISSING_REQUIRED_FILE,
                BUCKET_REPOSITORY_SURFACE,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreStandardFile),
            );
        }

        let missing_directories = self
            .repository
            .missing_required_directories()
            .into_iter()
            .map(|check| {
                (
                    self.facade.root.join(check.path),
                    format!(
                        "Restore missing required standard directory `{}` ({})",
                        check.path, check.label
                    ),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing_directories {
            self.push_warning(
                codes::MISSING_REQUIRED_DIRECTORY,
                BUCKET_REPOSITORY_SURFACE,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreStandardDirectory),
            );
        }
    }

    fn collect_manifest_issues(&mut self) {
        let manifest_issues = self
            .manifest
            .manifests
            .iter()
            .flat_map(|manifest| {
                manifest.issues.iter().map(|issue| {
                    let fix = match issue.code {
                        codes::MISSING_LINTS_WORKSPACE => Some(FacadeFixKind::AddWorkspaceLints),
                        codes::MISSING_FACADE_CHILD_DEPENDENCY_OPTIONAL => None,
                        codes::MISSING_FACADE_CHILD_FEATURE => None,
                        codes::MISSING_FULL_FEATURE_MEMBER => None,
                        _ => None,
                    };

                    (
                        manifest.path.clone(),
                        severity_from_manifest(issue.severity.as_str()),
                        issue.code,
                        codes::manifest_shape_bucket(issue.code),
                        issue.message.clone(),
                        fix,
                    )
                })
            })
            .collect::<Vec<_>>();

        for (path, severity, code, bucket, message, fix) in manifest_issues {
            self.push_issue(FacadeIssue {
                severity,
                code,
                bucket,
                path: Some(path),
                message,
                fix,
            });
        }
    }

    fn collect_crate_documentation_issues(&mut self) {
        let documentation_issues = self
            .facade
            .crate_manifest_paths
            .iter()
            .flat_map(|manifest_path| {
                let crate_dir = manifest_path.parent().unwrap_or(&self.facade.root);
                let crate_name = crate_name(crate_dir);
                let is_facade_package = crate_name == self.facade.name;

                let mut issues = Vec::new();

                if !crate_dir.join("README.md").is_file() {
                    issues.push((
                        codes::MISSING_PACKAGE_README_FILE,
                        crate_dir.join("README.md"),
                        format!("Add `README.md` for crate `{crate_name}`."),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                if !crate_dir.join("src/lib.rs").is_file() {
                    issues.push((
                        codes::MISSING_CRATE_LIB,
                        crate_dir.join("src/lib.rs"),
                        format!("Add `src/lib.rs` for crate `{crate_name}`."),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                if is_facade_package && !crate_dir.join("src/prelude.rs").is_file() {
                    issues.push((
                        codes::MISSING_FACADE_PRELUDE,
                        crate_dir.join("src/prelude.rs"),
                        "Add facade `src/prelude.rs`.".to_owned(),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                issues
            })
            .collect::<Vec<_>>();

        for (code, path, message, fix) in documentation_issues {
            self.push_warning(code, BUCKET_DOCUMENTATION, Some(path), message, fix);
        }
    }

    fn collect_tooling_issues(&mut self) {
        let missing = self
            .tooling
            .surface
            .iter()
            .filter(|check| !check.present)
            .map(|check| {
                (
                    self.facade.root.join(check.path.as_str()),
                    format!("Restore missing tooling surface `{}`.", check.path),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing {
            self.push_warning(
                codes::MISSING_TOOLING_SURFACE,
                BUCKET_TOOLING,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreStandardFile),
            );
        }
    }

    fn collect_ci_cd_issues(&mut self) {
        let missing_github_paths = self
            .github_workflows
            .missing_required_paths()
            .into_iter()
            .map(|path| {
                (
                    self.facade.root.join(path),
                    format!("Restore missing required GitHub CI/CD file `{path}`."),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing_github_paths {
            self.push_warning(
                codes::MISSING_GITHUB_CI_CD_SURFACE,
                BUCKET_CI_CD,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreGithubWorkflow),
            );
        }
    }

    fn collect_release_issues(&mut self) {
        let missing_release_surface = self
            .release
            .surface
            .iter()
            .filter(|check| !check.present)
            .map(|check| {
                (
                    self.facade.root.join(check.path.as_str()),
                    format!("Restore missing release surface file `{}`.", check.path),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing_release_surface {
            self.push_warning(
                codes::MISSING_RELEASE_SURFACE,
                BUCKET_RELEASE,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreStandardFile),
            );
        }

        let missing_release_ci_surface = self
            .release
            .ci_surface
            .iter()
            .filter(|check| !check.present)
            .map(|check| {
                (
                    self.facade.root.join(check.path.as_str()),
                    format!(
                        "Restore missing release CI/CD surface file `{}`.",
                        check.path
                    ),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in missing_release_ci_surface {
            self.push_warning(
                codes::MISSING_RELEASE_CI_SURFACE,
                BUCKET_RELEASE,
                Some(path),
                message,
                Some(FacadeFixKind::RestoreGithubWorkflow),
            );
        }
    }

    fn collect_non_standard_path_issues(&mut self) {
        let present_non_standard_paths = self
            .non_standard_paths
            .present_checks()
            .into_iter()
            .map(|check| {
                (
                    self.facade.root.join(check.path),
                    check.recommendation.to_owned(),
                )
            })
            .collect::<Vec<_>>();

        for (path, message) in present_non_standard_paths {
            self.push_warning(
                codes::NON_STANDARD_PATH,
                BUCKET_NON_STANDARD_PATHS,
                Some(path),
                message,
                Some(FacadeFixKind::RemoveNonStandardPath),
            );
        }
    }

    fn push_error(
        &mut self,
        code: &'static str,
        bucket: &'static str,
        path: Option<PathBuf>,
        message: impl Into<String>,
        fix: Option<FacadeFixKind>,
    ) {
        self.push_issue(FacadeIssue {
            severity: FacadeIssueSeverity::Error,
            code,
            bucket,
            path,
            message: message.into(),
            fix,
        });
    }

    fn push_warning(
        &mut self,
        code: &'static str,
        bucket: &'static str,
        path: Option<PathBuf>,
        message: impl Into<String>,
        fix: Option<FacadeFixKind>,
    ) {
        self.push_issue(FacadeIssue {
            severity: FacadeIssueSeverity::Warning,
            code,
            bucket,
            path,
            message: message.into(),
            fix,
        });
    }

    fn push_issue(&mut self, issue: FacadeIssue) {
        self.issues.push(issue);
    }
}

fn severity_from_manifest(value: &str) -> FacadeIssueSeverity {
    if value == "error" {
        FacadeIssueSeverity::Error
    } else {
        FacadeIssueSeverity::Warning
    }
}

fn crate_name(crate_dir: &Path) -> String {
    crate_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
        .to_owned()
}
