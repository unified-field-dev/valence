//! bm-v2: instrumentation overhead (recording telemetry vs off).

use std::sync::Arc;
use std::time::Instant;

use anyhow::{bail, Result};
use valence_backend_mem::InMemoryBackend;
use valence_core::instrumentation::wrap_backend;
use valence_testkit::{StorageAdapter, TelemetryAdapter};

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !matches!(ctx.matrix.storage, StorageAdapter::Mem) {
        bail!("bm-v2 defaults to mem storage");
    }
    if !matches!(ctx.matrix.telemetry, TelemetryAdapter::Recording) {
        bail!("bm-v2 requires --telemetry recording");
    }

    let bare: Arc<dyn valence_core::DatabaseBackend> = Arc::new(InMemoryBackend::new());
    let wrapped = wrap_backend(Arc::new(InMemoryBackend::new()));

    let mut bare_samples = Vec::with_capacity(ctx.plan.default_ops);
    let mut wrapped_samples = Vec::with_capacity(ctx.plan.default_ops);

    for i in 0..ctx.plan.default_ops {
        let id = format!("b{i}");
        let payload = serde_json::json!({"id": id});

        let start = Instant::now();
        bare.create_record("bm_v2", payload.clone()).await?;
        bare.get_record("bm_v2", &id).await?;
        bare_samples.push(start.elapsed().as_secs_f64() * 1000.0);

        let start = Instant::now();
        wrapped.create_record("bm_v2", payload).await?;
        wrapped.get_record("bm_v2", &id).await?;
        wrapped_samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let bare_stats = MetricStats::summarize(bare_samples);
    let wrapped_stats = MetricStats::summarize(wrapped_samples);
    let overhead = wrapped_stats.p50 - bare_stats.p50;

    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(wrapped_stats);
    report.pass_notes = Some(format!(
        "instrumentation overhead p50 {:.3} ms (bare {:.3}, wrapped {:.3})",
        overhead, bare_stats.p50, wrapped_stats.p50
    ));
    Ok(report)
}
