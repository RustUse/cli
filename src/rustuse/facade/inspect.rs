//! Facade repository surface inspection.

use crate::rustuse::facade::{
    FACADE_OPTIONAL_DIRECTORIES, FACADE_OPTIONAL_ROOT_FILES, FACADE_REQUIRED_DIRECTORIES,
    FACADE_REQUIRED_ROOT_FILES,
};
use crate::rustuse::root::scan::{
    RepositorySurfaceReport, SurfaceProfile, inspect_repository_surface,
};

use super::discover::FacadeInfo;

const FACADE_SURFACE_PROFILE: SurfaceProfile = SurfaceProfile {
    required_files: FACADE_REQUIRED_ROOT_FILES,
    optional_files: FACADE_OPTIONAL_ROOT_FILES,
    required_directories: FACADE_REQUIRED_DIRECTORIES,
    optional_directories: FACADE_OPTIONAL_DIRECTORIES,
};

pub(crate) type FacadeRepositoryReport = RepositorySurfaceReport;

pub(crate) fn inspect_facade_repository(facade: &FacadeInfo) -> FacadeRepositoryReport {
    inspect_repository_surface(&facade.root, &FACADE_SURFACE_PROFILE)
}
