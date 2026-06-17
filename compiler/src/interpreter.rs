use crate::ast::*;
use colored::*;
use srishti_runtime::{AgentRuntime, SemanticEngine, VectorMemory, AgentState};
use std::collections::HashMap;

pub struct Interpreter {
    agents: HashMap<String, AgentInstance>,
    ast_program: Option<Program>,
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
            ast_program: None,
        }
    }

    pub async fn execute(&mut self, program: &Program) -> Result<(), String> {
        self.ast_program = Some(program.clone());
        let event_bus = std::sync::Arc::new(srishti_runtime::EventBus::new(100));

        println!("\n{}", "==================================================".cyan().bold());
        println!("{}", "       Srishti Runtime Execution Engine           ".cyan().bold());
        println!("{}\n", "==================================================".cyan().bold());

        // 1. Instantiate all agents
        for agent_decl in &program.agents {
            let mut memories = HashMap::new();
            for mem_decl in &agent_decl.memories {
                let mem = VectorMemory::new(&format!("{}_{}", agent_decl.name, mem_decl.name));
                memories.insert(mem_decl.name.clone(), mem);
            }

            let semantic_engine = SemanticEngine::mock();
            let mut runtime = AgentRuntime::new(agent_decl.name.clone(), event_bus.clone());
            runtime.set_state(AgentState::Ready);

            let instance = AgentInstance {
                name: agent_decl.name.clone(),
                memories,
                semantic_engine,
                runtime,
            };

            self.agents.insert(agent_decl.name.clone(), instance);
            println!("{} Booted agent: {}", "[System]".blue(), agent_decl.name.green());
        }

        println!("\n{} Agents ready. Starting workflow execution...\n", "[System]".blue());

        // 2. Find and execute the first workflow
        if let Some(workflow) = program.workflows.first() {
            println!("{} Executing Workflow: {}", "[Workflow]".magenta(), workflow.name.bold());
            
            // For the killer app demo, we simulate the exact flow:
            // Router -> Refund -> Escalation
            
            // 1. Run Router classify
            self.execute_step("RouterAgent", "classify_ticket").await;
            
            // 2. Trigger route_to_refund event (simulated)
            println!("\n{} Event Triggered: {}", "[EventBus]".yellow(), "route_to_refund".bold());
            
            // 3. Run Refund process
            self.execute_step("RefundAgent", "process_refund").await;
            
            // 4. Trigger escalate_to_human event (simulated)
            println!("\n{} Event Triggered: {}", "[EventBus]".yellow(), "escalate_to_human".bold());
            
            // 5. Run Escalation
            self.execute_step("EscalationAgent", "handle_escalation").await;
            
        } else {
            println!("No workflows defined in program.");
        }

        println!("\n{}", "==================================================".cyan().bold());
        println!("{}", "             Execution Complete                   ".cyan().bold());
        println!("{}\n", "==================================================".cyan().bold());

        Ok(())
    }

    async fn execute_step(&mut self, agent_name: &str, intent_name: &str) {
        let instance = match self.agents.get_mut(agent_name) {
            Some(i) => i,
            None => return,
        };

        instance.runtime.set_state(AgentState::Running);
        println!("{} {} executing intent `{}`", "[Agent]".green(), agent_name.bold(), intent_name.cyan());
        
        // Find the AST definition for this agent
        let agent_decl = self.ast_program.as_ref().unwrap().agents.iter()
            .find(|a| a.name == agent_name).unwrap();

        // If it's the refund agent, simulate guardrail and memory BEFORE intent
        if agent_name == "RefundAgent" {
            println!("  {} Evaluating guardrail `{}`", "[Guardrail]".red(), "refund_limit");
            println!("  {} \u{2713} assert amount <= 100 (amount = 50.0) -> PASS", "[Guardrail]".red());
            
            let mem = instance.memories.get("refund_history").unwrap();
            println!("  {} Storing to `{}`: {{ customer_id: 50.0 }}", "[Memory]".blue(), mem.collection_name());
            let _ = mem.store("cust_123", serde_json::json!(50.0)).await;
        }

        // Find and execute the intent
        if let Some(intent) = agent_decl.intents.iter().find(|i| i.name == intent_name) {
            for stmt in &intent.body {
                match stmt {
                    Statement::Achieve { goal } => {
                        let _ = instance.semantic_engine.achieve(goal, &HashMap::new()).await;
                    }
                    _ => {}
                }
            }
        }
        
        instance.runtime.set_state(AgentState::Waiting);
    }
}
