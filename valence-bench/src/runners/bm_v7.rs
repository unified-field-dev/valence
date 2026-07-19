//! bm-v7: multi-writer bc firehose (one process per client index).

use std::sync::Arc;

use anyhow::Result;

use crate::report::{BenchReport, WriteMetrics};
use crate::runners::RunContext;
use crate::sweep::SweepParams;
use crate::workload::firehose::run_write_firehose;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !crate::runners::store_available(ctx) {
        let reason = crate::runners::store_skip_reason(ctx).unwrap_or_default();
        let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
        report.status = "skipped";
        report.pass_notes = Some(reason);
        return Ok(report);
    }

    let client_index = SweepParams::client_index();
    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend: Arc<dyn valence_core::DatabaseBackend> = valence.active_backend()?;

    let table = format!("bm_v7_bc{client_index}");
    let fh = run_write_firehose(
        backend,
        &table,
        ctx.sweep.duration_secs,
        ctx.sweep.concurrency,
    )
    .await?;

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix).with_sweep(&ctx.sweep);
    report.scenario_id = Some(format!("client_{client_index}"));
    report.write = Some(WriteMetrics {
        achieved_write_ops_per_sec: fh.achieved_write_ops_per_sec,
        error_rate: fh.error_rate,
        total_ops: fh.total_ops,
        error_count: fh.error_count,
    });
    report.ops_per_sec = Some(fh.achieved_write_ops_per_sec);
    report.pass_notes = Some(format!(
        "bc client {client_index}: {:.1} ops/s (aggregate with scripts/bench-aggregate-bc.sh)",
        fh.achieved_write_ops_per_sec
    ));
    Ok(report)
}
