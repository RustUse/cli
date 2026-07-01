pub mod discover;
pub mod lookup;
pub mod model;
pub mod search;

// pub use lookup::{all_entries, find_by_name};
pub use lookup::all_entries;
// pub use model::{CatalogEntry, DistributionMode};
pub use search::search;
