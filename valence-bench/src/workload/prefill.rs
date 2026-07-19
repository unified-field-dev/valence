//! Bulk row insertion before query benchmarks.

use std::sync::Arc;

use anyhow::Result;
use valence_core::DatabaseBackend;

const BATCH: usize = 1000;

/// Insert `count` schemaless rows into `table` via the adapter.
pub async fn prefill_table(
    backend: Arc<dyn DatabaseBackend>,
    table: &str,
    count: usize,
) -> Result<usize> {
    backend.ensure_schemaless_table(table).await?;
    let mut inserted = 0usize;
    while inserted < count {
        let end = (inserted + BATCH).min(count);
        for i in inserted..end {
            backend
                .create_record(
                    table,
                    serde_json::json!({
                        "id": format!("prefill-{i}"),
                        "idx": i,
                        "label": format!("row-{i}"),
                    }),
                )
                .await?;
        }
        inserted = end;
    }
    Ok(count)
}
