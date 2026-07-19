//! bm-v18: telemetry recording vs off (extends bm-v2).

use std::sync::Arc;
use std::time::Instant;

use anyhow::{bail, Result};
use valence_backend_mem::InMemoryBackend;
use valence_core::instrumentation::wrap_backend;
use valence_testkit::{BootstrapSession, StorageAdapter, TelemetryAdapter};

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !matches!(ctx.matrix.storage, StorageAdapter::Mem) {
        bail!("bm-v18 defaults to mem storage");
    }

    let bare: Arc<dyn valence_core::DatabaseBackend> = Arc::new(InMemoryBackend::new());
    let wrapped = wrap_backend(Arc::new(InMemoryBackend::new()));

    let mut bare_samples = Vec::with_capacity(ctx.plan.default_ops);
    let mut wrapped_samples = Vec::with_capacity(ctx.plan.default_ops);

    for i in 0..ctx.plan.default_ops {
        let id = format!("t{i}");
        let payload = serde_json::json!({"id": id});

        let start = Instant::now();
        bare.create_record("bm_v18", payload.clone()).await?;
        bare.get_record("bm_v18", &id).await?;
        bare_samples.push(start.elapsed().as_secs_f64() * 1000.0);

        let start = Instant::now();
        wrapped.create_record("bm_v18", payload).await?;
        wrapped.get_record("bm_v18", &id).await?;
        wrapped_samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let bare_stats = MetricStats::summarize(bare_samples);
    let wrapped_stats = MetricStats::summarize(wrapped_samples);

    let mut matrix = ctx.matrix;
    matrix.telemetry = TelemetryAdapter::Recording;
    let mut session = BootstrapSession::new(matrix).with_wire_options(ctx.wire.clone());
    let _ = session.spawn().await;

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(wrapped_stats);
    report.pass_notes = Some(format!(
        "telemetry p50 delta {:.3} ms (bare {:.3}, wrapped {:.3})",
        wrapped_stats.p50 - bare_stats.p50,
        bare_stats.p50,
        wrapped_stats.p50
    ));
    Ok(report)
}
