//! JSON merge helpers.

use serde_json::{Map, Value};

/// Shallow merge `patch` into `base`.
pub fn json_merge(base: &mut Map<String, Value>, patch: &Map<String, Value>) {
    for (k, v) in patch {
        if k != "id" {
            base.insert(k.clone(), v.clone());
        }
    }
}
