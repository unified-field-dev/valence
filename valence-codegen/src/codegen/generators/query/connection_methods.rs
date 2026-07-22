//! Connection-based `where_*_has_results`, `where_*_contains`, and `query_*` hop methods.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::LitStr;

use crate::codegen::schema::SchemaContext;

use super::target::{
    emits_typed_connection_hop, resolve_target_query_type, resolve_target_query_type_with_lifetime,
};

pub(super) fn collect_connection_hop_methods(
    schema: &SchemaContext,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut connection_methods = Vec::new();
    let mut hop_methods = Vec::new();

    let field_names: Vec<&str> = schema.fields.iter().map(|f| f.name.as_str()).collect();

    for conn in &schema.schema.connections {
        if conn.cardinality == "HasOne" {
            push_has_one(
                conn,
                &field_names,
                &mut connection_methods,
                &mut hop_methods,
            );
        } else if conn.cardinality == "HasMany" {
            push_has_many(conn, &mut connection_methods, &mut hop_methods);
        } else if conn.cardinality == "ManyToMany" {
            push_many_to_many(conn, &mut connection_methods, &mut hop_methods);
        }
    }

    (connection_methods, hop_methods)
}

fn push_has_one(
    conn: &valence_core::SchemaConnection,
    field_names: &[&str],
    connection_methods: &mut Vec<TokenStream>,
    hop_methods: &mut Vec<TokenStream>,
) {
    if conn.target_trait.is_some() {
        push_has_one_trait_target(conn, field_names, hop_methods);
        return;
    }
    if !field_names.contains(&conn.from_field.as_str()) {
        return;
    }
    let from_field = &conn.from_field;
    let to_table = &conn.to_table;
    let method_name = format_ident!("where_{}_has_results", from_field);
    let from_field_lit = LitStr::new(from_field, proc_macro2::Span::call_site());
    let to_table_lit = LitStr::new(to_table, proc_macro2::Span::call_site());
    connection_methods.push(quote! {
        /// Filter where the connected `#from_field` record matches the subquery.
        pub fn #method_name<F>(mut self, f: F) -> Self
        where
            F: FnOnce(valence::QueryCore) -> valence::QueryCore,
        {
            let base = valence::QueryCore::new(#to_table_lit.to_string());
            let sub = f(base);
            self.inner = self.inner.where_connection_exists(
                #from_field_lit.to_string(),
                #to_table_lit.to_string(),
                sub,
            );
            self
        }
    });

    if emits_typed_connection_hop(conn) {
        let hop_method_name = format_ident!("query_{}", from_field);
        let target_query_type = resolve_target_query_type(conn);
        let target_query_type_lt = resolve_target_query_type_with_lifetime(conn);
        hop_methods.push(quote! {
            /// Hop to the connected `#from_field` record (HasOne).
            /// Switches the result type from this model to the target model.
            pub fn #hop_method_name(self) -> #target_query_type_lt {
                let valence = self.valence;
                let source = self.inner;
                let mut target = valence::QueryCore::new(#to_table_lit.to_string());
                target.hop_source = Some(valence::HopSource {
                    source_query: Box::new(source),
                    hop_type: valence::HopType::HasOneForward {
                        fk_field: #from_field_lit.to_string(),
                    },
                });
                #target_query_type::from_parts(target, valence)
            }
        });
    }
}

fn push_has_many(
    conn: &valence_core::SchemaConnection,
    connection_methods: &mut Vec<TokenStream>,
    hop_methods: &mut Vec<TokenStream>,
) {
    if conn.target_trait.is_some() {
        push_has_many_trait_target(conn, hop_methods);
        return;
    }
    let Some(reverse_field) = conn.reverse_field.as_ref() else {
        return;
    };
    let conn_name = &conn.from_field;
    let _conn_name_ident = format_ident!("{}", conn_name);
    let to_table = &conn.to_table;
    let method_name = format_ident!("where_{}_has_results", conn_name);
    let to_table_lit = LitStr::new(to_table, proc_macro2::Span::call_site());
    let reverse_field_lit = LitStr::new(reverse_field, proc_macro2::Span::call_site());
    connection_methods.push(quote! {
        /// Filter where the `#conn_name_ident` connection (HasMany) has records matching the subquery.
        pub fn #method_name<F>(mut self, f: F) -> Self
        where
            F: FnOnce(valence::QueryCore) -> valence::QueryCore,
        {
            let base = valence::QueryCore::new(#to_table_lit.to_string());
            let sub = f(base);
            self.inner = self.inner.where_connection_exists_reverse(
                #to_table_lit.to_string(),
                #reverse_field_lit.to_string(),
                sub,
            );
            self
        }
    });

    if emits_typed_connection_hop(conn) {
        let hop_method_name = format_ident!("query_{}", conn_name);
        let target_query_type = resolve_target_query_type(conn);
        let target_query_type_lt = resolve_target_query_type_with_lifetime(conn);
        hop_methods.push(quote! {
            /// Hop to the connected `#conn_name` records (HasMany).
            /// Switches the result type from this model to the target model.
            pub fn #hop_method_name(self) -> #target_query_type_lt {
                let valence = self.valence;
                let source = self.inner;
                let mut target = valence::QueryCore::new(#to_table_lit.to_string());
                target.hop_source = Some(valence::HopSource {
                    source_query: Box::new(source),
                    hop_type: valence::HopType::HasManyForward {
                        reverse_field: #reverse_field_lit.to_string(),
                    },
                });
                #target_query_type::from_parts(target, valence)
            }
        });
    }
}

fn push_many_to_many(
    conn: &valence_core::SchemaConnection,
    connection_methods: &mut Vec<TokenStream>,
    hop_methods: &mut Vec<TokenStream>,
) {
    let Some(edge_table) = conn.edge_table.as_ref() else {
        return;
    };
    let conn_name = &conn.from_field;
    let _conn_name_ident = format_ident!("{}", conn_name);
    let edge_table_lit = LitStr::new(edge_table, proc_macro2::Span::call_site());

    if conn.target_trait.is_some() {
        let contains_method = format_ident!("where_{}_contains", conn_name);
        connection_methods.push(quote! {
            /// Filter where the `#conn_name_ident` connection (ManyToMany trait target)
            /// contains a specific target record id.
            pub fn #contains_method(mut self, target: valence::RecordId) -> Self {
                self.inner = self.inner.where_connection_contains_many_to_many(
                    #edge_table_lit.to_string(),
                    target,
                );
                self
            }
        });
        return;
    }

    let to_table = &conn.to_table;
    let method_name = format_ident!("where_{}_has_results", conn_name);
    let to_table_lit = LitStr::new(to_table, proc_macro2::Span::call_site());
    connection_methods.push(quote! {
        /// Filter where the `#conn_name_ident` connection (ManyToMany) has records matching the subquery.
        pub fn #method_name<F>(mut self, f: F) -> Self
        where
            F: FnOnce(valence::QueryCore) -> valence::QueryCore,
        {
            let base = valence::QueryCore::new(#to_table_lit.to_string());
            let sub = f(base);
            self.inner = self.inner.where_connection_exists_many_to_many(
                #edge_table_lit.to_string(),
                #to_table_lit.to_string(),
                sub,
            );
            self
        }
    });

    if emits_typed_connection_hop(conn) {
        let hop_method_name = format_ident!("query_{}", conn_name);
        let target_query_type = resolve_target_query_type(conn);
        let target_query_type_lt = resolve_target_query_type_with_lifetime(conn);
        hop_methods.push(quote! {
            /// Hop to the connected `#conn_name` records (ManyToMany).
            /// Switches the result type from this model to the target model.
            pub fn #hop_method_name(self) -> #target_query_type_lt {
                let valence = self.valence;
                let source = self.inner;
                let mut target = valence::QueryCore::new(#to_table_lit.to_string());
                target.hop_source = Some(valence::HopSource {
                    source_query: Box::new(source),
                    hop_type: valence::HopType::ManyToManyForward {
                        edge_table: #edge_table_lit.to_string(),
                    },
                });
                #target_query_type::from_parts(target, valence)
            }
        });
    }
}

fn push_has_one_trait_target(
    conn: &valence_core::SchemaConnection,
    field_names: &[&str],
    hop_methods: &mut Vec<TokenStream>,
) {
    if !field_names.contains(&conn.from_field.as_str()) {
        return;
    }
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

fn push_has_many_trait_target(
    conn: &valence_core::SchemaConnection,
    hop_methods: &mut Vec<TokenStream>,
) {
    let Some(reverse_field) = conn.reverse_field.as_ref() else {
        return;
    };
    let target_trait = conn.target_trait.as_deref().unwrap_or_default();
    let conn_name = &conn.from_field;
    let hop_method_name = format_ident!("query_{}", conn_name);
    let target_query_all = format_ident!("{}QueryAll", target_trait);
    let reverse_field_lit = LitStr::new(reverse_field, proc_macro2::Span::call_site());
    let target_trait_lit = LitStr::new(target_trait, proc_macro2::Span::call_site());

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
