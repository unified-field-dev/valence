//! Shared Rust type / predicate tokens for trait definition and per-schema trait impls.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use valence_core::SchemaField;

use crate::codegen::generators::rust_types::{parse_json_as, rust_type_tokens};

/// Resolve the Rust type for a field, handling enum / JsonAs fields.
pub fn rust_type_for_field(field: &SchemaField, context_name: &str) -> TokenStream {
    rust_type_tokens(field, context_name)
}

pub fn predicate_type_for(field_type: &str) -> TokenStream {
    if parse_json_as(field_type).is_some() {
        return quote! { valence::StringPredicate };
    }
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
