//! Hybrid IndraDB cache + SQL primary [`DatabaseBackend`](valence_core::DatabaseBackend).
//!
//! **IndraDB is the in-process cache** for record bodies, graph edges, and M2M hop fan-out.
//! Postgres (or another SQL primary) remains the durable source of truth and the engine for
//! filtered/list SQL. See the crate modules for focused APIs:
//!
//! - [`HybridBackendBuilder`] — capacities, include/exclude rules, edge warm-up
//! - [`CacheRules`] / [`CachePolicy`] — cache selection
//! - [`HybridBackend`] — `DatabaseBackend` implementation
//!
//! # Examples
//!
//! Capacity `0` disables that cache class:
//!
//! ```no_run
//! use std::sync::Arc;
//! use valence_backend_hybrid::HybridBackend;
//! use valence_backend_mem::InMemoryBackend;
//!
//! # async fn demo() -> valence_core::Result<()> {
//! let hybrid = HybridBackend::builder()
//!     .primary(Arc::new(InMemoryBackend::new()))
//!     .record_capacity(0)
//!     .edge_capacity(0)
//!     .warm_edges(false)
//!     .build()
//!     .await?;
//! assert!(!hybrid.policy().caches_record("any"));
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]

mod backend;
mod builder;
mod cache_policy;
mod edge_cache;
mod hop_exec;
mod record_cache;
mod telemetry;
mod write_through;

pub use backend::{HybridBackend, ENGINE_ID, PRIMARY};
pub use builder::HybridBackendBuilder;
pub use cache_policy::{
    CachePolicy, CacheRules, DEFAULT_EDGE_CAPACITY, DEFAULT_RECORD_CAPACITY,
};
pub use hop_exec::{HybridHopBody, HybridHopPlan};
