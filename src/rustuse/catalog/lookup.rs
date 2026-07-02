//! Catalog lookups over the generated RustUse index.

use super::index;
use super::model::CatalogEntry;

/// Returns every catalog entry.
pub(crate) fn all_entries() -> &'static [CatalogEntry] {
    index::entries()
}

/// Finds a catalog entry by its exact name.
pub(crate) fn find_by_name(name: &str) -> Option<&'static CatalogEntry> {
    index::entries().iter().find(|entry| entry.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_known_entry_by_name() {
        let entry = find_by_name("use-geometry").expect("use-geometry is in the catalog");
        assert_eq!(entry.name, "use-geometry");
    }

    #[test]
    fn unknown_name_returns_none() {
        assert!(find_by_name("use-does-not-exist").is_none());
    }

    #[test]
    fn all_entries_is_non_empty() {
        assert!(!all_entries().is_empty());
    }
}
