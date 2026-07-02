//! In-memory index over the generated RustUse catalog.
//!
//! The index is built once from [`generated`] data and sorted by name so that
//! lookups and search return deterministic results without touching the
//! filesystem.

use std::sync::OnceLock;

use super::generated;
use super::model::CatalogEntry;

/// Returns the sorted catalog index, building it on first use.
pub(crate) fn entries() -> &'static [CatalogEntry] {
    static INDEX: OnceLock<Vec<CatalogEntry>> = OnceLock::new();

    INDEX.get_or_init(|| {
        let mut entries = generated::entries();
        entries.sort_by(|left, right| left.name.cmp(&right.name));
        entries.dedup_by(|left, right| left.name == right.name);
        entries
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_is_sorted_by_name() {
        let entries = entries();
        let mut sorted = entries.to_vec();
        sorted.sort_by(|left, right| left.name.cmp(&right.name));

        assert_eq!(entries, sorted.as_slice());
    }

    #[test]
    fn index_has_no_duplicate_names() {
        let entries = entries();
        let mut names: Vec<&str> = entries.iter().map(|entry| entry.name.as_str()).collect();
        names.sort_unstable();
        let unique = names.len();
        names.dedup();

        assert_eq!(unique, names.len());
    }
}
