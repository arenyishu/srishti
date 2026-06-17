use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
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

    pub fn publish(&self, event: Event) -> Result<usize, broadcast::error::SendError<Event>> {
        println!("  [EventBus] Publishing event '{}' from {}", event.name, event.source_agent);
        self.sender.send(event)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(100)
    }
}
