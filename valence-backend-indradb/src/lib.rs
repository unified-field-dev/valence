//! IndraDB embedded graph [`DatabaseBackend`](valence_core::DatabaseBackend) adapter.

#![deny(missing_docs)]

mod backend;

pub use backend::{IndradbBackend, ENGINE_ID, PRIMARY};
