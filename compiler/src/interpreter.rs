use crate::ast::*;
use srishti_runtime::SemanticEngine;
use std::collections::HashMap;

pub struct Interpreter {
    semantic_engine: SemanticEngine,
}

impl Interpreter {
    pub fn new() -> Self {
        // Simple fallback logic for Phase 3
        let provider = if std::env::var("OPENAI_API_KEY").is_ok() {
            "openai"
        } else {
            "mock"
        };
        let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
        
        Self {
            semantic_engine: SemanticEngine::new(api_key, provider.to_string()),
        }
    }

    pub async fn execute(&mut self, program: &Program) -> anyhow::Result<()> {
        if program.agents.is_empty() {
            return Err(anyhow::anyhow!("No agents defined in the program."));
        }

        let agent = &program.agents[0];
        println!("🤖 Booting Agent: {}", agent.name);
        println!("🧠 LLM Provider: {}", self.semantic_engine.provider_name());

        // Process Guardrails conceptually (we will just print them for now in the interpreter)
        if !agent.guardrails.is_empty() {
            println!("🛡️ Loaded {} Guardrails", agent.guardrails.len());
        }

        // Execute Intents
        for intent in &agent.intents {
            println!("⚡ Executing Intent: {}", intent.name);
            for stmt in &intent.body {
                self.execute_statement(stmt).await?;
            }
        }

        println!("✅ Agent {} execution complete.", agent.name);
        Ok(())
    }

    async fn execute_statement(&self, stmt: &Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::Achieve { goal } => {
                let context = HashMap::new();
                let response = self.semantic_engine.achieve(goal, &context).await?;
                println!("   💡 LLM Output: {}", response);
                Ok(())
            }
            Statement::Assert { condition, else_action } => {
                println!("   🔍 Evaluating Assert: {:?}", condition);
                // In a full interpreter, we would evaluate the AST expression.
                // For Phase 3 MVP, we just acknowledge it.
                if let Some(action) = else_action {
                    println!("      ↳ Else action: {}", action);
                }
                Ok(())
            }
            Statement::RawRust(code) => {
                println!("   🦀 (Skipping Raw Rust block during Interpretation: {})", code);
                Ok(())
            }
        }
    }
}
