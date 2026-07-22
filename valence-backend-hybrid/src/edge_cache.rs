//! Edge warm-up, completeness tracking, and capacity-bounded dual-write helpers.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use valence_backend_indradb::IndradbBackend;
use valence_core::compiled_query::CompiledQuery;
use valence_core::error::Result;
use valence_core::record_id::RecordId;
use valence_core::DatabaseBackend;

use crate::cache_policy::CachePolicy;

/// Tracks which edge tables are fully represented in the IndraDB mirror.
#[derive(Debug, Default)]
pub struct EdgeCache {
    state: Mutex<EdgeState>,
}

#[derive(Debug, Default)]
struct EdgeState {
    /// Edge tables known to be complete in the mirror.
    complete: HashSet<String>,
    /// Edge tables marked incomplete (capacity overflow or failed warm-up).
    incomplete: HashSet<String>,
    /// Total edges currently held in the mirror.
    edge_count: usize,
    /// Per-table edge counts for capacity accounting.
    per_table: HashMap<String, usize>,
}

impl EdgeCache {
    /// Create an empty edge-cache tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether `edge_table` may be served from the mirror.
    #[must_use]
    pub fn is_complete(&self, edge_table: &str) -> bool {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.complete.contains(edge_table) && !state.incomplete.contains(edge_table)
    }

    /// Mark an edge table incomplete so hops/reads fall back to the primary.
    pub fn mark_incomplete(&self, edge_table: &str) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.complete.remove(edge_table);
        state.incomplete.insert(edge_table.to_string());
        crate::telemetry::record_edge_fallback(edge_table);
    }

    /// Bulk-load `valence_edges` from the primary into the mirror.
    ///
    /// Tables that would exceed `policy.edge_capacity` are marked incomplete.
    ///
    /// # Errors
    ///
    /// Propagates primary query or mirror write errors that prevent warm-up.
    pub async fn warm_from_primary(
        &self,
        primary: &Arc<dyn DatabaseBackend>,
        mirror: &IndradbBackend,
        policy: &CachePolicy,
    ) -> Result<()> {
        if policy.edge_capacity == 0 {
            return Ok(());
        }
        let compiled = CompiledQuery::new(
            "SELECT from_table, from_id, edge_type, to_table, to_id FROM valence_edges".into(),
            vec![],
        );
        let rows = match primary.execute_compiled_query(&compiled).await {
            Ok(rows) => rows,
            Err(_) => {
                // Empty / missing edges table is fine on a fresh database.
                return Ok(());
            }
        };

        let mut by_type: HashMap<String, Vec<(RecordId, RecordId)>> = HashMap::new();
        for row in rows {
            let Some(edge_type) = row.get("edge_type").and_then(|v| v.as_str()) else {
                continue;
            };
            if !policy.caches_edge(edge_type) {
                continue;
            }
            let Some(from) = record_id_from_parts(row.get("from_table"), row.get("from_id")) else {
                continue;
            };
            let Some(to) = record_id_from_parts(row.get("to_table"), row.get("to_id")) else {
                continue;
            };
            by_type
                .entry(edge_type.to_string())
                .or_default()
                .push((from, to));
        }

        for (edge_table, edges) in by_type {
            self.warm_edge_table(mirror, policy, &edge_table, edges)
                .await?;
        }
        Ok(())
    }

    /// Load one edge table, or mark it incomplete when capacity would be exceeded.
    async fn warm_edge_table(
        &self,
        mirror: &IndradbBackend,
        policy: &CachePolicy,
        edge_table: &str,
        edges: Vec<(RecordId, RecordId)>,
    ) -> Result<()> {
        let len = edges.len();
        {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if state.edge_count.saturating_add(len) > policy.edge_capacity {
                drop(state);
                self.mark_incomplete(edge_table);
                return Ok(());
            }
        }
        for (from, to) in edges {
            mirror.relate_edge(&from, edge_table, &to).await?;
        }
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.edge_count = state.edge_count.saturating_add(len);
        *state.per_table.entry(edge_table.to_string()).or_default() += len;
        state.incomplete.remove(edge_table);
        state.complete.insert(edge_table.to_string());
        Ok(())
    }

    /// Dual-write a new edge; marks the table incomplete if capacity is exceeded.
    ///
    /// # Errors
    ///
    /// Propagates mirror relate errors when the table is still considered cacheable.
    pub async fn relate(
        &self,
        mirror: &IndradbBackend,
        policy: &CachePolicy,
        from: &RecordId,
        edge_table: &str,
        to: &RecordId,
    ) -> Result<()> {
        if !policy.caches_edge(edge_table) {
            return Ok(());
        }
        if self.is_incomplete(edge_table) {
            return Ok(());
        }
        {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if state.edge_count >= policy.edge_capacity {
                drop(state);
                self.mark_incomplete(edge_table);
                return Ok(());
            }
        }
        mirror.relate_edge(from, edge_table, to).await?;
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.edge_count = state.edge_count.saturating_add(1);
        *state.per_table.entry(edge_table.to_string()).or_default() += 1;
        if !state.incomplete.contains(edge_table) {
            state.complete.insert(edge_table.to_string());
        }
        Ok(())
    }

    /// Remove an edge from the mirror when the table is still cached.
    ///
    /// # Errors
    ///
    /// Propagates mirror unrelate errors.
    pub async fn unrelate(
        &self,
        mirror: &IndradbBackend,
        from: &RecordId,
        edge_table: &str,
        to: &RecordId,
    ) -> Result<()> {
        if self.is_incomplete(edge_table) || !self.is_complete(edge_table) {
            return Ok(());
        }
        mirror.unrelate_edge(from, edge_table, to).await?;
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.edge_count = state.edge_count.saturating_sub(1);
        if let Some(count) = state.per_table.get_mut(edge_table) {
            *count = count.saturating_sub(1);
        }
        Ok(())
    }

    /// Whether the table was marked incomplete.
    fn is_incomplete(&self, edge_table: &str) -> bool {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.incomplete.contains(edge_table)
    }
}

/// Build a [`RecordId`] from JSON string parts.
fn record_id_from_parts(
    table: Option<&serde_json::Value>,
    id: Option<&serde_json::Value>,
) -> Option<RecordId> {
    let table = table.and_then(|v| v.as_str())?;
    let id = id.and_then(|v| v.as_str())?;
    Some(RecordId::new(table, id))
}
