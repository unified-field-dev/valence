//! Counter and gauge helpers for Valence instrumentation.

use std::sync::OnceLock;

use valence_telemetry::{try_log_event_value, try_record_counter, try_record_gauge};

use super::events;
use super::labels::{EdgeWriteOp, ReadOp, WriteOp};

fn db_wall_ms_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        matches!(
            std::env::var("VALENCE_DB_WALL_MS").as_deref(),
            Ok("1") | Ok("true") | Ok("TRUE")
        )
    })
}

fn db_wall_ms_sample_rate() -> f64 {
    static RATE: OnceLock<f64> = OnceLock::new();
    *RATE.get_or_init(|| {
        std::env::var("VALENCE_DB_WALL_MS_SAMPLE")
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0_f64)
            .clamp(0.0_f64, 1.0_f64)
    })
}

fn should_emit_db_wall_ms(table: &str, op: &str) -> bool {
    if db_wall_ms_enabled() {
        return true;
    }
    let rate = db_wall_ms_sample_rate();
    if rate <= 0.0 {
        return false;
    }
    if rate >= 1.0 {
        return true;
    }
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    (table, op).hash(&mut hasher);
    (hasher.finish() % 10_000) as f64 / 10_000.0 < rate
}

fn slow_op_threshold_ms() -> Option<f64> {
    static THRESHOLD: OnceLock<Option<f64>> = OnceLock::new();
    *THRESHOLD.get_or_init(|| {
        std::env::var("VALENCE_SLOW_OP_MS")
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|t| *t > 0.0)
    })
}

pub fn record_read(table: &str, telemetry_label: &str, op: ReadOp) {
    let op_s = op.as_str();
    try_record_counter(
        "valence_db_reads",
        &[
            ("table", table),
            ("database_type", telemetry_label),
            ("op", op_s),
        ],
        1,
    );
}

pub fn record_write(table: &str, telemetry_label: &str, op: WriteOp) {
    let op_s = op.as_str();
    try_record_counter(
        "valence_db_writes",
        &[
            ("table", table),
            ("database_type", telemetry_label),
            ("op", op_s),
        ],
        1,
    );
}

pub fn record_edge_read(edge_table: &str, telemetry_label: &str) {
    try_record_counter(
        "valence_edge_reads",
        &[
            ("edge_table", edge_table),
            ("database_type", telemetry_label),
        ],
        1,
    );
}

pub fn record_edge_write(edge_table: &str, telemetry_label: &str, op: EdgeWriteOp) {
    try_record_counter(
        "valence_edge_writes",
        &[
            ("edge_table", edge_table),
            ("database_type", telemetry_label),
            ("op", op.as_str()),
        ],
        1,
    );
}

pub fn record_db_error(operation: &str, telemetry_label: &str, message: &str) {
    try_record_counter(
        "valence_db_errors",
        &[("operation", operation), ("database_type", telemetry_label)],
        1,
    );
    try_log_event_value(
        "valence_error_log",
        &events::error_log_fields("database", operation, "", telemetry_label, message),
    );
}

pub fn record_db_wall_ms(table: &str, telemetry_label: &str, op: &str, wall_ms: f64) {
    if !should_emit_db_wall_ms(table, op) {
        return;
    }
    try_record_gauge(
        "valence_db_wall_ms",
        &[
            ("table", table),
            ("database_type", telemetry_label),
            ("op", op),
        ],
        wall_ms,
    );
}

pub fn maybe_record_slow_op(
    operation: &str,
    table: &str,
    op: &str,
    telemetry_label: &str,
    wall_ms: f64,
    record_id: Option<&str>,
) {
    let Some(threshold) = slow_op_threshold_ms() else {
        return;
    };
    if wall_ms < threshold {
        return;
    }
    try_log_event_value(
        "valence_slow_op",
        &events::slow_op_fields(
            operation,
            table,
            op,
            telemetry_label,
            wall_ms,
            record_id.unwrap_or(""),
        ),
    );
}

pub fn record_router_resolve_error(table: &str) {
    try_record_counter("valence_router_resolve_errors", &[("table", table)], 1);
}

pub fn record_query_rows_filtered(_table: &str, _reason: &str, _count: i64) {}

pub fn record_unique_violation(table: &str, field: &str) {
    try_record_counter(
        "valence_unique_violations",
        &[("table", table), ("field", field)],
        1,
    );
}

pub fn record_mutation_wall_ms(operation: &str, wall_ms: f64) {
    try_record_gauge(
        "valence_mutation_wall_ms",
        &[("operation", operation)],
        wall_ms,
    );
}

pub fn record_retry_error(operation: &str, database_type: &str, message: &str) {
    try_record_counter(
        "valence_db_retry_errors",
        &[
            ("operation", operation),
            ("database_type", database_type),
            ("message", message),
        ],
        1,
    );
}

pub fn record_ownership_fetch_mode(mode: &str) {
    try_record_counter("valence_ownership_fetch_mode", &[("mode", mode)], 1);
}

pub fn record_validation_failure(table: &str, field: &str, validator: &str) {
    try_record_counter(
        "valence_validation_failures",
        &[("table", table), ("field", field), ("validator", validator)],
        1,
    );
}

pub fn record_ownership_transfer(table: &str) {
    try_record_counter("valence_ownership_transfers", &[("table", table)], 1);
}
