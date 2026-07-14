use std::path::Path;

use anyhow::Result;

use crate::rustuse::facade::manifest::{FacadeManifestReport, analyze_facade_repository_manifests};
use crate::rustuse::fleet::discover::FleetFacadeEntry;

pub(crate) fn analyze_fleet_manifests(
    fleet: &Path,
    facades: &[FleetFacadeEntry],
) -> Result<Vec<FacadeManifestReport>> {
    facades
        .iter()
        .map(|facade| analyze_fleet_facade_manifests(fleet, facade))
        .collect()
}

fn analyze_fleet_facade_manifests(
    fleet: &Path,
    facade: &FleetFacadeEntry,
) -> Result<FacadeManifestReport> {
    analyze_facade_repository_manifests(&fleet.join(&facade.name), &facade.name)
}
