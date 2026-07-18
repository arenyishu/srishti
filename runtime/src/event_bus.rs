use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub name: String,
    pub source_agent: String,
    pub target_agent: Option<String>,
    pub payload: HashMap<String, Value>,
}

pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub async fn publish(&self, event: Event) -> Result<usize, broadcast::error::SendError<Event>> {
        println!("  [EventBus] Publishing event '{}' from {}", event.name, event.source_agent);
        
        let event_clone = event.clone();
        let client = reqwest::Client::new();
        println!("[TELEMETRY] Sending POST /api/internal/events");
        println!("  URL: http://127.0.0.1:3000/api/internal/events");
        println!("  Payload: {:?}", event_clone);
        match client.post("http://127.0.0.1:3000/api/internal/events")
            .json(&event_clone)
            .send()
            .await {
            Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
            Err(e) => println!("  Failure: {}", e),
        }

        self.sender.send(event)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(100)
    }
}
