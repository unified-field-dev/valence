//! L2 privacy evaluation telemetry stubs.

use crate::actor::Actor;
use crate::privacy::PrivacyOperation;

pub fn record_policy_evaluation(
    _policy: &str,
    _actor: &Actor,
    _operation: PrivacyOperation,
    _table: &str,
    _rule_phase: &str,
    _matched: bool,
    _check_outcome: &str,
) {
}

pub fn record_privacy_denial(_table: &str, _policy: &str, _telemetry_label: &str, _message: &str) {}

pub fn record_field_redactions(_table: &str, _hidden_fields: &[String]) {}
