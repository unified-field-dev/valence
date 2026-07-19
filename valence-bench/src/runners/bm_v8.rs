//! bm-v8: merge/update throughput.

use std::time::Instant;

use anyhow::Result;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !crate::runners::store_available(ctx) {
        let reason = crate::runners::store_skip_reason(ctx).unwrap_or_default();
        let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
        report.status = "skipped";
        report.pass_notes = Some(reason);
        return Ok(report);
    }

    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend = valence.active_backend()?;

    backend
        .create_record(
            "bm_v8",
            serde_json::json!({"id": "m1", "score": 0, "name": "alpha"}),
        )
        .await?;

    let mut samples = Vec::with_capacity(ctx.plan.default_ops);
    for i in 0..ctx.plan.default_ops {
        let start = Instant::now();
        backend
            .merge_record("bm_v8", "m1", serde_json::json!({"score": i}))
            .await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(stats);
    report.pass_notes = Some(format!("merge p95 {:.3} ms", stats.p95));
    Ok(report)
}
