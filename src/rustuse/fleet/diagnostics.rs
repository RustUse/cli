use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::rustuse::facade::manifest::FacadeManifestReport;
use crate::rustuse::fleet::discover::{
    FleetFacadeEntry, FleetRepoEntry, discover_facades, discover_fleet_repos, resolve_existing_path,
};
use crate::rustuse::fleet::inspect::{RootSummary, display_name, inspect_root};
use crate::rustuse::fleet::manifest::analyze_fleet_manifests;

#[derive(Debug)]
pub(crate) struct FleetDiagnostics {
    pub(crate) fleet: PathBuf,
    pub(crate) name: String,
    root_summary: RootSummary,
    pub(crate) repos: Vec<FleetRepoEntry>,
    pub(crate) facades: Vec<FleetFacadeEntry>,
    pub(crate) manifests: Vec<FacadeManifestReport>,
}

impl FleetDiagnostics {
    pub(crate) fn inspect(path: &Path) -> Result<Self> {
        let fleet = resolve_existing_path(path);
        let name = display_name(&fleet).to_owned();
        let root_summary = inspect_root(&fleet)?;
        let repos = discover_fleet_repos(&fleet);
        let facades = discover_facades(&fleet)?;
        let manifests = analyze_fleet_manifests(&fleet, &facades)?;

        Ok(Self {
            fleet,
            name,
            root_summary,
            repos,
            facades,
            manifests,
        })
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.manifest_error_count() > 0 {
            "error"
        } else if self.has_warning() {
            "warning"
        } else {
            "ok"
        }
    }

    pub(crate) fn facade_count(&self) -> usize {
        self.root_summary.use_dir_count
    }

    pub(crate) fn facade_git_count(&self) -> usize {
        self.root_summary.facade_git_count
    }

    pub(crate) fn missing_facade_git_count(&self) -> usize {
        self.root_summary.missing_git.len()
    }

    pub(crate) fn has_cli(&self) -> bool {
        self.root_summary.has_cli
    }

    pub(crate) fn has_docs(&self) -> bool {
        self.root_summary.has_docs
    }

    pub(crate) fn missing_facade_git_names(&self) -> impl Iterator<Item = &str> {
        self.root_summary
            .missing_git
            .iter()
            .map(|path| display_name(path))
    }

    pub(crate) fn child_crate_count(&self) -> usize {
        self.facades
            .iter()
            .map(|facade| facade.child_crate_count)
            .sum()
    }

    pub(crate) fn missing_fleet_repo_count(&self) -> usize {
        self.repos.iter().filter(|repo| !repo.present).count()
    }

    pub(crate) fn manifest_count(&self) -> usize {
        self.manifests
            .iter()
            .map(FacadeManifestReport::manifest_count)
            .sum()
    }

    pub(crate) fn manifest_issue_count(&self) -> usize {
        self.manifests
            .iter()
            .map(FacadeManifestReport::issue_count)
            .sum()
    }

    pub(crate) fn manifest_error_count(&self) -> usize {
        self.manifests
            .iter()
            .map(FacadeManifestReport::error_count)
            .sum()
    }

    pub(crate) fn manifest_warning_count(&self) -> usize {
        self.manifests
            .iter()
            .map(FacadeManifestReport::warning_count)
            .sum()
    }

    pub(crate) fn invalid_category_count(&self) -> usize {
        self.manifests
            .iter()
            .map(FacadeManifestReport::invalid_category_count)
            .sum()
    }

    fn has_warning(&self) -> bool {
        !self.has_cli()
            || !self.has_docs()
            || self.missing_fleet_repo_count() > 0
            || self.facade_count() == 0
            || self.missing_facade_git_count() > 0
            || self.manifest_warning_count() > 0
    }
}
