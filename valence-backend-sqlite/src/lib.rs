//! SQLite [`DatabaseBackend`](valence_core::DatabaseBackend) adapter.

#![deny(missing_docs)]

mod backend;

pub use backend::{SqliteBackend, ENGINE_ID, PRIMARY};
