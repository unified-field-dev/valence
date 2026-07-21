//! Multi-logical registration helper (integration tests; avoids core↔mem unit-test cycle).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use valence_backend_mem::InMemoryBackend;
use valence_core::{
    register_backend_logical_names, register_backend_logical_names_slices, router_key,
    DatabaseBackend, DatabaseRouter, RegisterBackendLogicalNamesOptions,
};

fn mem_backend() -> Arc<dyn DatabaseBackend> {
    Arc::new(InMemoryBackend::new())
}

#[test]
fn slices_dedupe_across_groups() {
    let backend = mem_backend();
    let mut router = DatabaseRouter::new();
    register_backend_logical_names_slices(
        &mut router,
        Arc::clone(&backend),
        &[&["default", "billing"], &["billing", "jobs"]],
        RegisterBackendLogicalNamesOptions::default(),
    );
    assert_eq!(router.len().expect("len"), 3);
    let engine_id = backend.engine_id();
    assert_eq!(
        router
            .resolve(&router_key("default", engine_id))
            .expect("default")
            .engine_id(),
        engine_id
    );
    assert!(router.resolve(&router_key("billing", engine_id)).is_ok());
    assert!(router.resolve(&router_key("jobs", engine_id)).is_ok());
}

#[test]
fn keys_match_router_key_and_engine_id() {
    let backend = mem_backend();
    let engine_id = backend.engine_id().to_owned();
    let mut router = DatabaseRouter::new();
    register_backend_logical_names(
        &mut router,
        backend,
        &["analytics"],
        RegisterBackendLogicalNamesOptions::default(),
    );
    let key = router_key("analytics", &engine_id);
    let resolved = router.resolve(&key).expect("resolve");
    assert_eq!(resolved.engine_id(), engine_id);
}

#[test]
fn two_logicals_share_same_arc() {
    let backend = mem_backend();
    let mut router = DatabaseRouter::new();
    register_backend_logical_names(
        &mut router,
        Arc::clone(&backend),
        &["default", "billing"],
        RegisterBackendLogicalNamesOptions::default(),
    );
    let engine_id = backend.engine_id();
    let a = router
        .resolve(&router_key("default", engine_id))
        .expect("default");
    let b = router
        .resolve(&router_key("billing", engine_id))
        .expect("billing");
    assert!(Arc::ptr_eq(&a, &b));
    assert!(Arc::ptr_eq(&a, &backend));
}

#[test]
fn empty_input_is_noop() {
    let backend = mem_backend();
    let mut router = DatabaseRouter::new();
    register_backend_logical_names(
        &mut router,
        Arc::clone(&backend),
        &[],
        RegisterBackendLogicalNamesOptions::default(),
    );
    assert_eq!(router.len().expect("len"), 0);
    register_backend_logical_names_slices(
        &mut router,
        backend,
        &[],
        RegisterBackendLogicalNamesOptions::default(),
    );
    assert_eq!(router.len().expect("len"), 0);
    assert!(router.is_empty().expect("empty"));
}

#[test]
fn alias_engine_registers_second_key() {
    let backend = mem_backend();
    let engine_id = backend.engine_id().to_owned();
    let mut router = DatabaseRouter::new();
    register_backend_logical_names(
        &mut router,
        backend,
        &["default"],
        RegisterBackendLogicalNamesOptions {
            // Same dialect migration shim only — not a cross-engine alias.
            register_alias_engine_id: Some("legacy_mem"),
        },
    );
    assert_eq!(router.len().expect("len"), 2);
    assert!(router.resolve(&router_key("default", &engine_id)).is_ok());
    assert!(router.resolve(&router_key("default", "legacy_mem")).is_ok());
}
