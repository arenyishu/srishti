use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessIdentity {
    pub name: String,
    pub id: String,
    pub version: String,
    pub permissions: Vec<String>,
}

#[derive(Clone)]
pub struct ProcessTable {
    processes: Arc<RwLock<HashMap<String, ProcessIdentity>>>,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_local(&self, identity: ProcessIdentity) -> Result<(), String> {
        let mut table = self.processes.write().await;
        if table.contains_key(&identity.id) {
            return Err(format!("Process with ID '{}' already registered", identity.id));
        }
        table.insert(identity.id.clone(), identity);
        Ok(())
    }

    pub async fn register(&self, identity: ProcessIdentity) -> Result<(), String> {
        let mut table = self.processes.write().await;
        if table.contains_key(&identity.id) {
            return Err(format!("Process with ID '{}' already registered", identity.id));
        }
        
        let identity_clone = identity.clone();
        let client = reqwest::Client::new();
        println!("[TELEMETRY] Sending POST /api/internal/process");
        println!("  URL: http://127.0.0.1:3000/api/internal/process");
        println!("  Payload: {:?}", identity_clone);
        match client.post("http://127.0.0.1:3000/api/internal/process")
            .json(&identity_clone)
            .send()
            .await {
            Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
            Err(e) => println!("  Failure: {}", e),
        }

        table.insert(identity.id.clone(), identity);
        Ok(())
    }

    pub async fn get_permissions(&self, id: &str) -> Option<Vec<String>> {
        let table = self.processes.read().await;
        table.get(id).map(|i| i.permissions.clone())
    }

    pub async fn check_permission(&self, id: &str, required_perm: &str) -> bool {
        if let Some(perms) = self.get_permissions(id).await {
            perms.iter().any(|p| p == required_perm || p == "*")
        } else {
            false
        }
    }

    pub async fn list(&self) -> Vec<ProcessIdentity> {
        let table = self.processes.read().await;
        table.values().cloned().collect()
    }

    pub async fn terminate_local(&self, id: &str) -> Result<(), String> {
        let mut table = self.processes.write().await;
        if table.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Process with ID '{}' not found", id))
        }
    }

    pub async fn terminate(&self, id: &str) -> Result<(), String> {
        let mut table = self.processes.write().await;
        if table.remove(id).is_some() {
            let id_clone = id.to_string();
            let payload = serde_json::json!({ "id": id_clone });
            let client = reqwest::Client::new();
            println!("[TELEMETRY] Sending POST /api/internal/process_remove");
            println!("  URL: http://127.0.0.1:3000/api/internal/process_remove");
            println!("  Payload: {:?}", payload);
            match client.post("http://127.0.0.1:3000/api/internal/process_remove")
                .json(&payload)
                .send()
                .await {
                Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
                Err(e) => println!("  Failure: {}", e),
            }
            Ok(())
        } else {
            Err(format!("Process with ID '{}' not found", id))
        }
    }
}

impl Default for ProcessTable {
    fn default() -> Self {
        Self::new()
    }
}
