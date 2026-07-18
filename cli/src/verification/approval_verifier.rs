use std::sync::Arc;
use srishti_runtime::governance::approval_store::{ApprovalStore, PendingApproval};

pub async fn verify() -> bool {
    let store = Arc::new(ApprovalStore::new());

    store.add_approval("req_1".to_string(), PendingApproval {
        id: "req_1".to_string(),
        agent_pid: "pid_1".to_string(),
        action: "refund".to_string(),
        amount: Some(150.0),
        status: "pending".to_string(),
        context: "Testing approval".to_string(),
    }).await;

    let pending = store.get_pending().await;
    if pending.len() != 1 {
        return false;
    }

    if store.update_status("req_1", "approved").await.is_err() {
        return false;
    }

    let pending_after = store.get_pending().await;
    pending_after.is_empty()
}
