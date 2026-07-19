//! Common reusable privacy policies for Valence schemas.

mod checks;
pub mod definitions;

pub use definitions::{common, helpers, owner};

#[cfg(test)]
mod tests {
    use super::definitions::{common, owner};
    use crate::actor::Actor;
    use crate::privacy::PrivacyEvaluator;
    use serde_json::json;

    #[test]
    fn test_public_read_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![common::PUBLIC_READ],
            block: vec![],
            always_block: vec![],
        };

        let anon_actor = Actor::Anonymous;
        let user_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };

        let record = json!({});

        assert!(PrivacyEvaluator::evaluate(&policy, &record, &anon_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &user_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &system_actor).is_ok());
    }

    #[test]
    fn test_authenticated_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![common::AUTHENTICATED],
            block: vec![],
            always_block: vec![],
        };

        let anon_actor = Actor::Anonymous;
        let user_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };

        let record = json!({});

        assert!(PrivacyEvaluator::evaluate(&policy, &record, &anon_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &user_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &system_actor).is_ok());
    }

    #[test]
    fn test_system_only_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![common::SYSTEM_ONLY],
            block: vec![],
            always_block: vec![],
        };

        let anon_actor = Actor::Anonymous;
        let user_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };

        let record = json!({});

        assert!(PrivacyEvaluator::evaluate(&policy, &record, &anon_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &user_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &system_actor).is_ok());
    }

    #[test]
    fn test_owner_by_id_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![owner::OWNER_BY_ID],
            block: vec![],
            always_block: vec![],
        };

        let owner_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let other_actor = Actor::User {
            user_id: "user456".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };
        let anon_actor = Actor::Anonymous;

        let owner_record = json!({ "id": "user123" });

        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &owner_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &other_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &system_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &anon_actor).is_err());
    }

    #[test]
    fn test_owner_by_user_field_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![owner::OWNER_BY_USER_FIELD],
            block: vec![],
            always_block: vec![],
        };

        let owner_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let other_actor = Actor::User {
            user_id: "user456".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };

        let owner_record = json!({ "user": "user:user123" });
        let simple_record = json!({ "user": "user123" });

        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &owner_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &other_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &owner_record, &system_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policy, &simple_record, &owner_actor).is_ok());
    }

    #[test]
    fn test_block_all_policy() {
        let policy = crate::privacy::PrivacyPolicy {
            always_allow: vec![],
            allow: vec![],
            block: vec![],
            always_block: vec![common::BLOCK_ALL],
        };

        let anon_actor = Actor::Anonymous;
        let user_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let system_actor = Actor::System {
            operation: "test".to_string(),
        };

        let record = json!({});

        assert!(PrivacyEvaluator::evaluate(&policy, &record, &anon_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &user_actor).is_err());
        assert!(PrivacyEvaluator::evaluate(&policy, &record, &system_actor).is_err());
    }

    #[test]
    fn test_privacy_policies_struct() {
        use crate::privacy::PrivacyPolicies;

        let policies = PrivacyPolicies {
            read: crate::privacy::PrivacyPolicy {
                always_allow: vec![],
                allow: vec![common::PUBLIC_READ],
                block: vec![],
                always_block: vec![],
            },
            create: crate::privacy::PrivacyPolicy {
                always_allow: vec![],
                allow: vec![common::AUTHENTICATED],
                block: vec![],
                always_block: vec![],
            },
            update: crate::privacy::PrivacyPolicy {
                always_allow: vec![],
                allow: vec![owner::OWNER_BY_ID],
                block: vec![],
                always_block: vec![],
            },
            delete: crate::privacy::PrivacyPolicy {
                always_allow: vec![],
                allow: vec![common::SYSTEM_ONLY],
                block: vec![],
                always_block: vec![],
            },
        };

        let user_actor = Actor::User {
            user_id: "user123".to_string(),
        };
        let record = json!({ "id": "user123" });

        assert!(PrivacyEvaluator::evaluate(&policies.read, &record, &user_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policies.create, &record, &user_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policies.update, &record, &user_actor).is_ok());
        assert!(PrivacyEvaluator::evaluate(&policies.delete, &record, &user_actor).is_err());
    }
}
