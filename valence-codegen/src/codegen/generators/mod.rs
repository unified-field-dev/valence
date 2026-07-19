//! Procedural fragments merged per schema: struct, connections, CRUD, query, metadata, traits.
//!
//! Each submodule owns one concern; the parent `codegen` module calls them in dependency order.

mod connections;
mod crud;
pub mod enums;
mod iters;
mod metadata;
mod query;
mod side_effects;
mod structs;
mod trait_definition_quote;
mod trait_helpers;
mod trait_query_connections;
mod traits;
mod validation;

pub use connections::generate_connections;
pub use crud::generate_crud_operations;
pub use iters::generate_iters;
pub use metadata::generate_schema_metadata_method;
pub use query::generate_query_builder;
pub use side_effects::generate_side_effects;
pub use structs::generate_struct;
pub use trait_definition_quote::generate_trait_definition;
pub use traits::generate_trait_impls;
