//! Process resource sampling helpers.

use serde::Serialize;

/// Optional CPU/RSS snapshot from `/proc/self/stat`.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ResourceMetrics {
    pub user_cpu_ms: f64,
    pub rss_kb: u64,
}

/// Read user CPU time (ms) and RSS (KB) for the current process.
pub fn sample_resource() -> Option<ResourceMetrics> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    let parts: Vec<&str> = stat.split_whitespace().collect();
    if parts.len() < 24 {
        return None;
    }
    let utime: u64 = parts[13].parse().ok()?;
    let rss_pages: u64 = parts[23].parse().ok()?;
    let page_size = 4096u64;
    Some(ResourceMetrics {
        user_cpu_ms: utime as f64 * 10.0,
        rss_kb: rss_pages * page_size / 1024,
    })
}
