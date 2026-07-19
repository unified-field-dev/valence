//! `SELECT VALUE` projection row decoding.

use surrealdb::types::Value;

use valence_core::error::{Error, Result};

use super::from_surreal::{
    record_map_to_json_object, try_value_as_record_id, try_value_as_record_map,
};

/// `true` when the statement is `SELECT VALUE …` (one projected cell per row, not full records).
pub fn is_select_value_projection_query(query: &str) -> bool {
    let mut words = query.split_whitespace();
    matches!(
        (words.next(), words.next()),
        (Some(a), Some(b)) if a.eq_ignore_ascii_case("select") && b.eq_ignore_ascii_case("value")
    )
}

fn projection_cell_to_json(cell: Value) -> Result<serde_json::Value> {
    match cell {
        Value::Object(o) => {
            let wrapped = Value::Object(o);
            if let Some(record) = try_value_as_record_id(&wrapped) {
                let wire = match Value::RecordId(record).into_json_value() {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                return Ok(serde_json::Value::String(wire));
            }
            let m = try_value_as_record_map(&wrapped).ok_or_else(|| {
                Error::Validation("Projection cell object could not decode as record map".into())
            })?;
            Ok(record_map_to_json_object(&m))
        }
        Value::Array(arr) => {
            let mut it = arr.into_inner().into_iter();
            let Some(only) = it.next() else {
                return Ok(serde_json::Value::Null);
            };
            if it.next().is_some() {
                return Err(Error::Validation(
                    "Unexpected multi-value array in SELECT VALUE projection row".into(),
                ));
            }
            projection_cell_to_json(only)
        }
        other => Ok(other.into_json_value()),
    }
}

/// Decode `SELECT VALUE …` rows without expanding bare record ids into full records.
pub fn decode_query_value_projection_rows_to_json(result: Value) -> Result<Vec<serde_json::Value>> {
    let rows = match result {
        Value::Array(arr) => arr.into_inner(),
        Value::None | Value::Null => vec![],
        other => vec![other],
    };
    rows.into_iter().map(projection_cell_to_json).collect()
}
