//! bm-v11: compiled query latency after prefill depth sweep.

use std::sync::Arc;

use anyhow::Result;
use valence_core::CompiledQuery;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::workload::prefill::prefill_table;
use crate::workload::query_bench::run_query_loop;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !crate::runners::store_available(ctx) {
        let reason = crate::runners::store_skip_reason(ctx).unwrap_or_default();
        let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
        report.status = "skipped";
        report.pass_notes = Some(reason);
        return Ok(report);
    }

    let depth = ctx.sweep.prefill;
    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend: Arc<dyn valence_core::DatabaseBackend> = valence.active_backend()?;

    prefill_table(Arc::clone(&backend), "bm_v11", depth).await?;
    let compiled = CompiledQuery::new(format!("SELECT * FROM bm_v11 LIMIT 100"), vec![]);
    let stats = run_query_loop(backend, &compiled, ctx.sweep.query_iters, ctx.warmup).await?;

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix).with_sweep(&ctx.sweep);
    report.prefill_count = Some(depth);
    report.query_ms = Some(stats);
    report.pass_notes = Some(format!(
        "compiled query @ prefill={depth} p95 {:.3} ms",
        stats.p95
    ));
    Ok(report)
}
