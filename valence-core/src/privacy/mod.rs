//! Field-level privacy evaluation for generated models and admin queries.
//!
//! [`PrivacyEvaluator`] applies schema policy rules; policy constants live in [`policies`].
//!
//! Privacy is **schema-driven**, not a [`crate::ValenceBuilder`] port:
//!
//! - App crates `impl` [`PolicyEvaluator`] and export `pub const MY_RULE`
//! - Schemas reference those consts in `policies: { … }`
//! - Built-ins (`AUTHENTICATED`, `PUBLIC_READ`, …) live in [`crate::privacy_policies`]
//!
//! There is **no** `ValenceBuilder::register_policy(...)`.
mod policy_evaluator;
mod types;

pub use policy_evaluator::PolicyEvaluator;
pub use types::{PrivacyOperation, PrivacyPolicies, PrivacyPolicy, PrivacyRule};

mod evaluator;

pub use evaluator::PrivacyEvaluator;

pub mod policies;
