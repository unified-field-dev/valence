//! SQL column references for JSON document rows (`id` + `body`).

/// Map a Valence field name to a SQL expression against document storage.
pub fn sql_doc_column(field: &str) -> String {
    if field == "id" {
        "id".to_string()
    } else {
        format!("json_extract(body, '$.{field}')")
    }
}

/// Rewrite `SELECT *` / field lists for document tables.
pub fn sql_select_clause(projection: Option<&Vec<String>>) -> String {
    match projection {
        None => "id, body".to_string(),
        Some(fields) if fields.len() == 1 && fields[0].trim() == "*" => "id, body".to_string(),
        Some(fields) => fields
            .iter()
            .map(|f| {
                let trimmed = f.trim();
                if trimmed.starts_with("VALUE ") {
                    let inner = trimmed.trim_start_matches("VALUE ").trim();
                    if inner == "id" {
                        "id".to_string()
                    } else {
                        sql_doc_column(inner)
                    }
                } else if trimmed == "id" {
                    "id".to_string()
                } else if trimmed == "*" {
                    "id, body".to_string()
                } else {
                    sql_doc_column(trimmed)
                }
            })
            .collect::<Vec<_>>()
            .join(", "),
    }
}
