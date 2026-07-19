//! bm-v25: nested hop chain depth-3/4 (inner-query exists track).

use std::time::Instant;

use anyhow::Result;
use valence_testkit::{
    hop_quads_representative, hop_triples_representative, run_hop_chain_contract,
    run_hop_quad_contract,
};

use crate::report::BenchReport;
use crate::runners::RunContext;
use crate::stats::MetricStats;

pub async fn run(ctx: &RunContext) -> Result<BenchReport> {
    let triple = hop_triples_representative()
        .into_iter()
        .next()
        .expect("at least one triple");
    let quad = hop_quads_representative()
        .into_iter()
        .next()
        .expect("at least one quad");

    let mut samples = Vec::new();
    for _ in 0..ctx.plan.default_ops.max(1) {
        let start = Instant::now();
        run_hop_chain_contract(triple, Some(&ctx.wire)).await?;
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    let start = Instant::now();
    run_hop_quad_contract(quad, Some(&ctx.wire)).await?;
    let quad_ms = start.elapsed().as_secs_f64() * 1000.0;

    let stats = MetricStats::summarize(samples);
    let p95 = stats.p95;
    let ops = stats.count;
    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix);
    report.ops = Some(ops);
    report.query_ms = Some(stats);
    report.scenario_id = Some(format!("triple={};quad={}", triple.slug(), quad.slug()));
    report.bench_topology = Some("aws".into());
    report.pass_notes = Some(format!(
        "depth3 {} p95 {:.3} ms; depth4 {} wall {:.3} ms",
        triple.slug(),
        p95,
        quad.slug(),
        quad_ms
    ));
    Ok(report)
}
