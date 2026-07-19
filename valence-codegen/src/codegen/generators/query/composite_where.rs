//! `where_<a>_and_<b>` combined predicate for composite primary keys on the query builder.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::LitStr;

use crate::codegen::schema::SchemaContext;

fn composite_key_query_param_type(field_type_str: &str, is_required: bool) -> TokenStream {
    let base = if field_type_str.starts_with("record<") && field_type_str.ends_with('>') {
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
    };

    if is_required {
        base
    } else {
        quote! { Option<#base> }
    }
}

fn composite_key_predicate_stmt(
    field_name_str: &str,
    field_type_str: &str,
    is_required: bool,
    param_ident: &proc_macro2::Ident,
) -> TokenStream {
    let field_name_lit = LitStr::new(field_name_str, proc_macro2::Span::call_site());

    let is_record = field_type_str.starts_with("record<") && field_type_str.ends_with('>');

    if is_required {
        if is_record {
            quote! {
                self.inner = self.inner.where_record(
                    #field_name_lit.to_string(),
                    valence::RecordPredicate::Equals(#param_ident),
                );
            }
        } else {
            match field_type_str {
                "integer" => quote! {
                    self.inner = self.inner.where_int(
                        #field_name_lit.to_string(),
                        valence::IntPredicate::Equals(#param_ident),
                    );
                },
                "datetime" => quote! {
                    self.inner = self.inner.where_datetime(
                        #field_name_lit.to_string(),
                        valence::DateTimePredicate::Equals(#param_ident),
                    );
                },
                _ => quote! {
                    self.inner = self.inner.where_string(
                        #field_name_lit.to_string(),
                        valence::StringPredicate::Equals(#param_ident),
                    );
                },
            }
        }
    } else if is_record {
        quote! {
            match #param_ident {
                Some(v) => {
                    self.inner = self.inner.where_record(
                        #field_name_lit.to_string(),
                        valence::RecordPredicate::Equals(v),
                    );
                }
                None => {
                    self.inner = self.inner.where_null(
                        #field_name_lit.to_string(),
                        valence::NullPredicate::IsNone,
                    );
                }
            }
        }
    } else if field_type_str == "integer" {
        quote! {
            match #param_ident {
                Some(v) => {
                    self.inner = self.inner.where_int(
                        #field_name_lit.to_string(),
                        valence::IntPredicate::Equals(v),
                    );
                }
                None => {
                    self.inner = self.inner.where_null(
                        #field_name_lit.to_string(),
                        valence::NullPredicate::IsNone,
                    );
                }
            }
        }
    } else {
        quote! {
            match #param_ident {
                Some(v) => {
                    self.inner = self.inner.where_string(
                        #field_name_lit.to_string(),
                        valence::StringPredicate::Equals(v),
                    );
                }
                None => {
                    self.inner = self.inner.where_null(
                        #field_name_lit.to_string(),
                        valence::NullPredicate::IsNone,
                    );
                }
            }
        }
    }
}

pub(super) fn generate_composite_key_where(
    schema: &SchemaContext,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    if schema.composite_key.is_empty() {
        return Ok(TokenStream::new());
    }

    let method_name_str = schema
        .composite_key
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join("_and_");
    let method_name = format_ident!("where_{}", method_name_str);

    let mut params = Vec::new();
    let mut stmts = Vec::new();

    for ck_name in &schema.composite_key {
        let field = schema
            .fields
            .iter()
            .find(|f| f.name == *ck_name)
            .unwrap_or_else(|| panic!("composite_key field '{ck_name}' not found"));

        let param_ident = format_ident!("{}", ck_name);
        let param_type = composite_key_query_param_type(&field.field_type, !field.nullable);
        params.push(quote! { #param_ident: #param_type });

        stmts.push(composite_key_predicate_stmt(
            ck_name,
            &field.field_type,
            !field.nullable,
            &param_ident,
        ));
    }

    Ok(quote! {
        /// Filter by all composite key fields at once.
        pub fn #method_name(mut self, #(#params),*) -> Self {
            #(#stmts)*
            self
        }
    })
}
