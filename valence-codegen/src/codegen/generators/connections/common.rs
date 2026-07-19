//! Shared helpers for connection codegen (`target` type path, etc.).

use syn::parse_str;
use valence_core::SchemaConnection;

use crate::codegen::utils::to_pascal_case;

pub(super) fn connection_target_type(conn: &SchemaConnection) -> syn::Type {
    let target_table_pascal = to_pascal_case(&conn.to_table);
    let target_type_path = conn
        .model_path
        .clone()
        .unwrap_or_else(|| format!("crate::generated::{target_table_pascal}"));
    parse_str(&target_type_path)
        .unwrap_or_else(|_| parse_str("crate::generated::Unknown").expect("fallback"))
}
