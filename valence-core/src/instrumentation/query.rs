//! L4 query lifecycle telemetry stubs.

use crate::error::Error;
use crate::query::QueryCore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryTarget {
    Schema,
    TraitUnion,
    TraitHop,
}

impl QueryTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            QueryTarget::Schema => "schema",
            QueryTarget::TraitUnion => "trait_union",
            QueryTarget::TraitHop => "trait_hop",
        }
    }
}

pub fn classify_query_target(table: &str, hop_is_trait: bool) -> QueryTarget {
    if hop_is_trait {
        QueryTarget::TraitHop
    } else if table.contains(',') {
        QueryTarget::TraitUnion
    } else {
        QueryTarget::Schema
    }
}

pub fn resolve_trait_name(_table_csv: &str) -> String {
    String::new()
}

pub fn record_compile_error(_core: &QueryCore, _err: &Error) {}

pub fn record_deserialize_error(_table: &str, _err: &Error) {}

pub fn record_query_success(
    _core: &QueryCore,
    _target: QueryTarget,
    _trait_name: &str,
    _rows_db: i64,
    _rows_after_hop: i64,
    _rows_after_pending: i64,
    _wall_ms: i64,
) {
}
