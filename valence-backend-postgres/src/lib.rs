//! Postgres [`DatabaseBackend`](valence_core::DatabaseBackend) adapter.

#![deny(missing_docs)]

mod backend;
mod config;

pub use backend::{PostgresBackend, ENGINE_ID, PRIMARY};
pub use config::{PostgresBackendBuilder, URL_ENV};
