use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DistributionMode {
    Cargo,
    Copy,
    Cli,
}

impl DistributionMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
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
pub struct CatalogEntry {
    pub name: String,
    pub kind: String,
    pub set: String,
    pub docs_url: String,
    pub api_docs_url: String,
    pub workspace_docs_url: Option<String>,
    pub modes: Vec<DistributionMode>,
}

impl CatalogEntry {
    #[must_use]
    pub fn supports_mode(&self, mode: DistributionMode) -> bool {
        self.modes.contains(&mode)
    }

    #[must_use]
    pub fn modes_label(&self) -> String {
        self.modes
            .iter()
            .map(|mode| mode.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
