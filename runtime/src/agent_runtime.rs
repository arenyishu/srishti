use crate::event_bus::{Event, EventBus};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Init,
    Ready,
    Running,
    Waiting,
    Done,
}

pub struct AgentRuntime {
    pub name: String,
    pub state: AgentState,
    event_bus: Arc<EventBus>,
}

impl AgentRuntime {
    pub fn new(name: String, event_bus: Arc<EventBus>) -> Self {
        Self {
            name,
            state: AgentState::Init,
            event_bus,
        }
    }

    pub fn set_state(&mut self, state: AgentState) {
        println!("  [Agent:{}] State transition: {:?} -> {:?}", self.name, self.state, state);
        self.state = state;
    }

    pub fn emit(&self, event_name: String, payload: serde_json::Value) {
        let mut map = std::collections::HashMap::new();
        map.insert("data".to_string(), payload);
        
        let event = Event {
            name: event_name,
            source_agent: self.name.clone(),
            target_agent: None,
            payload: map,
        };
        
        let _ = self.event_bus.publish(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_bus.subscribe()
    }
}
