use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub id: String,
    pub agent_pid: String,
    pub action: String,
    pub amount: Option<f64>,
    pub status: String, // "pending", "approved", "rejected"
    pub context: String,
}

#[derive(Clone)]
pub struct ApprovalStore {
    // In a real OS this would be SQLite or etcd. For now, in-memory.
    approvals: Arc<RwLock<HashMap<String, PendingApproval>>>,
}

impl ApprovalStore {
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_approval_local(&self, id: String, approval: PendingApproval) {
        let mut store = self.approvals.write().await;
        store.insert(id, approval);
    }

    pub async fn add_approval(&self, id: String, approval: PendingApproval) {
        let mut store = self.approvals.write().await;
        
        let approval_clone = approval.clone();
        let client = reqwest::Client::new();
        println!("[TELEMETRY] Sending POST /api/internal/approval");
        println!("  URL: http://127.0.0.1:3000/api/internal/approval");
        println!("  Payload: {:?}", approval_clone);
        match client.post("http://127.0.0.1:3000/api/internal/approval")
            .json(&approval_clone)
            .send()
            .await {
            Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
            Err(e) => println!("  Failure: {}", e),
        }

        store.insert(id, approval);
    }

    pub async fn get_pending(&self) -> Vec<PendingApproval> {
        let store = self.approvals.read().await;
        store.values()
            .filter(|a| a.status == "pending")
            .cloned()
            .collect()
    }

    pub async fn update_status_local(&self, id: &str, new_status: &str) -> Result<(), String> {
        let mut store = self.approvals.write().await;
        if let Some(approval) = store.get_mut(id) {
            approval.status = new_status.to_string();
            Ok(())
        } else {
            Err(format!("Approval ID {} not found", id))
        }
    }

    pub async fn update_status(&self, id: &str, new_status: &str) -> Result<(), String> {
        let mut store = self.approvals.write().await;
        if let Some(approval) = store.get_mut(id) {
            approval.status = new_status.to_string();
            
            let approval_clone = approval.clone();
            let client = reqwest::Client::new();
            println!("[TELEMETRY] Sending POST /api/internal/approval_update");
            println!("  URL: http://127.0.0.1:3000/api/internal/approval_update");
            println!("  Payload: {:?}", approval_clone);
            match client.post("http://127.0.0.1:3000/api/internal/approval_update")
                .json(&approval_clone)
                .send()
                .await {
                Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
                Err(e) => println!("  Failure: {}", e),
            }
            
            Ok(())
        } else {
            Err(format!("Approval ID {} not found", id))
        }
    }
}

impl Default for ApprovalStore {
    fn default() -> Self {
        Self::new()
    }
}
