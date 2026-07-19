//! Optional async owner resolution for schemas using `ownership: { resolve: ... }`.

use async_trait::async_trait;
use serde_json::Value;

use crate::actor::Actor;
use crate::owner_ref::OwnerRef;
use crate::runtime::Valence;
use crate::Result;

/// Application-provided owner resolution (runs on create / upsert-create path).
///
/// Implementors should be zero-sized types with a [`Default`] impl (same pattern as [`crate::SideEffect`]).
#[async_trait]
pub trait OwnerResolver: Send + Sync + Default + 'static {
    /// Choose the [`OwnerRef`] for a row about to be created.
    async fn resolve_owner(
        &self,
        record: &Value,
        actor: &Actor,
        valence: &Valence,
    ) -> Result<OwnerRef>;
}
