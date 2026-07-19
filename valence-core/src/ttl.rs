//! Table-level TTL policy hooks.

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaTtlPolicy {
    pub seconds: u64,
    pub mode: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendTtlCapability {
    SupportedNative,
    Deferred,
    Unsupported,
}

#[async_trait::async_trait]
pub trait BackendTtlAdapter: Send + Sync {
    fn capability(&self) -> BackendTtlCapability;
    async fn apply_table_policy(&self, table: &str, policy: &SchemaTtlPolicy) -> Result<()>;
}
