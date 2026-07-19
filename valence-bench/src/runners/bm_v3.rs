//! bm-v3: Surreal embedded CRUD wall time.

use std::time::Instant;

use anyhow::{bail, Result};
use valence_testkit::StorageAdapter;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !matches!(
        ctx.matrix.storage,
        StorageAdapter::SurrealMem | StorageAdapter::SurrealRocksdb
    ) {
        bail!("bm-v3 requires surreal-mem or surreal-rocksdb storage");
    }

    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend = valence.active_backend()?;

    for i in 0..ctx.warmup {
        let id = format!("warm{i}");
        backend
            .create_record("bm_v3", serde_json::json!({"id": id}))
            .await?;
    }

    let mut samples = Vec::with_capacity(ctx.plan.default_ops);
    for i in 0..ctx.plan.default_ops {
        let id = format!("s{i}");
        let start = Instant::now();
        backend
            .create_record("bm_v3", serde_json::json!({"id": id, "n": i}))
            .await?;
        backend.get_record("bm_v3", &id).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(stats);
    report.pass_notes = Some(format!("surreal crud p95 {:.3} ms", stats.p95));
    Ok(report)
}
