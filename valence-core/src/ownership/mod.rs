//! First-class data ownership (platform tables + hooks from generated CRUD).

mod resolver;
mod service;

pub use resolver::OwnerResolver;
pub use service::{
    ensure_lookup_indexes, normalize_record_id_for_ownership, owner_ref_from_ownership_json,
    ownership_colocate_enabled, ownership_get_join_enabled, ownership_unified_fetch_enabled,
    parse_owner_kind, skip_ownership_for_table, OwnerDataSummary, OwnerSchemaRowCount,
    OwnershipGateStatus, OwnershipService, RecordOwnershipBundle, SKIP_OWNERSHIP_TABLES,
};
