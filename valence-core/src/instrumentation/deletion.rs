//! Deletion instrumentation stubs.

pub fn record_dag_computed(
    _table: &str,
    _id: &str,
    _nodes: usize,
    _max_depth: usize,
    _restrict_violations: usize,
    _skipped: usize,
) {
}

pub fn record_restrict_blocked(_table: &str, _id: &str, _conn: &str, _count: usize) {}

pub fn record_run_queued(_table: &str, _id: &str, _nodes: usize, _max_depth: usize) {}
