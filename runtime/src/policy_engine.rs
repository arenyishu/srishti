use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PolicyAction {
    Allow,
    Deny,
    SuspendForApproval { limit: f64 },
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub name: String,
    pub allowed_roles: Vec<String>,
    pub human_approval_limit: Option<f64>,
}

use crate::governance::approval_store::{ApprovalStore, PendingApproval};
use std::sync::Arc;
use uuid::Uuid;

pub struct PolicyEngine {
    policies: HashMap<String, Policy>,
    role_hierarchy: HashMap<String, Vec<String>>, // Role -> Inherits these roles
    pub approval_store: Arc<ApprovalStore>,
}

impl PolicyEngine {
    pub fn new(approval_store: Arc<ApprovalStore>) -> Self {
        Self {
            policies: HashMap::new(),
            role_hierarchy: HashMap::new(),
            approval_store,
        }
    }

    pub fn register_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    pub fn add_role_inheritance(&mut self, parent: &str, child: &str) {
        self.role_hierarchy.entry(parent.to_string()).or_insert_with(Vec::new).push(child.to_string());
    }

    fn role_matches(&self, user_role: &str, allowed_roles: &[String]) -> bool {
        if allowed_roles.contains(&user_role.to_string()) {
            return true;
        }
        if let Some(children) = self.role_hierarchy.get(user_role) {
            for child in children {
                if self.role_matches(child, allowed_roles) {
                    return true;
                }
            }
        }
        false
    }

    pub async fn evaluate(&self, pid: &str, policy_name: &str, role: Option<&str>, amount: Option<f64>) -> PolicyAction {
        if let Some(policy) = self.policies.get(policy_name) {
            if !policy.allowed_roles.is_empty() {
                let matches = role.map_or(false, |r| self.role_matches(r, &policy.allowed_roles));
                if !matches {
                    return PolicyAction::Deny;
                }
            }

            if let Some(limit) = policy.human_approval_limit {
                if let Some(amt) = amount {
                    if amt > limit {
                        let id = Uuid::new_v4().to_string();
                        self.approval_store.add_approval(id.clone(), PendingApproval {
                            id: id.clone(),
                            agent_pid: pid.to_string(),
                            action: policy_name.to_string(),
                            amount: Some(amt),
                            status: "pending".to_string(),
                            context: format!("Exceeded limit {}", limit),
                        }).await;
                        return PolicyAction::SuspendForApproval { limit };
                    }
                }
            }
            PolicyAction::Allow
        } else {
            // Default deny if policy is enforced but not found
            PolicyAction::Deny
        }
    }
}
