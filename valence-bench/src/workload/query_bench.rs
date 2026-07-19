//! Timed compiled-query loops after prefill.

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use valence_core::{CompiledQuery, DatabaseBackend};

use crate::stats::MetricStats;

/// Run a compiled query loop and return latency samples.
pub async fn run_query_loop(
    backend: Arc<dyn DatabaseBackend>,
    compiled: &CompiledQuery,
    iters: usize,
    warmup: usize,
) -> Result<MetricStats> {
    for _ in 0..warmup {
        backend.execute_compiled_query(compiled).await?;
    }
    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let start = Instant::now();
        backend.execute_compiled_query(compiled).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    Ok(MetricStats::summarize(samples))
}
