use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::rustuse::adapter::github::workflows::{GitHubWorkflowReport, inspect_github_workflows};
use crate::rustuse::adapter::gitlab::{GitLabReport, inspect_gitlab};
use crate::rustuse::facade::codes::FacadeIssueCode;
use crate::rustuse::facade::discover::{FacadeInfo, discover_facade};
use crate::rustuse::facade::inspect::{FacadeRepositoryReport, inspect_facade_repository};
use crate::rustuse::facade::issue::{FacadeFixKind, FacadeIssue, FacadeIssueSeverity};
use crate::rustuse::facade::layout::MAKEFILE;
use crate::rustuse::facade::manifest::{
    FacadeManifestReport, ManifestFileReport, ManifestIssueSeverity,
    analyze_facade_repository_manifests,
};
use crate::rustuse::facade::model::{FacadeCrateInfo, FacadeCrateKind, FacadeStatus};
use crate::rustuse::facade::non_standard::{
    NonStandardPathKind, NonStandardPathReport, NonStandardPathRule, inspect_non_standard_paths,
};
use crate::rustuse::facade::release::{ReleaseSurfaceReport, inspect_release_surface};
use crate::rustuse::utils::tooling::{ToolingSurfaceReport, inspect_tooling_surface};

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
    pub(crate) kind: FacadeCrateKind,
    pub(crate) crate_name: String,
    pub(crate) manifest_path: PathBuf,
    pub(crate) readme_present: bool,
    pub(crate) lib_present: bool,
    pub(crate) prelude_present: bool,
    pub(crate) documentation_status: FacadeStatus,
    pub(crate) documentation_notes: String,
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
        self.facade_status().as_str()
    }

    pub(crate) fn facade_status(&self) -> FacadeStatus {
        self.facade
            .facade_status()
            .combine(FacadeStatus::from_error_warning_counts(
                self.error_count(),
                self.warning_count(),
            ))
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
                let crate_info = FacadeCrateInfo::from_manifest(&self.facade, manifest_path);
                let relative_manifest_path = self
                    .facade
                    .relative_path(&crate_info.manifest_path)
                    .to_path_buf();

                let manifest_report = self
                    .manifest
                    .manifests
                    .iter()
                    .find(|manifest| manifest.path == relative_manifest_path);

                FacadeChildCrateRow {
                    kind: crate_info.kind,
                    crate_name: crate_info.name.clone(),
                    manifest_path: relative_manifest_path,
                    readme_present: crate_info.readme_present(),
                    lib_present: crate_info.lib_present(),
                    prelude_present: crate_info.prelude_present(),
                    documentation_status: crate_info.documentation_status(),
                    documentation_notes: crate_info.documentation_notes(),
                    manifest_status: manifest_report.map_or("unknown", ManifestFileReport::status),
                    manifest_issue_count: manifest_report
                        .map_or(0, ManifestFileReport::issue_count),
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
                FacadeIssueCode::MissingGitRepository,
                Some(self.facade.root.join(".git")),
                "Initialize or restore the facade `.git` repository.",
                None,
            );
        }

        if !self.facade.has_manifest() {
            self.push_error(
                FacadeIssueCode::MissingRootManifest,
                Some(self.facade.root.join("Cargo.toml")),
                "Add the facade root `Cargo.toml`.",
                None,
            );
        }

        if !self.facade.has_crates_dir() {
            self.push_warning(
                FacadeIssueCode::MissingCratesDirectory,
                Some(self.facade.root.join("crates")),
                "Add the facade `crates/` directory.",
                Some(FacadeFixKind::RestoreStandardDirectory),
            );
        }

        if self.facade.has_crates_dir() && !self.facade.has_child_crates() {
            self.push_warning(
                FacadeIssueCode::MissingChildCrates,
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
                FacadeIssueCode::MissingRequiredFile,
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
                FacadeIssueCode::MissingRequiredDirectory,
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
                    let fix = manifest_issue_fix(issue.code);

                    (
                        manifest.path.clone(),
                        facade_severity(issue.severity),
                        issue.code,
                        issue.message.clone(),
                        fix,
                    )
                })
            })
            .collect::<Vec<_>>();

        for (path, severity, code, message, fix) in manifest_issues {
            let issue = FacadeIssue::new(severity, code, Some(path), message);

            self.push_issue_with_fix(issue, fix);
        }
    }

    fn collect_crate_documentation_issues(&mut self) {
        let documentation_issues = self
            .facade
            .crate_manifest_paths
            .iter()
            .flat_map(|manifest_path| {
                let crate_info = FacadeCrateInfo::from_manifest(&self.facade, manifest_path);

                let mut issues = Vec::new();

                if !crate_info.readme_present() {
                    issues.push((
                        FacadeIssueCode::MissingPackageReadmeFile,
                        crate_info.readme_path.clone(),
                        format!("Add `README.md` for crate `{}`.", crate_info.name),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                if !crate_info.lib_present() {
                    issues.push((
                        FacadeIssueCode::MissingCrateLib,
                        crate_info.lib_path.clone(),
                        format!("Add `src/lib.rs` for crate `{}`.", crate_info.name),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                if crate_info.requires_prelude() && !crate_info.prelude_present() {
                    issues.push((
                        FacadeIssueCode::MissingFacadePrelude,
                        crate_info.prelude_path,
                        "Add facade `src/prelude.rs`.".to_owned(),
                        Some(FacadeFixKind::RestoreStandardFile),
                    ));
                }

                issues
            })
            .collect::<Vec<_>>();

        for (code, path, message, fix) in documentation_issues {
            self.push_warning(code, Some(path), message, fix);
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
                FacadeIssueCode::MissingToolingSurface,
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
                FacadeIssueCode::MissingGithubCiCdSurface,
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
                FacadeIssueCode::MissingReleaseSurface,
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
                FacadeIssueCode::MissingReleaseCiSurface,
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
                FacadeIssueCode::NonStandardPath,
                Some(path),
                message,
                Some(FacadeFixKind::RemoveNonStandardPath),
            );
        }
    }

    fn push_error(
        &mut self,
        code: FacadeIssueCode,
        path: Option<PathBuf>,
        message: impl Into<String>,
        fix: Option<FacadeFixKind>,
    ) {
        let issue = FacadeIssue::error(code, path, message);
        self.push_issue_with_fix(issue, fix);
    }

    fn push_warning(
        &mut self,
        code: FacadeIssueCode,
        path: Option<PathBuf>,
        message: impl Into<String>,
        fix: Option<FacadeFixKind>,
    ) {
        let issue = FacadeIssue::warning(code, path, message);
        self.push_issue_with_fix(issue, fix);
    }

    fn push_issue_with_fix(&mut self, issue: FacadeIssue, fix: Option<FacadeFixKind>) {
        let issue = match fix {
            Some(fix) => issue.with_fix(fix),
            None => issue,
        };

        self.push_issue(issue);
    }

    fn push_issue(&mut self, issue: FacadeIssue) {
        self.issues.push(issue);
    }
}

const fn facade_severity(severity: ManifestIssueSeverity) -> FacadeIssueSeverity {
    match severity {
        ManifestIssueSeverity::Error => FacadeIssueSeverity::Error,
        ManifestIssueSeverity::Warning => FacadeIssueSeverity::Warning,
    }
}

const fn manifest_issue_fix(code: FacadeIssueCode) -> Option<FacadeFixKind> {
    match code {
        FacadeIssueCode::MissingLintsWorkspace => Some(FacadeFixKind::AddWorkspaceLints),
        _ => None,
    }
}
