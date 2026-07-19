//! Setter-time validation snippets merged into mutable builder methods.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

fn validation_tokens_for_rule(field_ident: &Ident, validation: &str) -> TokenStream {
    match validation {
        "email" => quote! {
            valence::validation::validate_email(&#field_ident)
                .map_err(valence::Error::Validation)?;
        },
        "phone" => quote! {
            valence::validation::validate_phone(&#field_ident)
                .map_err(valence::Error::Validation)?;
        },
        "non_empty" => quote! {
            valence::validation::validate_non_empty(&#field_ident)
                .map_err(valence::Error::Validation)?;
        },
        "non_negative" => quote! {
            valence::validation::validate_non_negative(&#field_ident)
                .map_err(valence::Error::Validation)?;
        },
        "positive" => quote! {
            valence::validation::validate_positive(&#field_ident)
                .map_err(valence::Error::Validation)?;
        },
        v if v.starts_with("min_length:") => {
            let len: usize = v
                .strip_prefix("min_length:")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            quote! {
                valence::validation::validate_min_length(&#field_ident, #len)
                    .map_err(valence::Error::Validation)?;
            }
        }
        v if v.starts_with("max_length:") => {
            let len: usize = v
                .strip_prefix("max_length:")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            quote! {
                valence::validation::validate_max_length(&#field_ident, #len)
                    .map_err(valence::Error::Validation)?;
            }
        }
        v if v.starts_with("min:") => {
            let min: i64 = v
                .strip_prefix("min:")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            quote! {
                valence::validation::validate_min(&#field_ident, #min)
                    .map_err(valence::Error::Validation)?;
            }
        }
        v if v.starts_with("max:") => {
            let max: i64 = v
                .strip_prefix("max:")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            quote! {
                valence::validation::validate_max(&#field_ident, #max)
                    .map_err(valence::Error::Validation)?;
            }
        }
        v if v.starts_with("range:") => range_validation_tokens(field_ident, v),
        v if v.starts_with("enum:") => enum_validation_tokens(field_ident, v),
        v if v.starts_with("pattern:") => {
            let pattern = v.strip_prefix("pattern:").unwrap_or("");
            let pattern_lit = LitStr::new(pattern, proc_macro2::Span::call_site());
            quote! {
                valence::validation::validate_pattern(&#field_ident, #pattern_lit)
                    .map_err(valence::Error::Validation)?;
            }
        }
        v if v.starts_with("fn:") => {
            let fn_name = v.strip_prefix("fn:").unwrap_or("");
            let fn_ident = format_ident!("{}", fn_name);
            quote! {
                #fn_ident(&#field_ident)
                    .map_err(valence::Error::Validation)?;
            }
        }
        _ => TokenStream::new(),
    }
}

fn range_validation_tokens(field_ident: &Ident, v: &str) -> TokenStream {
    let parts: Vec<&str> = v.strip_prefix("range:").unwrap_or("").split(':').collect();
    if parts.len() != 2 {
        return TokenStream::new();
    }
    let (Ok(min), Ok(max)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) else {
        return TokenStream::new();
    };
    quote! {
        valence::validation::validate_range(&#field_ident, #min, #max)
            .map_err(valence::Error::Validation)?;
    }
}

fn enum_validation_tokens(field_ident: &Ident, v: &str) -> TokenStream {
    let values: Vec<&str> = v.strip_prefix("enum:").unwrap_or("").split(',').collect();
    let values_lit: Vec<LitStr> = values
        .iter()
        .map(|s| LitStr::new(s, proc_macro2::Span::call_site()))
        .collect();
    quote! {
        valence::validation::validate_enum(&#field_ident, &[#(#values_lit),*])
            .map_err(valence::Error::Validation)?;
    }
}

pub fn generate_validation_code(field_name: &str, validations: &[String]) -> TokenStream {
    let field_ident = format_ident!("{}", field_name);
    let mut validation_checks = Vec::new();

    for validation in validations {
        let validation_code = validation_tokens_for_rule(&field_ident, validation.as_str());
        if !validation_code.is_empty() {
            validation_checks.push(validation_code);
        }
    }

    quote! {
        #(#validation_checks)*
    }
}
