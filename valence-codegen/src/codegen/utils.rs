//! Small string helpers shared by parsers and metadata emission.

/// Map internal DSL field types (e.g. `record<user>`) to logical column type names for metadata.
pub fn map_field_type_to_string(field_type: &str) -> String {
    if field_type.starts_with("record<") && field_type.ends_with('>') {
        let table_name = field_type
            .strip_prefix("record<")
            .and_then(|s| s.strip_suffix(">"))
            .unwrap_or("");
        format!("record({table_name})")
    } else if field_type.starts_with("enum:") || field_type.starts_with("ext_enum:") {
        "string".to_string()
    } else {
        match field_type {
            "string" => "string".to_string(),
            "integer" => "integer".to_string(),
            "float" | "decimal" => "decimal".to_string(),
            "boolean" => "boolean".to_string(),
            "datetime" => "timestamptz".to_string(),
            "date" => "date".to_string(),
            "json" => "json".to_string(),
            "currency" => "json".to_string(),
            _ => {
                if field_type.starts_with("json_as:") {
                    "json".to_string()
                } else {
                    field_type.to_string()
                }
            }
        }
    }
}

/// Sanitize a DSL/type name fragment for use inside generated function identifiers.
pub fn sanitize_for_rust_ident(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// `snake_case` or table name → `PascalCase` for generated Rust identifiers.
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
