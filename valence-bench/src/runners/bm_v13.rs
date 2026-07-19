//! bm-v13: filter shape comparison at fixed prefill depth.

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use valence_core::CompiledQuery;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;
use crate::workload::prefill::prefill_table;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !crate::runners::store_available(ctx) {
        let reason = crate::runners::store_skip_reason(ctx).unwrap_or_default();
        let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
        report.status = "skipped";
        report.pass_notes = Some(reason);
        return Ok(report);
    }

    let depth = 10_000.min(ctx.sweep.prefill);
    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend: Arc<dyn valence_core::DatabaseBackend> = valence.active_backend()?;

    prefill_table(Arc::clone(&backend), "bm_v13", depth).await?;

    let queries = [
        ("select_all", "SELECT * FROM bm_v13 LIMIT 50"),
        ("select_id", "SELECT id FROM bm_v13 LIMIT 50"),
    ];

    let mut notes = Vec::new();
    let mut last_stats = MetricStats::empty();
    for (label, sql) in queries {
        let compiled = CompiledQuery::new(sql.into(), vec![]);
        let mut samples = Vec::with_capacity(ctx.plan.default_ops);
        for _ in 0..ctx.plan.default_ops {
            let start = Instant::now();
            backend.execute_compiled_query(&compiled).await?;
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let stats = MetricStats::summarize(samples);
        notes.push(format!("{label} p95 {:.3} ms", stats.p95));
        last_stats = stats;
    }

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix).with_sweep(&ctx.sweep);
    report.prefill_count = Some(depth);
    report.query_ms = Some(last_stats);
    report.pass_notes = Some(notes.join("; "));
    Ok(report)
}
