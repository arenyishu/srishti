use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use ring::digest::{Context, SHA256};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub event_type: String,
    pub agent_id: String,
    pub description: String,
    pub previous_hash: String,
    pub current_hash: String,
}

#[derive(Clone)]
pub struct AuditLogger {
    file_path: String,
    last_hash: Arc<Mutex<String>>,
}

impl AuditLogger {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            last_hash: Arc::new(Mutex::new(hex::encode(Context::new(&SHA256).finish().as_ref()))),
        }
    }

    pub async fn log(&self, event_type: &str, agent_id: &str, description: &str) -> Result<(), String> {
        let mut last_hash_guard = self.last_hash.lock().await;
        
        let mut ctx = Context::new(&SHA256);
        ctx.update(last_hash_guard.as_bytes());
        ctx.update(event_type.as_bytes());
        ctx.update(agent_id.as_bytes());
        ctx.update(description.as_bytes());
        let current_hash = hex::encode(ctx.finish().as_ref());
        
        let event = AuditEvent {
            timestamp: Utc::now().to_rfc3339(),
            event_type: event_type.to_string(),
            agent_id: agent_id.to_string(),
            description: description.to_string(),
            previous_hash: last_hash_guard.clone(),
            current_hash: current_hash.clone(),
        };

        *last_hash_guard = current_hash;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| e.to_string())?;

        let json = serde_json::to_string(&event).map_err(|e| e.to_string())?;
        writeln!(file, "{}", json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
