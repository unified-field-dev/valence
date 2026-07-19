use valence::prelude::*;

valence_schema! {
    Project {
        table: "project",
        version: "0.1.0",
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            name: { r#type: FieldType::String, required: true },
        ],
        connections: [
            tasks: {
                table: "task",
                cardinality: HasMany,
                reverse_field: "project",
                on_delete: Cascade,
                model: "crate::generated::Task",
            },
        ],
    }
}
