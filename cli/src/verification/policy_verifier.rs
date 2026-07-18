use std::sync::Arc;
use srishti_runtime::policy_engine::{PolicyEngine, Policy, PolicyAction};
use srishti_runtime::governance::approval_store::ApprovalStore;

pub async fn verify() -> bool {
    let approval_store = Arc::new(ApprovalStore::new());
    let mut engine = PolicyEngine::new(approval_store);

    let policy = Policy {
        name: "RefundApproval".to_string(),
        allowed_roles: vec![],
        human_approval_limit: Some(100.0),
    };

    engine.register_policy(policy);

    let action = engine.evaluate("pid_123", "RefundApproval", None, Some(150.0)).await;

    matches!(action, PolicyAction::SuspendForApproval { .. })
}
