//! Execute hybrid M2M hop plans: source SQL → edge fan-out → target records.

use std::collections::HashSet;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use valence_backend_indradb::IndradbBackend;
use valence_core::compiled_query::CompiledQuery;
use valence_core::error::Result;
use valence_core::query::{apply_equality_where, apply_order_limit_offset};
use valence_core::record_id::RecordId;
use valence_core::DatabaseBackend;

use crate::cache_policy::CachePolicy;
use crate::edge_cache::EdgeCache;
use crate::record_cache::RecordCache;

/// JSON envelope emitted by [`valence_core::backend::HybridQueryCompiler`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridHopPlan {
    /// Marker object; presence selects the hybrid hop path.
    pub hybrid_hop: HybridHopBody,
}

/// Body of a hybrid many-to-many hop plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridHopBody {
    /// SQL that yields source rows (must expose an `id` field).
    pub source_sql: String,
    /// Bound parameters for `source_sql`.
    pub source_params: Vec<(String, Value)>,
    /// Source document table (for edge `from` ids).
    #[serde(default)]
    pub source_table: String,
    /// Edge table used for fan-out.
    pub edge_table: String,
    /// Target document table.
    pub target_table: String,
    /// Optional residual SQL used only for order/limit parsing on the adapter.
    #[serde(default)]
    pub residual_sql: Option<String>,
    /// Equality-filter params applied in-process after target fetch.
    #[serde(default)]
    pub residual_params: Vec<(String, Value)>,
}

/// Parse a hop-plan envelope from a compiled query string.
///
/// Returns `None` when the string is ordinary SQL.
pub fn parse_hop_plan(query_string: &str) -> Option<HybridHopPlan> {
    let value: Value = serde_json::from_str(query_string.trim()).ok()?;
    if value.get("hybrid_hop").is_none() {
        return None;
    }
    serde_json::from_value(value).ok()
}

/// Execute a hybrid hop plan against primary + mirror.
///
/// # Errors
///
/// Returns primary/mirror errors encountered while resolving the hop.
pub async fn execute_hop_plan(
    primary: &Arc<dyn DatabaseBackend>,
    mirror: &IndradbBackend,
    records: &RecordCache,
    edges: &EdgeCache,
    policy: &CachePolicy,
    plan: &HybridHopPlan,
) -> Result<Vec<Value>> {
    crate::telemetry::record_hop_plan();
    let body = &plan.hybrid_hop;
    let source_rows = primary
        .execute_compiled_query(&CompiledQuery::new(
            body.source_sql.clone(),
            body.source_params.clone(),
        ))
        .await?;
    let source_ids = extract_ids(&source_rows);

    let source_table = if body.source_table.is_empty() {
        guess_source_table(&body.source_sql)
    } else {
        body.source_table.clone()
    };
    let mut target_ids: HashSet<(String, String)> = HashSet::new();
    for source_id in source_ids {
        let from = RecordId::new(&source_table, source_id);
        let targets = resolve_edge_targets(primary, mirror, edges, policy, &from, &body.edge_table)
            .await?;
        for target in targets {
            if target.table() == body.target_table || body.target_table.is_empty() {
                target_ids.insert((target.table().to_string(), target.id().to_string()));
            }
        }
    }

    let mut rows = Vec::with_capacity(target_ids.len());
    for (table, id) in target_ids {
        if let Some(row) = get_target_record(primary, mirror, records, policy, &table, &id).await? {
            rows.push(row);
        }
    }

    if !body.residual_params.is_empty() {
        let compiled = CompiledQuery::new(
            body.residual_sql
                .clone()
                .unwrap_or_else(|| format!("SELECT * FROM {}", body.target_table)),
            body.residual_params.clone(),
        );
        rows = apply_equality_where(rows, &compiled);
    }
    if let Some(ref residual_sql) = body.residual_sql {
        rows = apply_order_limit_offset(rows, residual_sql);
    }
    Ok(rows)
}

/// Fan-out edge targets from the mirror when complete; otherwise from the primary.
async fn resolve_edge_targets(
    primary: &Arc<dyn DatabaseBackend>,
    mirror: &IndradbBackend,
    edges: &EdgeCache,
    policy: &CachePolicy,
    from: &RecordId,
    edge_table: &str,
) -> Result<Vec<RecordId>> {
    if policy.caches_edge(edge_table) && edges.is_complete(edge_table) {
        crate::telemetry::record_cache_hit("edge");
        return mirror.get_edge_targets(from, edge_table).await;
    }
    crate::telemetry::record_cache_miss("edge");
    if policy.caches_edge(edge_table) {
        crate::telemetry::record_edge_fallback(edge_table);
    }
    primary.get_edge_targets(from, edge_table).await
}

/// Point-get target via record cache, falling back to the primary.
async fn get_target_record(
    primary: &Arc<dyn DatabaseBackend>,
    mirror: &IndradbBackend,
    records: &RecordCache,
    policy: &CachePolicy,
    table: &str,
    id: &str,
) -> Result<Option<Value>> {
    if let Some(hit) = records.get(mirror, policy, table, id).await? {
        crate::telemetry::record_cache_hit("record");
        return Ok(Some(hit));
    }
    crate::telemetry::record_cache_miss("record");
    let row = primary.get_record(table, id).await?;
    if let Some(ref body) = row {
        let _ = records
            .put(mirror, policy, table, id, body.clone())
            .await;
    }
    Ok(row)
}

/// Extract bare ids from JSON rows.
fn extract_ids(rows: &[Value]) -> Vec<String> {
    rows.iter()
        .filter_map(|row| {
            if let Some(id) = row.as_str() {
                return Some(id.to_string());
            }
            row.get("id")
                .and_then(|v| v.get("id").and_then(|x| x.as_str()).or_else(|| v.as_str()))
                .map(str::to_string)
        })
        .collect()
}

/// Best-effort parse of the source table from a `FROM <table>` clause.
fn guess_source_table(sql: &str) -> String {
    let upper = sql.to_uppercase();
    let Some(idx) = upper.find(" FROM ") else {
        return "unknown".into();
    };
    sql[idx + 6..]
        .split_whitespace()
        .next()
        .unwrap_or("unknown")
        .trim_matches(|c| c == '`' || c == '"' || c == '\'')
        .to_string()
}

