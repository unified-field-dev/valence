//! SurrealDB ↔ Valence error boundary (keeps `surrealdb` out of `valence-core`).

use valence_core::error::Error;

pub fn db_err(e: surrealdb::Error) -> Error {
    Error::Database(e.to_string())
}
