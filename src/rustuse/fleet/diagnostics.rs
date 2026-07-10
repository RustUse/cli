use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::rustuse::facade::manifest::FacadeManifestReport;
use crate::rustuse::fleet::discover::{
    FleetFacadeEntry, FleetRepoEntry, discover_facades, discover_fleet_repos, resolve_existing_path,
};
use crate::rustuse::fleet::manifest::analyze_fleet_manifests;

#[derive(Debug)]
pub(crate) struct FleetDiagnostics {
    pub(crate) fleet: PathBuf,
    pub(crate) name: String,
    pub(crate) repos: Vec<FleetRepoEntry>,
    pub(crate) facades: Vec<FleetFacadeEntry>,
    pub(crate) manifests: Vec<FacadeManifestReport>,
}

impl FleetDiagnostics {
    pub(crate) fn inspect(path: &Path) -> Result<Self> {
        let fleet = resolve_existing_path(path);
        let name = fleet
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>")
            .to_owned();
        let repos = discover_fleet_repos(&fleet);
        let facades = discover_facades(&fleet)?;
        let manifests = analyze_fleet_manifests(&fleet, &facades)?;

        Ok(Self {
            fleet,
            name,
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
        self.facades.len()
    }

    pub(crate) fn facade_git_count(&self) -> usize {
        self.facades.iter().filter(|facade| facade.has_git).count()
    }

    pub(crate) fn missing_facade_git_count(&self) -> usize {
        self.facade_count().saturating_sub(self.facade_git_count())
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
        self.missing_fleet_repo_count() > 0
            || self.facade_count() == 0
            || self.missing_facade_git_count() > 0
            || self.manifest_warning_count() > 0
    }
}
