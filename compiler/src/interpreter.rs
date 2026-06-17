use crate::ast::*;
use srishti_runtime::{AgentRuntime, SemanticEngine, VectorMemory};
use std::collections::HashMap;

pub struct Interpreter {
    agents: HashMap<String, AgentInstance>,
}

pub struct AgentInstance {
    pub name: String,
    pub memories: HashMap<String, VectorMemory>,
    pub semantic_engine: SemanticEngine,
    pub runtime: AgentRuntime,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub async fn execute(&mut self, program: &Program) -> Result<(), String> {
        let event_bus = std::sync::Arc::new(srishti_runtime::EventBus::new(100));

        // 1. Instantiate all agents
        for agent_decl in &program.agents {
            let mut memories = HashMap::new();
            for mem_decl in &agent_decl.memories {
                memories.insert(
                    mem_decl.name.clone(),
                    VectorMemory::new(&format!("{}_{}", agent_decl.name, mem_decl.name)),
                );
            }

            let semantic_engine = SemanticEngine::mock(); // Default to mock for interpreter run
            let runtime = AgentRuntime::new(agent_decl.name.clone(), event_bus.clone());

            let instance = AgentInstance {
                name: agent_decl.name.clone(),
                memories,
                semantic_engine,
                runtime,
            };

            self.agents.insert(agent_decl.name.clone(), instance);
            println!("Agent instantiated: {}", agent_decl.name);
        }

        println!("Execution finished (Interpreter MVP).");
        Ok(())
    }
}
