//! Runtime resolution of which [`crate::backend::DatabaseBackend`] a schema uses.
//!
//! Schema `database:` fields point at a [`DatabaseEvaluator`] (usually [`DatabaseFromEngine`]).
//! That is **router-key** selection — not physical URL resolution
//! ([`crate::DatabaseEndpointResolver`] is bootstrap-only).

use crate::backend::DatabaseBackend;
use crate::error::Result;
use crate::router::DatabaseRouter;
use crate::router_key::router_key;
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct ResolverContext {
    pub environment: Option<String>,
}

#[async_trait]
pub trait DatabaseEvaluator: Send + Sync + std::fmt::Debug + Any + 'static {
    /// Logical database name used in schema metadata (`databases:` labels).
    fn logical_name(&self) -> &'static str;

    /// Alias for [`Self::logical_name`] — used by `valence_schema!` codegen.
    fn name(&self) -> &'static str {
        self.logical_name()
    }

    fn engine_id(&self) -> &'static str;
    async fn resolve(
        &self,
        ctx: &ResolverContext,
        router: &DatabaseRouter,
    ) -> Result<Arc<dyn DatabaseBackend>>;
    fn as_any(&self) -> &dyn Any;
}

/// Built-in evaluator: look up `logical_name` + `engine_id` on a [`DatabaseRouter`].
///
/// Composes [`crate::router_key()`] then [`DatabaseRouter::resolve`].
///
/// # Examples
///
/// ```
/// use valence_backend_mem::ENGINE_ID;
/// use valence_core::{Database, DatabaseEvaluator};
///
/// let db = Database::from_engine("default", ENGINE_ID);
/// assert_eq!(db.logical_name(), "default");
/// assert_eq!(db.engine_id(), ENGINE_ID);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DatabaseFromEngine {
    logical_name: &'static str,
    engine_id: &'static str,
}

impl DatabaseFromEngine {
    pub const fn new(logical_name: &'static str, engine_id: &'static str) -> Self {
        Self {
            logical_name,
            engine_id,
        }
    }
}

#[async_trait]
impl DatabaseEvaluator for DatabaseFromEngine {
    fn logical_name(&self) -> &'static str {
        self.logical_name
    }

    fn engine_id(&self) -> &'static str {
        self.engine_id
    }

    async fn resolve(
        &self,
        _ctx: &ResolverContext,
        router: &DatabaseRouter,
    ) -> Result<Arc<dyn DatabaseBackend>> {
        let key = router_key(self.logical_name, self.engine_id);
        router.resolve(&key)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Convenience namespace for built-in evaluators.
pub struct Database;

impl Database {
    pub const fn from_engine(
        logical_name: &'static str,
        engine_id: &'static str,
    ) -> DatabaseFromEngine {
        DatabaseFromEngine::new(logical_name, engine_id)
    }
}

use crate::known_engines::KnownEngines;

pub const DEFAULT_IN_MEMORY: DatabaseFromEngine =
    DatabaseFromEngine::new("default", KnownEngines::INMEMORY_MEM);

pub use crate::router_key::DEFAULT_IN_MEMORY_ROUTER_KEY;
