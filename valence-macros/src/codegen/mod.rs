//! Token generation for [`valence_schema!`](crate::valence_schema) and
//! [`valence_trait_schema!`](crate::valence_trait_schema).
//!
//! Keeping codegen out of `lib.rs` shrinks the proc-macro entry surface and
//! groups policy/schema emission so the DSL parser modules stay parse-only.

pub mod policies;
pub mod schema;
pub mod trait_schema;
