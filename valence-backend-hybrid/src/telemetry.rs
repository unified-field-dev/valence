//! Hybrid-backend metrics hooks (thin wrappers over valence-core instrumentation).

use valence_core::instrumentation;

/// Record a hybrid cache hit for `kind` (`"record"` or `"edge"`).
pub fn record_cache_hit(kind: &str) {
    instrumentation::record_ownership_fetch_mode(&format!("hybrid_hit_{kind}"));
}

/// Record a hybrid cache miss for `kind`.
pub fn record_cache_miss(kind: &str) {
    instrumentation::record_ownership_fetch_mode(&format!("hybrid_miss_{kind}"));
}

/// Record that a record body was evicted from the LRU.
pub fn record_record_eviction(table: &str) {
    let _ = table;
    instrumentation::record_ownership_fetch_mode("hybrid_record_evict");
}

/// Record that an edge table fell back to the primary because it is incomplete.
pub fn record_edge_fallback(edge_table: &str) {
    let _ = edge_table;
    instrumentation::record_ownership_fetch_mode("hybrid_edge_fallback");
}

/// Record that a hop plan executed via the hybrid path.
pub fn record_hop_plan() {
    instrumentation::record_ownership_fetch_mode("hybrid_hop_plan");
}

/// Record a soft mirror failure after a successful primary write.
pub fn record_mirror_soft_failure(op: &str) {
    instrumentation::record_db_error("hybrid_mirror", op, "soft_fail");
}
