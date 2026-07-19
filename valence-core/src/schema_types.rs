//! DSL type vocabulary used by `valence_schema!` and `valence_trait_schema!`.

/// Field data types accepted by the Valence DSL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    String,
    Integer,
    Decimal,
    Boolean,
    DateTime,
    Date,
    Json,
    Record(&'static str),
    Enum(&'static [&'static str]),
    ExternalEnum(&'static str),
}

impl FieldType {
    pub fn as_str(&self) -> String {
        match self {
            FieldType::String => "string".to_string(),
            FieldType::Integer => "integer".to_string(),
            FieldType::Decimal => "decimal".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::Json => "json".to_string(),
            FieldType::Record(table) => format!("record<{table}>"),
            FieldType::Enum(_) | FieldType::ExternalEnum(_) => "string".to_string(),
        }
    }
}

/// Built-in validators attachable to schema or trait fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Validator {
    Email,
    Phone,
    Url,
    NonEmpty,
    MinLength(usize),
    MaxLength(usize),
    Pattern(&'static str),
    Enum(&'static [&'static str]),
    Positive,
    NonNegative,
    Min(i64),
    Max(i64),
    Range(i64, i64),
    Custom(&'static str),
}

impl Validator {
    pub fn to_toml_string(&self) -> String {
        match self {
            Validator::Email => "email".to_string(),
            Validator::Phone => "phone".to_string(),
            Validator::Url => "url".to_string(),
            Validator::NonEmpty => "non_empty".to_string(),
            Validator::MinLength(n) => format!("min_length:{n}"),
            Validator::MaxLength(n) => format!("max_length:{n}"),
            Validator::Pattern(p) => format!("pattern:{p}"),
            Validator::Enum(values) => {
                let joined = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("enum:{joined}")
            }
            Validator::Positive => "positive".to_string(),
            Validator::NonNegative => "non_negative".to_string(),
            Validator::Min(n) => format!("min:{n}"),
            Validator::Max(n) => format!("max:{n}"),
            Validator::Range(min, max) => format!("range:{min},{max}"),
            Validator::Custom(name) => format!("fn:{name}"),
        }
    }
}

/// Role markers used by older privacy/mutation DSL shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Public,
    User,
    Admin,
    System,
    Owner,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Public => "public",
            Role::User => "user",
            Role::Admin => "admin",
            Role::System => "system",
            Role::Owner => "owner",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FieldType;

    #[test]
    fn field_type_record_as_str() {
        assert_eq!(FieldType::Record("user").as_str(), "record<user>");
    }
}
