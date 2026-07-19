//! Redis key layout for Valence document storage.

/// Namespaced Redis keys for one Valence deployment.
#[derive(Debug, Clone)]
pub struct Keyspace {
    prefix: String,
}

impl Keyspace {
    /// Build a keyspace with the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }

    /// Document JSON blob: `{prefix}:doc:{table}:{id}`.
    pub fn doc(&self, table: &str, id: &str) -> String {
        format!("{}:doc:{table}:{id}", self.prefix)
    }

    /// Table membership set: `{prefix}:ids:{table}`.
    pub fn table_ids(&self, table: &str) -> String {
        format!("{}:ids:{table}", self.prefix)
    }

    /// Outgoing edge set: `{prefix}:edge:{edge}:{from_table}:{from_id}`.
    pub fn edge(&self, edge_table: &str, from_table: &str, from_id: &str) -> String {
        format!("{}:edge:{edge_table}:{from_table}:{from_id}", self.prefix)
    }

    /// Unique index slot: `{prefix}:uniq:{table}:{field}:{value}`.
    pub fn uniq(&self, table: &str, field: &str, value: &str) -> String {
        format!("{}:uniq:{table}:{field}:{value}", self.prefix)
    }

    /// Unique index field registry: `{prefix}:uniqidx:{table}`.
    pub fn uniq_index(&self, table: &str) -> String {
        format!("{}:uniqidx:{table}", self.prefix)
    }
}
