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
