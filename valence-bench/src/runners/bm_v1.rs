//! bm-v1: side-effect-free compiled query fan-out (mem-focused).

use std::time::Instant;

use anyhow::{bail, Result};
use valence_core::compiled_query::CompiledQuery;
use valence_testkit::StorageAdapter;

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    if !matches!(ctx.matrix.storage, StorageAdapter::Mem) {
        bail!("bm-v1 defaults to mem storage");
    }

    let mut session = crate::runners::bootstrap_session(ctx);
    session.spawn().await?;
    let valence = session.ensure_valence()?;
    let backend = valence.active_backend()?;
    let compiled = CompiledQuery::new("SELECT * FROM bm_v1 LIMIT 10".into(), vec![]);

    for _ in 0..ctx.warmup {
        backend.execute_compiled_query(&compiled).await?;
    }

    let mut samples = Vec::with_capacity(ctx.plan.default_ops);
    for _ in 0..ctx.plan.default_ops {
        let start = Instant::now();
        backend.execute_compiled_query(&compiled).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ctx.plan.default_ops);
    report.op_ms = Some(stats);
    report.pass_notes = Some(format!("compiled query p95 {:.3} ms", stats.p95));
    Ok(report)
}
