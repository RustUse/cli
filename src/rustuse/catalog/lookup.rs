use anyhow::Result;

use super::{discover::discover_catalog, model::CatalogEntry};

pub fn all_entries() -> Result<Vec<CatalogEntry>> {
    discover_catalog()
}

pub fn find_by_name(name: &str) -> Result<Option<CatalogEntry>> {
    Ok(discover_catalog()?
        .into_iter()
        .find(|entry| entry.name == name))
}

/*


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

#[derive(Clone, Copy, Debug)]
pub struct RustUseEntry {
    pub name: &'static str,
    pub kind: &'static str,
    pub set: &'static str,
    pub docs_url: &'static str,
    pub api_docs_url: &'static str,
    pub workspace_docs_url: Option<&'static str>,
    pub modes: &'static [DistributionMode],
}

impl RustUseEntry {
    #[must_use]
    pub fn supports_mode(self, mode: DistributionMode) -> bool {
        self.modes.contains(&mode)
    }

    #[must_use]
    pub fn modes_label(self) -> String {
        self.modes
            .iter()
            .map(|mode| mode.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

const CARGO_COPY_CLI: &[DistributionMode] = &[
    DistributionMode::Cargo,
    DistributionMode::Copy,
    DistributionMode::Cli,
];
const CARGO_CLI: &[DistributionMode] = &[DistributionMode::Cargo, DistributionMode::Cli];

const ENTRIES: &[RustUseEntry] = &[
    RustUseEntry {
        name: "use-geometry",
        kind: "focused crate",
        set: "use-geometry",
        docs_url: "https://rustuse.org/use-geometry/",
        api_docs_url: "https://rustuse.org/api/use-geometry/",
        workspace_docs_url: None,
        modes: CARGO_COPY_CLI,
    },
    RustUseEntry {
        name: "use-combinatorics",
        kind: "child crate",
        set: "use-math",
        docs_url: "https://rustuse.org/use-math/use-combinatorics/",
        api_docs_url: "https://rustuse.org/api/use-combinatorics/",
        workspace_docs_url: Some("https://rustuse.org/api/workspaces/use-math/"),
        modes: CARGO_COPY_CLI,
    },
    RustUseEntry {
        name: "use-math",
        kind: "facade crate",
        set: "use-math",
        docs_url: "https://rustuse.org/use-math/",
        api_docs_url: "https://rustuse.org/api/use-math/",
        workspace_docs_url: Some("https://rustuse.org/api/workspaces/use-math/"),
        modes: CARGO_CLI,
    },
    RustUseEntry {
        name: "use-slug",
        kind: "child crate",
        set: "use-text",
        docs_url: "https://rustuse.org/use-text/use-slug/",
        api_docs_url: "https://rustuse.org/api/use-slug/",
        workspace_docs_url: None,
        modes: CARGO_COPY_CLI,
    },
];

#[must_use]
pub const fn all_entries() -> &'static [RustUseEntry] {
    ENTRIES
}

#[must_use]
pub fn find_by_name(name: &str) -> Option<RustUseEntry> {
    ENTRIES.iter().copied().find(|entry| entry.name == name)
}

#[must_use]
pub fn search(query: &str) -> Vec<RustUseEntry> {
    let query = query.trim().to_ascii_lowercase();

    if query.is_empty() {
        return Vec::new();
    }

    ENTRIES
        .iter()
        .copied()
        .filter(|entry| {
            entry.name.contains(&query)
                || entry.kind.contains(&query)
                || entry.set.contains(&query)
                || entry
                    .modes
                    .iter()
                    .any(|mode| mode.as_str().contains(&query))
        })
        .collect()
}


*/
