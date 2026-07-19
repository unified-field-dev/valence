//! [`PrivacyEvaluator`] — synchronous policy lists and async entity checks.

mod aggregation;
mod rules;

/// Privacy evaluation engine
pub struct PrivacyEvaluator;

#[cfg(test)]
mod tests {
    use super::super::{PolicyEvaluator, PrivacyEvaluator, PrivacyPolicy, PrivacyRule};
    include!("../evaluator_tests_part1.rs");
    include!("../evaluator_tests_part2.rs");
}
