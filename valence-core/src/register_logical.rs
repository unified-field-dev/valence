//! Multi-logical registration of a shared [`DatabaseBackend`] on a [`DatabaseRouter`].
//!
//! Hosts often connect one backend instance and register it under several logical names.
//! Router keys are `"{engine_id}:{logical_name}"` where `engine_id` comes from
//! [`DatabaseBackend::engine_id`]. Examples:
//!
//! | engine_id | logical_name | router key |
//! |-----------|--------------|------------|
//! | `sqlite` | `default` | `sqlite:default` |
//! | `sqlite` | `billing` | `sqlite:billing` |
//! | `hybrid_indra_sql` | `default` | `hybrid_indra_sql:default` |
//! | `hybrid_indra_sql` | `jobs` | `hybrid_indra_sql:jobs` |
//!
//! Empty `logical_names` / empty `groups` is a no-op (the router is left unchanged).

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::backend::DatabaseBackend;
use crate::router::DatabaseRouter;
use crate::router_key::router_key;

/// Options for multi-logical backend registration.
#[derive(Clone, Copy, Debug, Default)]
pub struct RegisterBackendLogicalNamesOptions {
    /// When set, also register each logical name under this alternate engine id
    /// (host migration shim).
    ///
    /// Aliases must stay within the same compiler/dialect. Never alias across
    /// engines (for example do not map `sqlite` ↔ `hybrid_indra_sql`).
    pub register_alias_engine_id: Option<&'static str>,
}

fn register_backend_for_logical(
    router: &mut DatabaseRouter,
    logical_name: &str,
    backend: Arc<dyn DatabaseBackend>,
    options: RegisterBackendLogicalNamesOptions,
) {
    let engine_id = backend.engine_id();
    let key = router_key(logical_name, engine_id);
    router.register(key, Arc::clone(&backend));
    if let Some(alias) = options.register_alias_engine_id {
        let alias_key = router_key(logical_name, alias);
        router.register(alias_key, backend);
    }
}

/// Register one shared backend under each logical name.
///
/// Keys use [`router_key`] with `backend.engine_id()`. All names share the same
/// [`Arc`]. Passing an empty `logical_names` slice is a no-op.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use valence_backend_mem::InMemoryBackend;
/// use valence_core::{
///     register_backend_logical_names, router_key, DatabaseBackend, DatabaseRouter,
///     RegisterBackendLogicalNamesOptions,
/// };
///
/// let backend: Arc<dyn DatabaseBackend> = Arc::new(InMemoryBackend::new());
/// let engine_id = backend.engine_id().to_owned();
/// let mut router = DatabaseRouter::new();
/// register_backend_logical_names(
///     &mut router,
///     backend,
///     &["default", "billing"],
///     RegisterBackendLogicalNamesOptions::default(),
/// );
/// assert!(router.resolve(&router_key("default", &engine_id)).is_ok());
/// assert!(router.resolve(&router_key("billing", &engine_id)).is_ok());
/// ```
pub fn register_backend_logical_names(
    router: &mut DatabaseRouter,
    backend: Arc<dyn DatabaseBackend>,
    logical_names: &[&str],
    options: RegisterBackendLogicalNamesOptions,
) {
    for &name in logical_names {
        register_backend_for_logical(router, name, Arc::clone(&backend), options);
    }
}

/// Register `backend` under every distinct logical name in `groups` (deduplicated).
///
/// Flattened names are collected into a set so duplicate logical names across
/// slices register once. Empty `groups` (or only empty inner slices) is a no-op.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use valence_backend_mem::InMemoryBackend;
/// use valence_core::{
///     register_backend_logical_names_slices, router_key, DatabaseBackend, DatabaseRouter,
///     RegisterBackendLogicalNamesOptions,
/// };
///
/// let backend: Arc<dyn DatabaseBackend> = Arc::new(InMemoryBackend::new());
/// let engine_id = backend.engine_id().to_owned();
/// let mut router = DatabaseRouter::new();
/// register_backend_logical_names_slices(
///     &mut router,
///     backend,
///     &[&["default"], &["billing"], &["jobs"]],
///     RegisterBackendLogicalNamesOptions::default(),
/// );
/// assert!(router.resolve(&router_key("jobs", &engine_id)).is_ok());
/// ```
pub fn register_backend_logical_names_slices(
    router: &mut DatabaseRouter,
    backend: Arc<dyn DatabaseBackend>,
    groups: &[&[&str]],
    options: RegisterBackendLogicalNamesOptions,
) {
    let mut seen = BTreeSet::<&str>::new();
    for group in groups {
        for &name in *group {
            seen.insert(name);
        }
    }
    for name in seen {
        register_backend_for_logical(router, name, Arc::clone(&backend), options);
    }
}
