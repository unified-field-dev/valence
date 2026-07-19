//! Schema validation for FK/connection consistency.
//!
//! Ensures:
//! - Every Record (FK) field has a matching connection (for deletion and privacy)
//! - Every HasOne connection has a matching Record field on this table

use valence_core::{Schema, SchemaField};

fn is_record_field(field: &SchemaField) -> bool {
    field.fk.is_some()
}

/// Validate that FK fields and connections are consistent.
///
/// - **HasOne**: Each HasOne connection must have a corresponding Record field
///   (the FK lives on this table). Connection without FK = error.
/// - **FK without connection**: Every Record field must have a matching connection
///   so deletion (on_delete) and privacy can be applied correctly.
pub fn validate_connections_and_fields(schema: &Schema) -> Result<(), String> {
    let field_names: std::collections::HashSet<&str> =
        schema.fields.iter().map(|f| f.name.as_str()).collect();

    // Validate composite_key field names exist and don't include the primary key
    for ck_field in &schema.composite_key {
        if !field_names.contains(ck_field.as_str()) {
            return Err(format!(
                "composite_key references unknown field '{}'. \
                 Available fields: {:?}",
                ck_field,
                field_names.iter().collect::<Vec<_>>()
            ));
        }
        if schema
            .fields
            .iter()
            .any(|f| f.name == *ck_field && f.primary)
        {
            return Err(format!(
                "composite_key must not include the primary key field '{ck_field}'"
            ));
        }
    }

    let record_field_names: std::collections::HashSet<&str> = schema
        .fields
        .iter()
        .filter(|f| is_record_field(f))
        .map(|f| f.name.as_str())
        .collect();

    let connection_names: std::collections::HashSet<&str> = schema
        .connections
        .iter()
        .map(|c| c.from_field.as_str())
        .collect();

    // 1. HasOne connection without matching Record field → error
    for conn in &schema.connections {
        if conn.on_delete.trim().is_empty() {
            return Err(format!(
                "Connection '{}' is missing required on_delete (Cascade | SetNull | Restrict)",
                conn.from_field
            ));
        }
        if conn.target_trait.is_some() {
            let allowed = conn.cardinality == "HasOne"
                || conn.cardinality == "ManyToMany"
                || conn.cardinality == "HasMany";
            if !allowed {
                return Err(format!(
                    "Connection '{}' targets trait '{}' with unsupported cardinality '{}'. \
                     Trait-targeted connections support HasOne, HasMany, and ManyToMany.",
                    conn.from_field,
                    conn.target_trait.clone().unwrap_or_default(),
                    conn.cardinality
                ));
            }
            // Trait-targeted relationships cannot validate a concrete target table here.
            // Keep FK/connection consistency checks, but skip concrete-table validation.
        }

        if conn.cardinality == "HasOne" {
            if !record_field_names.contains(conn.from_field.as_str()) {
                return Err(format!(
                    "Connection '{}' has cardinality HasOne but no matching Record field. \
                     HasOne requires an FK on this table. Add '{}: {{ r#type: FieldType::Record(\"{}\"), required: true }}' to fields.",
                    conn.from_field, conn.from_field, conn.to_table
                ));
            }
        } else if conn.cardinality == "HasMany" {
            if conn.reverse_field.is_none() {
                return Err(format!(
                    "Connection '{}' has cardinality HasMany but missing reverse_field. \
                     HasMany requires reverse_field naming the FK on the target table.",
                    conn.from_field
                ));
            }
            if conn.target_trait.is_some() && conn.to_table.is_empty() {
                return Err(format!(
                    "Connection '{}' HasMany trait target requires to_table (trait:...)",
                    conn.from_field
                ));
            }
        } else if conn.cardinality == "ManyToMany" && conn.edge_table.is_none() {
            return Err(format!(
                "Connection '{}' has cardinality ManyToMany but missing edge_table. \
                     ManyToMany requires edge_table naming the SurrealDB edge table.",
                conn.from_field
            ));
        }
    }

    // 2. Record (FK) field without matching connection → error
    for field in &schema.fields {
        if is_record_field(field) && !connection_names.contains(field.name.as_str()) {
            let ref_table = field.fk.as_ref().map_or("?", |fk| fk.ref_table.as_str());
            return Err(format!(
                "Record field '{}' (FK to {}) has no matching connection. \
                     All FKs must have a connection for deletion and privacy. \
                     Add to connections: {}: {{ table: \"{}\", model: \"...\" }}",
                field.name, ref_table, field.name, ref_table
            ));
        }
    }

    Ok(())
}
