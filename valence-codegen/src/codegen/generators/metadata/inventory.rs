//! Inventory registration for schema metadata and trait implementors.

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::codegen::schema::SchemaContext;

use super::collect::SchemaMetadataPieces;

pub(super) fn quote_schema_metadata_inventory(p: &SchemaMetadataPieces) -> TokenStream {
    let schema_struct_name = &p.schema_struct_name;
    quote! {
        #[cfg(not(target_family = "wasm"))]
        const _: () = {
            fn __valence_schema_metadata_init() -> &'static valence::SchemaMetadataStruct {
                #schema_struct_name::metadata()
            }
            valence::inventory::submit! {
                valence::SchemaMetadataInit(__valence_schema_metadata_init)
            }
        };
    }
}

pub(super) fn quote_trait_implementor_inventory(
    schema: &SchemaContext,
    table_name_lit: &LitStr,
) -> TokenStream {
    let submissions: Vec<TokenStream> = schema
        .traits
        .iter()
        .map(|trait_name| {
            let trait_name_lit = LitStr::new(trait_name, proc_macro2::Span::call_site());
            quote! {
                valence::inventory::submit! {
                    valence::TraitImplementor {
                        trait_name: #trait_name_lit,
                        table_name: #table_name_lit,
                    }
                }
            }
        })
        .collect();
    quote! {
        #[cfg(not(target_family = "wasm"))]
        const _: () = {
            #(#submissions)*
        };
    }
}

pub(super) fn quote_connections_overlay(p: &SchemaMetadataPieces) -> TokenStream {
    let schema_struct_name = &p.schema_struct_name;
    let table_name_lit = &p.table_name_lit;
    quote! {
        #[cfg(not(target_family = "wasm"))]
        const _: () = {
            fn __schema_connections_overlay() -> (
                &'static str,
                &'static [valence::SchemaConnection],
            ) {
                (
                    #table_name_lit,
                    #schema_struct_name::full().connections.as_slice(),
                )
            }
            valence::inventory::submit! {
                valence::SchemaConnectionsOverlayInit(__schema_connections_overlay)
            }
        };
    }
}
