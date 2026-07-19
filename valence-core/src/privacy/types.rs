//! Privacy policy value types: operations, per-model policy bags, and sync [`PrivacyRule`] callbacks.
//!
//! Async evaluation for schema-registered rules lives in [`super::policy_evaluator::PolicyEvaluator`].

use crate::actor::Actor;

/// Which CRUD operation is being checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyOperation {
    Read,
    Create,
    Update,
    Delete,
}
/// Complete privacy policies for a model (all operation types)
#[derive(Debug, Clone)]
pub struct PrivacyPolicies {
    pub read: PrivacyPolicy,
    pub create: PrivacyPolicy,
    pub update: PrivacyPolicy,
    pub delete: PrivacyPolicy,
}

impl Default for PrivacyPolicies {
    fn default() -> Self {
        Self {
            read: PrivacyPolicy::default(),
            create: PrivacyPolicy::default(),
            update: PrivacyPolicy::default(),
            delete: PrivacyPolicy::default(),
        }
    }
}

/// Policy for a single operation type (read or write)
#[derive(Debug, Clone)]
pub struct PrivacyPolicy {
    pub always_allow: Vec<PrivacyRule>,
    pub allow: Vec<PrivacyRule>,
    pub block: Vec<PrivacyRule>,
    pub always_block: Vec<PrivacyRule>,
}

impl Default for PrivacyPolicy {
    fn default() -> Self {
        Self {
            always_allow: Vec::new(),
            allow: Vec::new(),
            block: Vec::new(),
            always_block: Vec::new(),
        }
    }
}

/// A single privacy rule
#[derive(Clone)]
pub struct PrivacyRule {
    pub name: &'static str,
    pub description: Option<&'static str>,
    /// Function that evaluates the rule: (record, viewer) -> bool
    /// Record is the full record data as JSON
    /// Returns true if the rule allows access, false otherwise
    pub check: fn(record: &serde_json::Value, viewer: &Actor) -> bool,
}

impl std::fmt::Debug for PrivacyRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrivacyRule")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}
