//! Record-body LRU metadata layered over the IndraDB mirror.

use std::collections::{HashSet, VecDeque};
use std::sync::Mutex;

use serde_json::Value;
use valence_backend_indradb::IndradbBackend;
use valence_core::error::Result;
use valence_core::DatabaseBackend;

use crate::cache_policy::CachePolicy;

/// Tracks which record bodies are present in the IndraDB mirror and their LRU order.
#[derive(Debug, Default)]
pub struct RecordCache {
    order: Mutex<RecordLruState>,
}

#[derive(Debug, Default)]
struct RecordLruState {
    /// Recency queue: front = oldest, back = newest.
    queue: VecDeque<(String, String)>,
    /// Fast membership for `(table, id)`.
    present: HashSet<(String, String)>,
}

impl RecordCache {
    /// Create an empty record LRU tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up a record in the mirror, touching LRU on hit.
    ///
    /// # Errors
    ///
    /// Propagates IndraDB read errors.
    pub async fn get(
        &self,
        mirror: &IndradbBackend,
        policy: &CachePolicy,
        table: &str,
        id: &str,
    ) -> Result<Option<Value>> {
        if !policy.caches_record(table) {
            return Ok(None);
        }
        let row = mirror.get_record(table, id).await?;
        if row.is_some() {
            self.touch(table, id);
        }
        Ok(row)
    }

    /// Insert or refresh a record body in the mirror and LRU.
    ///
    /// # Errors
    ///
    /// Propagates IndraDB upsert errors.
    pub async fn put(
        &self,
        mirror: &IndradbBackend,
        policy: &CachePolicy,
        table: &str,
        id: &str,
        body: Value,
    ) -> Result<()> {
        if !policy.caches_record(table) {
            return Ok(());
        }
        mirror.upsert_record(table, id, body).await?;
        self.touch(table, id);
        self.evict_overflow(mirror, policy).await?;
        Ok(())
    }

    /// Remove a record from the mirror and LRU (best-effort mirror delete).
    ///
    /// # Errors
    ///
    /// Propagates IndraDB delete errors.
    pub async fn invalidate(&self, mirror: &IndradbBackend, table: &str, id: &str) -> Result<()> {
        self.remove_key(table, id);
        mirror.delete_record(table, id).await
    }

    /// Touch LRU order for an existing or new key.
    fn touch(&self, table: &str, id: &str) {
        let key = (table.to_string(), id.to_string());
        let mut state = self.order.lock().unwrap_or_else(|e| e.into_inner());
        if !state.present.insert(key.clone()) {
            if let Some(pos) = state.queue.iter().position(|k| k == &key) {
                state.queue.remove(pos);
            }
        }
        state.queue.push_back(key);
    }

    /// Drop LRU metadata for a key without touching the mirror.
    fn remove_key(&self, table: &str, id: &str) {
        let key = (table.to_string(), id.to_string());
        let mut state = self.order.lock().unwrap_or_else(|e| e.into_inner());
        state.present.remove(&key);
        if let Some(pos) = state.queue.iter().position(|k| k == &key) {
            state.queue.remove(pos);
        }
    }

    /// Evict oldest records until within `policy.record_capacity`.
    async fn evict_overflow(&self, mirror: &IndradbBackend, policy: &CachePolicy) -> Result<()> {
        let capacity = policy.record_capacity;
        if capacity == 0 {
            return Ok(());
        }
        loop {
            let victim = {
                let mut state = self.order.lock().unwrap_or_else(|e| e.into_inner());
                if state.queue.len() <= capacity {
                    None
                } else {
                    let victim = state.queue.pop_front();
                    if let Some(ref key) = victim {
                        state.present.remove(key);
                    }
                    victim
                }
            };
            let Some((table, id)) = victim else {
                break;
            };
            let _ = mirror.delete_record(&table, &id).await;
            crate::telemetry::record_record_eviction(&table);
        }
        Ok(())
    }
}
