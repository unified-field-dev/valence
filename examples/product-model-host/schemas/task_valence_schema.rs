use valence::prelude::*;

valence_schema! {
    Task {
        table: "task",
        version: "0.1.0",
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            title: { r#type: FieldType::String, required: true },
            project: { r#type: FieldType::Record("project"), required: true },
        ],
        connections: [
            project: {
                table: "project",
                on_delete: Cascade,
                model: "crate::generated::Project",
            },
        ],
    }
}
