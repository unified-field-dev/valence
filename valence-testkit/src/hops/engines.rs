//! Engines that participate in hop Cartesian (excludes acme-stub).

use crate::matrix::StorageAdapter;

/// Storage adapters eligible for cross-backend hop layouts.
pub fn hop_storage_engines() -> Vec<StorageAdapter> {
    crate::matrix::all_storage_adapters()
        .into_iter()
        .filter(|s| !matches!(s, StorageAdapter::AcmeStub))
        .collect()
}
