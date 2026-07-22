//! Serde helpers for [`FieldType::JsonAs`](crate::FieldType::JsonAs) fields.
//!
//! Codegen emits thin `serialize_with` / `deserialize_with` wrappers that call these
//! helpers with table, field, and type-path context.

use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::schema_types::JsonAsSerdeError;

fn context_message(table: &str, field: &str, type_path: &str, cause: &str) -> String {
    format!("JsonAs serde failed for {table}.{field} as {type_path}: {cause}")
}

/// Deserialize `T` with table/field/type context according to `mode`.
pub fn deserialize<'de, T, D>(
    deserializer: D,
    table: &'static str,
    field: &'static str,
    type_path: &'static str,
    mode: JsonAsSerdeError,
) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    match T::deserialize(deserializer) {
        Ok(v) => Ok(v),
        Err(e) => {
            let msg = context_message(table, field, type_path, &e.to_string());
            match mode {
                JsonAsSerdeError::Error => Err(serde::de::Error::custom(msg)),
                JsonAsSerdeError::Panic => {
                    panic!(
                        "{msg} (JsonAsSerdeError::Panic: trusted stored JSON must match {type_path})"
                    )
                }
            }
        }
    }
}

/// Serialize `T` with table/field/type context according to `mode`.
pub fn serialize<T, S>(
    value: &T,
    serializer: S,
    table: &'static str,
    field: &'static str,
    type_path: &'static str,
    mode: JsonAsSerdeError,
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    match value.serialize(serializer) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            let msg = context_message(table, field, type_path, &e.to_string());
            match mode {
                JsonAsSerdeError::Error => Err(serde::ser::Error::custom(msg)),
                JsonAsSerdeError::Panic => {
                    panic!(
                        "{msg} (JsonAsSerdeError::Panic: Serialize for {type_path} must not fail)"
                    )
                }
            }
        }
    }
}

/// Format a serialization error for CRUD boundaries that map `serde_json` failures.
pub fn format_serialization_context(
    table: &str,
    field: &str,
    type_path: &str,
    cause: &str,
) -> String {
    context_message(table, field, type_path, cause)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Payload {
        n: i64,
    }

    #[test]
    fn deserialize_error_includes_context() {
        let err = serde_json::from_value::<PayloadWrapperError>(json!({"payload": "nope"}))
            .expect_err("bad json");
        let msg = err.to_string();
        assert!(msg.contains("widget.payload"), "{msg}");
        assert!(msg.contains("crate::Payload"), "{msg}");
    }

    #[test]
    #[should_panic(expected = "JsonAsSerdeError::Panic")]
    fn deserialize_panic_mode() {
        let _ = serde_json::from_value::<PayloadWrapperPanic>(json!({"payload": "nope"}));
    }

    #[derive(Debug, Deserialize)]
    struct PayloadWrapperError {
        #[serde(deserialize_with = "deser_error")]
        #[allow(dead_code)]
        payload: Payload,
    }

    #[derive(Debug, Deserialize)]
    struct PayloadWrapperPanic {
        #[serde(deserialize_with = "deser_panic")]
        #[allow(dead_code)]
        payload: Payload,
    }

    fn deser_error<'de, D: Deserializer<'de>>(d: D) -> Result<Payload, D::Error> {
        deserialize(
            d,
            "widget",
            "payload",
            "crate::Payload",
            JsonAsSerdeError::Error,
        )
    }

    fn deser_panic<'de, D: Deserializer<'de>>(d: D) -> Result<Payload, D::Error> {
        deserialize(
            d,
            "widget",
            "payload",
            "crate::Payload",
            JsonAsSerdeError::Panic,
        )
    }
}
