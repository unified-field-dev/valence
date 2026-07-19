//! Shared Rust type / predicate tokens for trait definition and per-schema trait impls.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use valence_core::SchemaField;

use crate::codegen::utils::to_pascal_case;

pub fn rust_type_for(field_type: &str) -> TokenStream {
    if field_type.starts_with("record<") && field_type.ends_with('>') {
        quote! { valence::RecordId }
    } else {
        match field_type {
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

/// Resolve the Rust type for a field, handling enum fields.
pub fn rust_type_for_field(field: &SchemaField, context_name: &str) -> TokenStream {
    if field.field_type.starts_with("enum:") || field.field_type.starts_with("ext_enum:") {
        if let Some(ref etype) = field.enum_type {
            let parsed: TokenStream = etype.parse().unwrap_or_else(|_| {
                let ident = format_ident!("{}", etype);
                quote! { #ident }
            });
            return parsed;
        }
        let enum_name = format!("{}{}", context_name, to_pascal_case(&field.name));
        let ident = format_ident!("{}", enum_name);
        quote! { #ident }
    } else {
        rust_type_for(&field.field_type)
    }
}

pub fn predicate_type_for(field_type: &str) -> TokenStream {
    if field_type.starts_with("record<") && field_type.ends_with('>') {
        quote! { valence::RecordPredicate }
    } else {
        match field_type {
            "integer" => quote! { valence::IntPredicate },
            "datetime" => quote! { valence::DateTimePredicate },
            _ => quote! { valence::StringPredicate },
        }
    }
}

pub fn where_core_call_for(field_type: &str) -> proc_macro2::Ident {
    if field_type.starts_with("record<") && field_type.ends_with('>') {
        format_ident!("where_record")
    } else {
        match field_type {
            "integer" => format_ident!("where_int"),
            "datetime" => format_ident!("where_datetime"),
            _ => format_ident!("where_string"),
        }
    }
}
