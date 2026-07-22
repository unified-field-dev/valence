//! Maps schema field type strings to Rust token types for generated models.

use proc_macro2::TokenStream;
use valence_core::SchemaField;

use crate::codegen::generators::rust_types::rust_type_tokens;

pub(super) fn field_type_tokens_for(field: &SchemaField, model_name: &str) -> TokenStream {
    rust_type_tokens(field, model_name)
}
