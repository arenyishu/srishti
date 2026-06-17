use crate::memory::VectorMemory;
use crate::semantic_engine::SemanticEngine;
use serde_json::Value;
use std::collections::HashMap;

/// Agent lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Initializing,
    Ready,
    Running,
    Waiting,
    Done,
    Error(String),
}

impl std::fmt::Display for AgentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "INITIALIZING"),
            Self::Ready => write!(f, "READY"),
            Self::Running => write!(f, "RUNNING"),
            Self::Waiting => write!(f, "WAITING"),
            Self::Done => write!(f, "DONE"),
            Self::Error(e) => write!(f, "ERROR: {}", e),
        }
    }
}

/// Represents a guardrail check result
#[derive(Debug)]
pub enum GuardrailResult {
    Pass,
    Fail(String),
}

/// Configuration for creating an agent instance
pub struct AgentConfig {
    pub name: String,
    pub provider: String,
    pub api_key: String,
    pub memory_collections: Vec<String>,
}

/// A running agent instance
pub struct AgentInstance {
    pub name: String,
    pub state: AgentState,
    pub semantic_engine: SemanticEngine,
    pub memories: HashMap<String, VectorMemory>,
    pub inbox: Vec<AgentMessage>,
    pub outbox: Vec<AgentMessage>,
    pub context: HashMap<String, Value>,
}

/// Message between agents
#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub from: String,
    pub to: String,
    pub content: Value,
    pub message_type: String,
}

impl AgentInstance {
    pub fn new(config: AgentConfig) -> Self {
        let semantic_engine = SemanticEngine::new(config.api_key, config.provider);
        let mut memories = HashMap::new();
        for collection in &config.memory_collections {
            memories.insert(
                collection.clone(),
                VectorMemory::new(&format!("{}_{}", config.name, collection)),
            );
        }

        Self {
            name: config.name,
            state: AgentState::Initializing,
            semantic_engine,
            memories,
            inbox: Vec::new(),
            outbox: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Create a mock agent for testing
    pub fn mock(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: AgentState::Initializing,
            semantic_engine: SemanticEngine::mock(),
            memories: HashMap::new(),
            outbox: Vec::new(),
            inbox: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Transition to Ready state
    pub fn initialize(&mut self) {
        println!("  [Agent:{}] Initializing...", self.name);
        self.state = AgentState::Ready;
        println!("  [Agent:{}] State -> {}", self.name, self.state);
    }

    /// Execute an intent using the semantic engine
    pub async fn execute_intent(
        &mut self,
        intent_name: &str,
        goal: &str,
    ) -> Result<Value, anyhow::Error> {
        self.state = AgentState::Running;
        println!(
            "  [Agent:{}] Executing intent '{}': {}",
            self.name, intent_name, goal
        );

        let result = self.semantic_engine.achieve(goal, &self.context).await?;

        println!("  [Agent:{}] Intent '{}' completed", self.name, intent_name);
        self.state = AgentState::Ready;
        Ok(result)
    }

    /// Run a guardrail check
    pub fn check_guardrail(
        &self,
        guardrail_name: &str,
        condition: bool,
        error_msg: &str,
    ) -> GuardrailResult {
        println!(
            "  [Agent:{}] Checking guardrail '{}'",
            self.name, guardrail_name
        );
        if condition {
            println!(
                "  [Agent:{}] Guardrail '{}' -> PASS",
                self.name, guardrail_name
            );
            GuardrailResult::Pass
        } else {
            println!(
                "  [Agent:{}] Guardrail '{}' -> FAIL: {}",
                self.name, guardrail_name, error_msg
            );
            GuardrailResult::Fail(error_msg.to_string())
        }
    }

    /// Send a message to another agent
    pub fn send_message(&mut self, to: &str, content: Value, message_type: &str) {
        let msg = AgentMessage {
            from: self.name.clone(),
            to: to.to_string(),
            content,
            message_type: message_type.to_string(),
        };
        println!("  [Agent:{}] Sending {} to {}", self.name, message_type, to);
        self.outbox.push(msg);
    }

    /// Receive a message
    pub fn receive_message(&mut self, msg: AgentMessage) {
        println!(
            "  [Agent:{}] Received {} from {}",
            self.name, msg.message_type, msg.from
        );
        self.inbox.push(msg);
    }

    /// Get a memory collection
    pub fn memory(&self, name: &str) -> Option<&VectorMemory> {
        self.memories.get(name)
    }

    /// Add memory collection
    pub fn add_memory(&mut self, name: &str) {
        self.memories.insert(
            name.to_string(),
            VectorMemory::new(&format!("{}_{}", self.name, name)),
        );
    }

    /// Set context value
    pub fn set_context(&mut self, key: &str, value: Value) {
        self.context.insert(key.to_string(), value);
    }

    /// Mark as done
    pub fn finish(&mut self) {
        self.state = AgentState::Done;
        println!("  [Agent:{}] State -> {}", self.name, self.state);
    }
}
