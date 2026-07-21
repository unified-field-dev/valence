//! Builder-facing cache selection rules and the resolved hot-path policy.

use std::collections::HashSet;

use valence_core::error::{Error, Result};

/// Default maximum number of cached record bodies.
pub const DEFAULT_RECORD_CAPACITY: usize = 10_000;

/// Default maximum number of cached graph edges.
pub const DEFAULT_EDGE_CAPACITY: usize = 100_000;

/// Whether tables are cached by default, plus include/exclude overrides.
///
/// # Examples
///
/// Cache everything except audit tables:
///
/// ```
/// use valence_backend_hybrid::CacheRules;
///
/// let rules = CacheRules::cache_all().exclude(["audit_log"]);
/// assert!(rules.allows("project"));
/// assert!(!rules.allows("audit_log"));
/// ```
///
/// Cache only explicit edge tables:
///
/// ```
/// use valence_backend_hybrid::CacheRules;
///
/// let rules = CacheRules::cache_none().include(["project_members"]);
/// assert!(rules.allows("project_members"));
/// assert!(!rules.allows("other_edge"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheRules {
    default_cache: bool,
    include: HashSet<String>,
    exclude: HashSet<String>,
}

impl CacheRules {
    /// Cache all tables unless listed in [`Self::exclude`].
    #[must_use]
    pub fn cache_all() -> Self {
        Self {
            default_cache: true,
            include: HashSet::new(),
            exclude: HashSet::new(),
        }
    }

    /// Cache no tables unless listed in [`Self::include`].
    #[must_use]
    pub fn cache_none() -> Self {
        Self {
            default_cache: false,
            include: HashSet::new(),
            exclude: HashSet::new(),
        }
    }

    /// Add tables that must be cached (overrides default `cache_none`).
    #[must_use]
    pub fn include(mut self, tables: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.include.extend(tables.into_iter().map(Into::into));
        self
    }

    /// Add tables that must not be cached (overrides default `cache_all`).
    #[must_use]
    pub fn exclude(mut self, tables: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude.extend(tables.into_iter().map(Into::into));
        self
    }

    /// Whether `table` is eligible for caching under these rules.
    #[must_use]
    pub fn allows(&self, table: &str) -> bool {
        if self.exclude.contains(table) {
            return false;
        }
        if self.include.contains(table) {
            return true;
        }
        self.default_cache
    }

    /// Validate that include/exclude sets do not contradict each other.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Internal`] when the same table appears in both sets.
    pub fn validate(&self) -> Result<()> {
        for table in &self.include {
            if self.exclude.contains(table) {
                return Err(Error::Internal(format!(
                    "cache rules conflict: `{table}` is both included and excluded"
                )));
            }
        }
        Ok(())
    }
}

impl Default for CacheRules {
    fn default() -> Self {
        Self::cache_all()
    }
}

/// Resolved cache policy used on hot paths (built once at [`crate::HybridBackendBuilder::build`]).
#[derive(Debug, Clone)]
pub struct CachePolicy {
    /// Maximum cached record bodies (`0` disables record caching).
    pub record_capacity: usize,
    /// Maximum cached edges (`0` disables edge caching).
    pub edge_capacity: usize,
    /// Record table selection rules.
    pub record_rules: CacheRules,
    /// Edge table selection rules.
    pub edge_rules: CacheRules,
}

impl CachePolicy {
    /// Build a validated policy from builder inputs.
    ///
    /// # Errors
    ///
    /// Returns an error when record or edge rules are contradictory.
    pub fn new(
        record_capacity: usize,
        edge_capacity: usize,
        record_rules: CacheRules,
        edge_rules: CacheRules,
    ) -> Result<Self> {
        record_rules.validate()?;
        edge_rules.validate()?;
        Ok(Self {
            record_capacity,
            edge_capacity,
            record_rules,
            edge_rules,
        })
    }

    /// Whether record caching is enabled and `table` is eligible.
    #[must_use]
    pub fn caches_record(&self, table: &str) -> bool {
        self.record_capacity > 0 && self.record_rules.allows(table)
    }

    /// Whether edge caching is enabled and `edge_table` is eligible.
    #[must_use]
    pub fn caches_edge(&self, edge_table: &str) -> bool {
        self.edge_capacity > 0 && self.edge_rules.allows(edge_table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_exclude_precedence() {
        let rules = CacheRules::cache_all()
            .include(["keep"])
            .exclude(["drop", "keep"]);
        assert!(rules.validate().is_err());
    }

    #[test]
    fn zero_capacity_disables_class() {
        let policy = CachePolicy::new(0, 0, CacheRules::cache_all(), CacheRules::cache_all())
            .expect("policy");
        assert!(!policy.caches_record("t"));
        assert!(!policy.caches_edge("e"));
    }
}
