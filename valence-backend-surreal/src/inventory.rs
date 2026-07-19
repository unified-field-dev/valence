//! Schema inventory helpers for embedded Surreal logical name discovery.

use std::collections::BTreeSet;

use valence_core::SchemaMetadataInit;

/// Include in bootstrap registration so schemas that omit explicit `database:` still resolve.
pub const DEFAULT_EMBEDDED_SURREAL_LOGICAL_NAMES: &[&str] = &["default"];

/// Distinct embedded Surreal logical names from every linked `valence_schema!` plus defaults.
pub fn collect_distinct_embedded_surreal_logical_names() -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for name in DEFAULT_EMBEDDED_SURREAL_LOGICAL_NAMES {
        set.insert((*name).to_string());
    }
    for init in inventory::iter::<SchemaMetadataInit> {
        let meta = (init.0)();
        for name in meta.databases {
            set.insert(name.clone());
        }
    }
    set.into_iter().collect()
}
