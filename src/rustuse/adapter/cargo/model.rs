#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CargoManifestKind {
    WorkspaceRoot,
    Package,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CargoPackageName(pub(crate) String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CargoFeatureName(pub(crate) String);
