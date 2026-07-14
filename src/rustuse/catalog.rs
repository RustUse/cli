//! RustUse crate and primitive catalog.
//!
//! Search and lookup use generated static data so catalog commands work
//! anywhere, without discovering a local RustUse root. Filesystem discovery of
//! `use-*` repositories belongs to the scan/report/check/facade workflows.

pub(crate) mod generated;
pub(crate) mod index;
pub(crate) mod lookup;
pub(crate) mod model;
pub(crate) mod search;

pub(crate) use lookup::{all_entries, find_by_name};
pub(crate) use search::search;
