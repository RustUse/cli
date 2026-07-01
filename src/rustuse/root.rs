//! Root-level orchestration for a local RustUse repository collection.
//!
//! A RustUse root is a development directory that contains multiple
//! repositories, including `use-*` facade repositories.
//!
//! Root logic should stay thin:
//!
//! - discover facade repositories
//! - aggregate facade-level scan/check/report data
//! - coordinate root-level release or publish workflows
//!
//! Facade-specific policy should live under `crate::rustuse::facade`.

pub(crate) mod discover;
pub(crate) mod report;
pub(crate) mod scan;

// Add these back only when their command paths are wired.
//
// pub(crate) mod inspect;
// pub(crate) mod publish;
