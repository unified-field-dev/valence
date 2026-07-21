//! Register one hybrid backend under several logical names.
//!
//! Uses an in-memory primary so the example runs without `DATABASE_URL`.
//! Production hosts typically use a SQL primary (e.g. Postgres) instead.
//!
//! ```bash
//! cargo run -p uf-valence --example hybrid_multi_logical --features hybrid,mem
//! ```

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::print_stdout,
    clippy::print_stderr
)]

use std::sync::Arc;

use valence::{
    register_backend_logical_names_slices, router_key, DatabaseBackend, DatabaseRouter,
    HybridBackend, InMemoryBackend, RegisterBackendLogicalNamesOptions, HYBRID_ENGINE_ID,
};

#[tokio::main]
async fn main() -> valence::Result<()> {
    // Mem primary keeps the example offline; durable deployments use a SQL primary.
    let primary: Arc<dyn DatabaseBackend> = Arc::new(InMemoryBackend::new());
    let backend: Arc<dyn DatabaseBackend> = Arc::new(
        HybridBackend::builder()
            .primary(primary)
            .warm_edges(true)
            .build()
            .await?,
    );

    let mut router = DatabaseRouter::new();
    register_backend_logical_names_slices(
        &mut router,
        backend,
        &[&["default"], &["billing"], &["jobs"]],
        RegisterBackendLogicalNamesOptions::default(),
    );

    for logical in ["default", "billing", "jobs"] {
        let key = router_key(logical, HYBRID_ENGINE_ID);
        let resolved = router.resolve(&key)?;
        assert_eq!(resolved.engine_id(), HYBRID_ENGINE_ID);
        println!("hybrid_multi_logical: resolved {key}");
    }
    Ok(())
}
