use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DistributionMode {
    Cargo,
    Copy,
    Cli,
}

impl DistributionMode {
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::Copy => "copy",
            Self::Cli => "cli",
        }
    }
}

impl fmt::Display for DistributionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CatalogEntry {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) set: String,
    pub(crate) docs_url: String,
    pub(crate) api_docs_url: String,
    pub(crate) workspace_docs_url: Option<String>,
    pub(crate) modes: Vec<DistributionMode>,
}
