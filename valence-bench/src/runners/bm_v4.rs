//! bm-v4: third-party acme-stub adapter throughput.

use std::time::Instant;

use anyhow::{bail, Result};
use valence_testkit::StorageAdapter;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !matches!(ctx.matrix.storage, StorageAdapter::AcmeStub) {
        bail!("bm-v4 requires --storage acme-stub");
    }

    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend = valence.active_backend()?;

    let mut samples = Vec::with_capacity(ctx.plan.default_ops);
    for i in 0..ctx.plan.default_ops {
        let id = format!("a{i}");
        let start = Instant::now();
        backend
            .create_record("bm_v4", serde_json::json!({"id": id}))
            .await?;
        backend.get_record("bm_v4", &id).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(stats);
    report.pass_notes = Some(format!("acme-stub p95 {:.3} ms", stats.p95));
    Ok(report)
}
