//! bm-v19: load rows + in-process map vs resource sample.

use std::sync::Arc;

use anyhow::Result;
use valence_core::CompiledQuery;

use crate::report::BenchReport;
use crate::resource::sample_resource;
use crate::runners::RunContext;
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

    let before = sample_resource();
    prefill_table(Arc::clone(&backend), "bm_v19", depth).await?;
    let compiled = CompiledQuery::new("SELECT * FROM bm_v19 LIMIT 10000".into(), vec![]);
    let rows = backend.execute_compiled_query(&compiled).await?;
    let mapped: Vec<_> = rows
        .iter()
        .filter_map(|r| r.get("label").and_then(|v| v.as_str()))
        .collect();
    let after = sample_resource();

    let mut report = BenchReport::base(&ctx.plan.id, &ctx.matrix).with_sweep(&ctx.sweep);
    report.prefill_count = Some(depth);
    report.resource = after;
    let rss_delta = match (before, after) {
        (Some(b), Some(a)) => a.rss_kb.saturating_sub(b.rss_kb),
        _ => 0,
    };
    report.pass_notes = Some(format!(
        "mapped {} labels; rss delta {} kb",
        mapped.len(),
        rss_delta
    ));
    Ok(report)
}
