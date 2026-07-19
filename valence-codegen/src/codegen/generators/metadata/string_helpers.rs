//! String literal helpers for schema metadata emission.

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

pub(super) fn string_names_to_vec_code(names: &[String]) -> TokenStream {
    if names.is_empty() {
        return quote! { Vec::new() };
    }
    let lits: Vec<LitStr> = names
        .iter()
        .map(|t| LitStr::new(t, proc_macro2::Span::call_site()))
        .collect();
    quote! { vec![#(#lits.to_string()),*] }
}

pub(super) fn optional_string_lit_code(opt: &Option<String>) -> TokenStream {
    if let Some(p) = opt {
        let lit = LitStr::new(p, proc_macro2::Span::call_site());
        quote! { Some(#lit.to_string()) }
    } else {
        quote! { None }
    }
}

pub(super) fn humanize_field_edge_label(field_name_str: &str) -> String {
    field_name_str
        .strip_suffix("_id")
        .unwrap_or(field_name_str)
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
