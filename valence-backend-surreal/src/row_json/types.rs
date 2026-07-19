//! SurrealDB dynamic value types for row JSON conversion.

use serde::de::{EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;

/// Max chained `SELECT * FROM $rid` refetches when a query row is a bare record id.
pub const MAX_QUERY_ROW_FETCH_DEPTH: u8 = 8;

/// Dynamic value that can deserialize SurrealDB's internal data model, including enum inputs.
#[derive(Debug, Clone)]
pub enum SurrealAny {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Seq(Vec<SurrealAny>),
    Map(BTreeMap<String, SurrealAny>),
}

impl<'de> Deserialize<'de> for SurrealAny {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SurrealAnyVisitor;

        impl<'de> Visitor<'de> for SurrealAnyVisitor {
            type Value = SurrealAny;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any SurrealDB-compatible value")
            }

            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::Null)
            }

            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::Null)
            }

            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::Bool(v))
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::I64(v))
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::U64(v))
            }

            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::F64(v))
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::String(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SurrealAny::String(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut out = Vec::new();
                while let Some(v) = seq.next_element()? {
                    out.push(v);
                }
                Ok(SurrealAny::Seq(out))
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut out = BTreeMap::new();
                while let Some((k, v)) = map.next_entry()? {
                    out.insert(k, v);
                }
                Ok(SurrealAny::Map(out))
            }

            fn visit_enum<A>(self, data: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, access) = data.variant::<String>()?;
                match variant.as_str() {
                    "Null" | "None" => Ok(SurrealAny::Null),
                    "Bool" => {
                        let v: bool = access.newtype_variant()?;
                        Ok(SurrealAny::Bool(v))
                    }
                    "String" | "Strand" => {
                        let v: String = access.newtype_variant()?;
                        Ok(SurrealAny::String(v))
                    }
                    "I64" | "Int" => {
                        let v: i64 = access.newtype_variant()?;
                        Ok(SurrealAny::I64(v))
                    }
                    "U64" => {
                        let v: u64 = access.newtype_variant()?;
                        Ok(SurrealAny::U64(v))
                    }
                    "F64" | "Float" => {
                        let v: f64 = access.newtype_variant()?;
                        Ok(SurrealAny::F64(v))
                    }
                    "Seq" | "Array" => {
                        let v: Vec<SurrealAny> = access.newtype_variant()?;
                        Ok(SurrealAny::Seq(v))
                    }
                    "Map" | "Object" => {
                        let v: BTreeMap<String, SurrealAny> = access.newtype_variant()?;
                        Ok(SurrealAny::Map(v))
                    }
                    other => {
                        let _ = access.newtype_variant::<serde::de::IgnoredAny>()?;
                        Err(serde::de::Error::custom(format!(
                            "unsupported SurrealAny enum variant {other:?}"
                        )))
                    }
                }
            }
        }

        deserializer.deserialize_any(SurrealAnyVisitor)
    }
}
