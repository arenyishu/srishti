use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AgentRegistry {
    endpoints: Arc<RwLock<HashMap<String, String>>>, // agent_name -> gRPC URL
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            endpoints: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_endpoint(&self, agent_name: &str, url: &str) {
        let mut ep = self.endpoints.write().await;
        ep.insert(agent_name.to_string(), url.to_string());
    }

    pub async fn resolve_endpoint(&self, agent_name: &str) -> Option<String> {
        let ep = self.endpoints.read().await;
        ep.get(agent_name).cloned()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
