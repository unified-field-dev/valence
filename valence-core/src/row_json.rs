//! JSON row helpers shared by query fetch paths.

/// Strip Surreal thing decorations to a bare id segment.
pub fn thing_to_id_only(mut s: String) -> String {
    s = s.split(':').next_back().unwrap_or(&s).to_string();
    s = s
        .replace(['⟩', '⟨', '›', '‹', '»', '«'], "")
        .trim()
        .to_string();
    s
}

/// Coerce a string or bare `id` field into the `{ table, id }` shape generated models expect.
pub fn normalize_record_id_field(table: &str, value: &mut serde_json::Value) {
    let Some(obj) = value.as_object_mut() else {
        return;
    };
    let Some(id_val) = obj.get("id").cloned() else {
        return;
    };
    if let Some(id_str) = id_val.as_str() {
        let bare = thing_to_id_only(id_str.to_string());
        obj.insert(
            "id".into(),
            serde_json::json!({ "table": table, "id": bare }),
        );
    }
}

/// JSON object form of [`crate::RecordId`] for adapter write responses.
pub fn record_id_json(table: &str, id: &str) -> serde_json::Value {
    serde_json::json!({
        "table": table,
        "id": id,
    })
}
