//! bm-v24: cross-backend hop depth-2 latency (mem→sqlite representative + pair slug).

use std::time::Instant;

use anyhow::Result;
use valence_testkit::{run_hop_pair_contract, HopPair, StorageAdapter};

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    let secondary = match ctx.matrix.storage {
        StorageAdapter::Mem => StorageAdapter::Sqlite,
        other => other,
    };
    let pair = HopPair {
        primary: StorageAdapter::Mem,
        secondary,
    };

    let mut samples = Vec::with_capacity(ctx.plan.default_ops.max(1));
    for _ in 0..ctx.plan.default_ops.max(1) {
        let start = Instant::now();
        run_hop_pair_contract(pair, Some(&ctx.wire)).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let stats = MetricStats::summarize(samples);
    let p95 = stats.p95;
    let ops = stats.count;
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ops);
    report.query_ms = Some(stats);
    report.scenario_id = Some(pair.slug());
    report.bench_topology = Some("aws".into());
    report.pass_notes = Some(format!("hop pair {} wall p95 {:.3} ms", pair.slug(), p95));
    Ok(report)
}
