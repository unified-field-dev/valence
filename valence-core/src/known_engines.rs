//! Stable engine id slugs for first-party reference adapters.
//!
//! These constants are ergonomic aliases only — **not** a closed set. Third-party adapters
//! define their own [`crate::DatabaseBackend::engine_id`] strings.

/// Well-known [`DatabaseBackend`](crate::DatabaseBackend)::engine_id values shipped with this workspace.
pub struct KnownEngines;

impl KnownEngines {
    pub const INMEMORY_MEM: &'static str = "inmemory_mem";
    pub const SURREALDB: &'static str = "surrealdb";
    pub const POSTGRES: &'static str = "postgres";
    pub const SQLITE: &'static str = "sqlite";
    pub const MONGODB: &'static str = "mongodb";
    pub const REDIS: &'static str = "redis";
    pub const INDRADB: &'static str = "indradb";
}
