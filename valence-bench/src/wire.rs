//! Wire backend builder options from CLI flags.

use valence_testkit::WireBackendOptions;

/// Shared wire URL flags for bench CLI.
#[derive(Debug, Clone, Default)]
pub struct WireCliArgs {
    pub redis_url: Option<String>,
    pub mongodb_uri: Option<String>,
    pub postgres_url: Option<String>,
    pub redis_urls: Option<String>,
}

impl WireCliArgs {
    pub fn into_wire_options(self) -> WireBackendOptions {
        let mut opts = WireBackendOptions::new();

        #[cfg(feature = "redis")]
        if let Some(url) = self.redis_url {
            opts = opts.redis(valence_backend_redis::RedisBackendBuilder::new().url(url));
        }
        #[cfg(feature = "redis")]
        if let Some(urls) = self.redis_urls {
            let list: Vec<String> = urls
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            if !list.is_empty() {
                opts = opts
                    .redis_fleet(valence_backend_redis::FleetRedisBackendBuilder::new().urls(list));
            }
        }

        #[cfg(feature = "mongodb")]
        if let Some(uri) = self.mongodb_uri {
            opts = opts.mongodb(valence_backend_mongodb::MongoBackendBuilder::new().uri(uri));
        }

        #[cfg(feature = "postgres")]
        if let Some(url) = self.postgres_url {
            opts = opts.postgres(valence_backend_postgres::PostgresBackendBuilder::new().url(url));
        }

        opts
    }
}
