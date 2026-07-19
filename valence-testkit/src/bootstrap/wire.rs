//! Explicit wire-backend builder options for bootstrap and bench.

/// Coded wire adapter configuration passed into [`super::BootstrapSession`].
#[derive(Debug, Clone, Default)]
pub struct WireBackendOptions {
    #[cfg(feature = "redis")]
    pub redis: Option<valence_backend_redis::RedisBackendBuilder>,
    #[cfg(feature = "mongodb")]
    pub mongodb: Option<valence_backend_mongodb::MongoBackendBuilder>,
    #[cfg(feature = "postgres")]
    pub postgres: Option<valence_backend_postgres::PostgresBackendBuilder>,
    #[cfg(feature = "redis")]
    pub redis_fleet: Option<valence_backend_redis::FleetRedisBackendBuilder>,
}

impl WireBackendOptions {
    /// Empty options — wire adapters fall back to `builder().from_env_defaults()`.
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "redis")]
    pub fn redis(mut self, builder: valence_backend_redis::RedisBackendBuilder) -> Self {
        self.redis = Some(builder);
        self
    }

    #[cfg(feature = "mongodb")]
    pub fn mongodb(mut self, builder: valence_backend_mongodb::MongoBackendBuilder) -> Self {
        self.mongodb = Some(builder);
        self
    }

    #[cfg(feature = "postgres")]
    pub fn postgres(mut self, builder: valence_backend_postgres::PostgresBackendBuilder) -> Self {
        self.postgres = Some(builder);
        self
    }

    #[cfg(feature = "redis")]
    pub fn redis_fleet(mut self, builder: valence_backend_redis::FleetRedisBackendBuilder) -> Self {
        self.redis_fleet = Some(builder);
        self
    }
}
