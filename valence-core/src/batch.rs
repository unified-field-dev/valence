//! Batch create hooks for codegen `BatchCreatable` impls (full batch runtime is host-owned).

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Result;
use crate::record_id::RecordId;
use crate::runtime::Valence;

/// Marker trait for models participating in batch create codegen.
#[async_trait]
pub trait BatchCreatable: Serialize + DeserializeOwned + Send + 'static {
    fn table_name() -> &'static str;

    fn set_id(&mut self, id: RecordId);

    async fn ensure_ownership_after_batch_create(
        _created_row: serde_json::Value,
        _valence: &Valence,
    ) -> Result<()> {
        Ok(())
    }
}
