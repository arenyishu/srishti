use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// An event that can be published and subscribed to
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub source: String,
    pub data: Value,
    pub timestamp: u64,
}

impl Event {
    pub fn new(name: &str, source: &str, data: Value) -> Self {
        Self {
            name: name.to_string(),
            source: source.to_string(),
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Callback type for event handlers
type EventCallback = Box<dyn Fn(&Event) + Send + Sync>;

/// Simple synchronous event bus for agent communication
pub struct EventBus {
    handlers: Arc<RwLock<HashMap<String, Vec<EventCallback>>>>,
    event_log: Arc<RwLock<Vec<Event>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_log: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Subscribe to an event
    pub fn on<F>(&self, event_name: &str, callback: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
        println!("  [EventBus] Registered handler for '{}'", event_name);
    }
    
    /// Emit an event
    pub fn emit(&self, event: Event) {
        println!("  [EventBus] Emitting '{}' from {}", event.name, event.source);
        
        // Log the event
        if let Ok(mut log) = self.event_log.write() {
            log.push(event.clone());
        }
        
        // Call handlers
        if let Ok(handlers) = self.handlers.read() {
            if let Some(callbacks) = handlers.get(&event.name) {
                for callback in callbacks {
                    callback(&event);
                }
            }
        }
    }
    
    /// Get event history
    pub fn history(&self) -> Vec<Event> {
        self.event_log.read().unwrap_or_else(|e| e.into_inner()).clone()
    }
    
    /// Clear all handlers and history
    pub fn clear(&self) {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.clear();
        }
        if let Ok(mut log) = self.event_log.write() {
            log.clear();
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
