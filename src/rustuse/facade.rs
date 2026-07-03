//! RustUse facade repository domain.
//!
//! A facade is a `use-*` repository that owns workspace-level wiring,
//! optional dependencies, feature exports, generated surfaces, repository
//! policy, and child crate discovery.
//!
//! Facade logic is the source of truth for repository shape. Root logic should
//! aggregate facade results instead of defining duplicate policy.

pub(crate) mod ci;
pub(crate) mod discover;
pub(crate) mod documentation;
pub(crate) mod flags;
pub(crate) mod layout;
pub(crate) mod manifest;
pub(crate) mod nonstandard;
pub(crate) mod release;
pub(crate) mod report;
pub(crate) mod scan;
pub(crate) mod standards;

// Add these back only when their command paths are wired.
//
// pub(crate) mod fix;
pub(crate) mod inspect;
// pub(crate) mod model;
// pub(crate) mod policy;
