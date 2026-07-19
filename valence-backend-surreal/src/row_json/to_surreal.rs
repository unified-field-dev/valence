//! Convert model JSON into Surreal write content.

use std::collections::BTreeMap;

use surrealdb::types::{Array, Number, Object, RecordId, Value};

fn split_wire_record_str(s: &str) -> Option<(&str, &str)> {
    let (table, id) = s.split_once(':')?;
    if table.is_empty() || id.is_empty() {
        return None;
    }
    if !table.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((table, id))
}

/// Convert model JSON into Surreal write content, turning `"table:id"` wire strings into native
/// [`RecordId`] values. Generated models serialize [`crate::RecordId`] fields as strings; v3
/// rejects or mis-coerces those for `record<…>` fields when left as plain strings.
pub fn json_to_surreal_content_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(Number::Int(i))
            } else if let Some(f) = n.as_f64() {
                Value::Number(Number::Float(f))
            } else {
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => {
            if let Some((table, id)) = split_wire_record_str(&s) {
                Value::RecordId(RecordId::new(table.to_string(), id.to_string()))
            } else {
                Value::String(s)
            }
        }
        serde_json::Value::Array(items) => Value::Array(Array::from(
            items
                .into_iter()
                .map(json_to_surreal_content_value)
                .collect::<Vec<_>>(),
        )),
        serde_json::Value::Object(map) => {
            let mut out = BTreeMap::new();
            for (k, val) in map {
                out.insert(k, json_to_surreal_content_value(val));
            }
            Value::Object(Object::from(out))
        }
    }
}
