//! Generated RustUse catalog data.
//!
//! This module is the static source of truth for `rustuse search` and
//! `rustuse info`. It builds [`CatalogEntry`] values from the shared catalog
//! model and never touches the filesystem, so catalog commands work anywhere.

use super::model::{CatalogEntry, DistributionMode};

/// Returns the built-in RustUse catalog entries.
pub(crate) fn entries() -> Vec<CatalogEntry> {
    vec![
        CatalogEntry {
            name: "use-geometry".to_owned(),
            kind: "focused crate".to_owned(),
            set: "use-geometry".to_owned(),
            docs_url: "https://rustuse.org/use-geometry/".to_owned(),
            api_docs_url: "https://rustuse.org/api/use-geometry/".to_owned(),
            workspace_docs_url: None,
            modes: vec![DistributionMode::Cargo, DistributionMode::Cli],
        },
        CatalogEntry {
            name: "use-combinatorics".to_owned(),
            kind: "child crate".to_owned(),
            set: "use-math".to_owned(),
            docs_url: "https://rustuse.org/use-math/use-combinatorics/".to_owned(),
            api_docs_url: "https://rustuse.org/api/use-combinatorics/".to_owned(),
            workspace_docs_url: Some("https://rustuse.org/api/workspaces/use-math/".to_owned()),
            modes: vec![DistributionMode::Cargo, DistributionMode::Cli],
        },
        CatalogEntry {
            name: "use-math".to_owned(),
            kind: "facade crate".to_owned(),
            set: "use-math".to_owned(),
            docs_url: "https://rustuse.org/use-math/".to_owned(),
            api_docs_url: "https://rustuse.org/api/use-math/".to_owned(),
            workspace_docs_url: Some("https://rustuse.org/api/workspaces/use-math/".to_owned()),
            modes: vec![DistributionMode::Cargo, DistributionMode::Cli],
        },
        CatalogEntry {
            name: "use-slug".to_owned(),
            kind: "child crate".to_owned(),
            set: "use-text".to_owned(),
            docs_url: "https://rustuse.org/use-text/use-slug/".to_owned(),
            api_docs_url: "https://rustuse.org/api/use-slug/".to_owned(),
            workspace_docs_url: None,
            modes: vec![DistributionMode::Cargo, DistributionMode::Cli],
        },
    ]
}
