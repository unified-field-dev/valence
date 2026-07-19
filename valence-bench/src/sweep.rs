//! Sweep parameters shared across benchmark experiments.

/// CLI/env sweep dimensions for write and query tracks.
#[derive(Debug, Clone)]
pub struct SweepParams {
    /// Rows to insert before query benchmarks.
    pub prefill: usize,
    /// Optional multi-point prefill sweep (e.g. 1000,10000,100000).
    pub prefill_sweep: Option<Vec<usize>>,
    /// Sustained write duration for firehose experiments.
    pub duration_secs: u64,
    /// Concurrent writer tasks.
    pub concurrency: usize,
    /// Multi-process bench client count (bc multibench).
    pub bench_clients: usize,
    /// Timed query loop iterations.
    pub query_iters: usize,
    /// Privacy policy eval sleep (microseconds) for bm-v17.
    pub privacy_sleep_us: u64,
}

impl Default for SweepParams {
    fn default() -> Self {
        Self {
            prefill: 10_000,
            prefill_sweep: None,
            duration_secs: 30,
            concurrency: 64,
            bench_clients: 1,
            query_iters: 1000,
            privacy_sleep_us: 0,
        }
    }
}

impl SweepParams {
    /// Effective prefill depths for this run (single or sweep list).
    pub fn prefill_depths(&self) -> Vec<usize> {
        if let Some(ref sweep) = self.prefill_sweep {
            if sweep.is_empty() {
                vec![self.prefill]
            } else {
                sweep.clone()
            }
        } else {
            vec![self.prefill]
        }
    }

    /// Bench client index for bc multibench (`VALENCE_BENCH_CLIENT_INDEX`).
    pub fn client_index() -> usize {
        std::env::var("VALENCE_BENCH_CLIENT_INDEX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    }
}

/// Parse comma-separated prefill sweep from CLI.
pub fn parse_prefill_sweep(raw: &str) -> Option<Vec<usize>> {
    let vals: Vec<usize> = raw
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    if vals.is_empty() {
        None
    } else {
        Some(vals)
    }
}
