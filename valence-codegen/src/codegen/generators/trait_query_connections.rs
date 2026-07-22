//! Connection hop methods on trait `*QueryAll` builders.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::LitStr;
use valence_core::SchemaConnection;

pub fn collect_trait_query_all_hop_methods(
    trait_name: &str,
    connections: &[SchemaConnection],
) -> Vec<TokenStream> {
    let mut hop_methods = Vec::new();
    let query_all_struct = format_ident!("{}QueryAll", trait_name);

    for conn in connections {
        match conn.cardinality.as_str() {
            "HasOne" if conn.target_trait.is_some() => {
                push_has_one_hop(conn, &query_all_struct, &mut hop_methods);
            }
            "HasMany" if conn.target_trait.is_some() => {
                push_has_many_hop(conn, &query_all_struct, &mut hop_methods);
            }
            _ => {}
        }
    }

    hop_methods
}

fn push_has_one_hop(
    conn: &SchemaConnection,
    _query_all_struct: &proc_macro2::Ident,
    hop_methods: &mut Vec<TokenStream>,
) {
    let target_trait = conn.target_trait.as_deref().unwrap_or_default();
    let from_field = &conn.from_field;
    let hop_method_name = format_ident!("query_{}", from_field);
    let target_query_all = format_ident!("{}QueryAll", target_trait);
    let from_field_lit = LitStr::new(from_field, proc_macro2::Span::call_site());
    let target_trait_lit = LitStr::new(target_trait, proc_macro2::Span::call_site());

    hop_methods.push(quote! {
        /// Hop to the connected `#from_field` trait-target (HasOne).
        pub fn #hop_method_name(self) -> #target_query_all<'a> {
            let valence = self.valence;
            let source = self.inner;
            let tables = valence::TraitRegistry::global().tables_for_trait(#target_trait_lit);
            let table_csv = tables.join(", ");
            let mut target = valence::QueryCore::new(table_csv);
            target.hop_source = Some(valence::HopSource {
                source_query: Box::new(source),
                hop_type: valence::HopType::HasOneForward {
                    fk_field: #from_field_lit.to_string(),
                },
            });
            #target_query_all::from_parts(target, valence)
        }
    });
}

fn push_has_many_hop(
    conn: &SchemaConnection,
    query_all_struct: &proc_macro2::Ident,
    hop_methods: &mut Vec<TokenStream>,
) {
    let Some(reverse_field) = conn.reverse_field.as_deref() else {
        return;
    };
    let target_trait = conn.target_trait.as_deref().unwrap_or_default();
    let conn_name = &conn.from_field;
    let hop_method_name = format_ident!("query_{}", conn_name);
    let target_query_all = format_ident!("{}QueryAll", target_trait);
    let reverse_field_lit = LitStr::new(reverse_field, proc_macro2::Span::call_site());
    let target_trait_lit = LitStr::new(target_trait, proc_macro2::Span::call_site());
    let _ = query_all_struct;

    hop_methods.push(quote! {
        /// Hop to the connected `#conn_name` trait-target records (HasMany).
        pub fn #hop_method_name(self) -> #target_query_all<'a> {
            let valence = self.valence;
            let source = self.inner;
            let parent_table = source.table.clone();
            let tables = valence::TraitRegistry::global().tables_for_trait(#target_trait_lit);
            let table_csv = tables.join(", ");
            let mut target = valence::QueryCore::new(table_csv);
            target = target.where_connection_exists(
                #reverse_field_lit.to_string(),
                parent_table,
                source,
            );
            #target_query_all::from_parts(target, valence)
        }
    });
}
