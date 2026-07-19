//! Maps schema field type strings to Rust token types for generated models.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use valence_core::SchemaField;

use crate::codegen::utils::to_pascal_case;

pub(super) fn field_type_tokens(field_type_str: &str) -> TokenStream {
    if field_type_str.starts_with("record<") && field_type_str.ends_with('>') {
        quote! { valence::RecordId }
    } else {
        match field_type_str {
            "string" => quote! { String },
            "integer" => quote! { i64 },
            "float" => quote! { f64 },
            "boolean" => quote! { bool },
            "datetime" => quote! { chrono::DateTime<chrono::Utc> },
            "json" => quote! { serde_json::Value },
            _ => quote! { String },
        }
    }
}

pub(super) fn field_type_tokens_for(field: &SchemaField, model_name: &str) -> TokenStream {
    let ft = field.field_type.as_str();
    if ft.starts_with("enum:") || ft.starts_with("ext_enum:") {
        if let Some(ref etype) = field.enum_type {
            return etype.parse().unwrap_or_else(|_| {
                let ident = format_ident!("{}", etype);
                quote! { #ident }
            });
        }
        if ft.starts_with("enum:") {
            let enum_name = format!("{}{}", model_name, to_pascal_case(&field.name));
            let ident = format_ident!("{}", enum_name);
            return quote! { #ident };
        }
    }
    field_type_tokens(ft)
}
