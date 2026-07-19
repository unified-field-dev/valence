//! bm-v9: soft-delete queue path (`queue_delete_entity`).

use std::time::Instant;

use anyhow::Result;
use valence_core::admin_entity_delete::queue_delete_entity;

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

    let mut samples = Vec::with_capacity(ctx.plan.default_ops);
    for i in 0..ctx.plan.default_ops {
        let id = format!("del{i}");
        backend
            .create_record(
                "bm_v9_smoke",
                serde_json::json!({"id": id, "name": "to-delete"}),
            )
            .await?;
        let start = Instant::now();
        queue_delete_entity("bm_v9_smoke", &id, valence).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(stats);
    report.pass_notes = Some(format!(
        "soft-delete queue p95 {:.3} ms (not cascade execution)",
        stats.p95
    ));
    Ok(report)
}
