//! MongoDB wire [`DatabaseBackend`](valence_core::DatabaseBackend) adapter for Valence.

#![deny(missing_docs)]

mod backend;
mod config;

pub use backend::{MongoBackend, ENGINE_ID, PRIMARY};
pub use config::{MongoBackendBuilder, MongoConfig, DATABASE_ENV, TEST_URI_ENV, URI_ENV};
