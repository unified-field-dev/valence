use valence::prelude::*;

valence_trait_schema! {
    HistorySource {
        fields: [
            record_history_table: { r#type: FieldType::String, required: true },
        ],
    }
}
