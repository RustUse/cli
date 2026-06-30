use anyhow::Result;

use super::{discover::discover_catalog, model::CatalogEntry};

pub fn search(query: &str) -> Result<Vec<CatalogEntry>> {
    let query = query.trim().to_ascii_lowercase();

    if query.is_empty() {
        return Ok(Vec::new());
    }

    let results = discover_catalog()?
        .into_iter()
        .filter(|entry| {
            entry.name.contains(&query)
                || entry.kind.contains(&query)
                || entry.set.contains(&query)
                || entry
                    .modes
                    .iter()
                    .any(|mode| mode.as_str().contains(&query))
        })
        .collect();

    Ok(results)
}
