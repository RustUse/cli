//! Catalog search over the generated RustUse index.

use super::index;
use super::model::CatalogEntry;

/// Returns catalog entries matching `query` (case-insensitive substring).
///
/// This searches the static catalog index and never touches the filesystem, so
/// it works outside a RustUse root.
pub(crate) fn search(query: &str) -> Vec<&'static CatalogEntry> {
    let query = query.trim().to_ascii_lowercase();

    if query.is_empty() {
        return Vec::new();
    }

    index::entries()
        .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_geometry_outside_a_rustuse_root() {
        let results = search("geometry");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "use-geometry");
    }

    #[test]
    fn search_is_case_insensitive() {
        assert_eq!(search("GEOMETRY").len(), 1);
    }

    #[test]
    fn empty_query_returns_no_results() {
        assert!(search("   ").is_empty());
    }

    #[test]
    fn matches_by_set() {
        let results = search("use-math");
        assert!(results.iter().any(|entry| entry.name == "use-math"));
        assert!(
            results
                .iter()
                .any(|entry| entry.name == "use-combinatorics")
        );
    }
}
