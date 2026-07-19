//! Redis wire [`DatabaseBackend`](valence_core::DatabaseBackend) adapter for Valence.

#![deny(missing_docs)]

mod backend;
mod config;
mod fleet;
mod keys;

pub use backend::{RedisBackend, ENGINE_ID, PRIMARY};
pub use config::{
    test_url, FleetRedisBackendBuilder, RedisBackendBuilder, RedisConfig, KEY_PREFIX_ENV,
    TEST_URL_ENV, URLS_ENV, URL_ENV,
};
pub use fleet::{connect_fleet_arc, FleetRedisBackend};
