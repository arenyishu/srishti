use crate::ast::*;
use colored::*;
use srishti_runtime::{AgentRuntime, SemanticEngine, VectorMemory, AgentState};
use serde_json::Value;
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

            let semantic_engine = SemanticEngine::from_env();
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

        // The Event Loop subscription must happen BEFORE we emit initial events!
        let mut rx = event_bus.subscribe();

        // Start by kicking off any workflows
        if let Some(workflow) = program.workflows.first() {
            println!("{} Executing Workflow: {}", "[Workflow]".magenta(), workflow.name.bold());
            for step in &workflow.steps {
                self.execute_intent(&step.agent, &step.intent).await;
            }
        } else {
            // For Chatbot or event-driven systems, emit an initial event
            println!("\n{} No workflows found. Emitting `start_chat` to bootstrap...", "[System]".blue());
            let mut payload = HashMap::new();
            payload.insert("data".to_string(), Value::Null);
            let _ = event_bus.publish(srishti_runtime::Event {
                name: "start_chat".to_string(),
                source_agent: "System".to_string(),
                target_agent: None,
                payload,
            });
        }

        // The Event Loop
        loop {
            // Wait for an event with a timeout. If idle, we break.
            let event = match tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await {
                Ok(Ok(ev)) => ev,
                _ => break, // Timeout or channel closed
            };

            // Avoid double printing if we just booted
            if event.source_agent != "System" {
                println!("\n{} Event Triggered: {}", "[EventBus]".yellow(), event.name.bold());
            }

            let mut handlers_to_run = Vec::new();

            for agent_decl in &program.agents {
                for handler in &agent_decl.event_handlers {
                    if handler.event_name == event.name {
                        handlers_to_run.push((agent_decl.name.clone(), handler.clone(), event.clone()));
                    }
                }
            }

            for (agent_name, handler, ev) in handlers_to_run {
                Box::pin(self.execute_event_handler(&agent_name, &handler, &ev)).await;
            }
        }

        println!("\n{}", "==================================================".cyan().bold());
        println!("{}", "             Execution Complete                   ".cyan().bold());
        println!("{}\n", "==================================================".cyan().bold());

        Ok(())
    }

    async fn execute_event_handler(&mut self, agent_name: &str, handler: &EventHandler, event: &srishti_runtime::Event) {
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Running);
        println!("{} {} handling event `{}`", "[Agent]".green(), agent_name.bold(), event.name.cyan());
        
        let mut ctx = HashMap::new();
        if handler.params.len() == 1 {
            if let Some(val) = event.payload.get("data") {
                ctx.insert(handler.params[0].name.clone(), val.clone());
            } else {
                ctx.insert(handler.params[0].name.clone(), serde_json::to_value(&event.payload).unwrap());
            }
        }
        
        Box::pin(self.execute_statements(agent_name, &handler.body, &mut ctx)).await;
        
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Waiting);
    }

    async fn execute_intent(&mut self, agent_name: &str, intent_name: &str) -> Value {
        let agent_decl = self.ast_program.as_ref().unwrap().agents.iter()
            .find(|a| a.name == agent_name).unwrap();

        let intent = agent_decl.intents.iter().find(|i| i.name == intent_name).unwrap();
        let body = intent.body.clone();
        
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Running);
        println!("{} {} executing intent `{}`", "[Agent]".green(), agent_name.bold(), intent_name.cyan());
        
        let mut ctx = HashMap::new();
        let result = Box::pin(self.execute_statements(agent_name, &body, &mut ctx)).await;
        
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Waiting);

        result.unwrap_or(Value::Null)
    }

    async fn execute_statements(&mut self, agent_name: &str, stmts: &[Statement], ctx: &mut HashMap<String, Value>) -> Option<Value> {
        for stmt in stmts {
            match Box::pin(self.execute_statement(agent_name, stmt, ctx)).await {
                Some(val) => return Some(val),
                None => {}
            }
        }
        None
    }

    async fn execute_statement(&mut self, agent_name: &str, stmt: &Statement, ctx: &mut HashMap<String, Value>) -> Option<Value> {
        match stmt {
            Statement::LetBinding { name, value } => {
                let val = Box::pin(self.execute_expression(agent_name, value, ctx)).await;
                ctx.insert(name.clone(), val);
                None
            }
            Statement::ExprStmt(expr) => {
                Box::pin(self.execute_expression(agent_name, expr, ctx)).await;
                None
            }
            Statement::EmitEvent { event_name, args } => {
                let mut payload = HashMap::new();
                if args.len() == 1 {
                    let val = Box::pin(self.execute_expression(agent_name, &args[0], ctx)).await;
                    payload.insert("data".to_string(), val);
                }
                let instance = self.agents.get(agent_name).unwrap();
                instance.runtime.emit(event_name.clone(), serde_json::to_value(&payload).unwrap());
                None
            }
            Statement::ReturnStmt { value } => {
                if let Some(v) = value {
                    Some(Box::pin(self.execute_expression(agent_name, v, ctx)).await)
                } else {
                    Some(Value::Null)
                }
            }
            Statement::IfStmt { condition, then_body, else_body } => {
                let cond_val = Box::pin(self.execute_expression(agent_name, condition, ctx)).await;
                if cond_val.as_bool().unwrap_or(false) {
                    Box::pin(self.execute_statements(agent_name, then_body, ctx)).await
                } else if let Some(el) = else_body {
                    Box::pin(self.execute_statements(agent_name, el, ctx)).await
                } else {
                    None
                }
            }
            Statement::Achieve { goal } => {
                let instance = self.agents.get(agent_name).unwrap();
                let result = match instance.semantic_engine.achieve(goal, ctx).await {
                    Ok(val) => val,
                    Err(e) => Value::String(format!("Error: {}", e)),
                };
                Some(result)
            }
            Statement::Assert { condition, else_action } => {
                let cond_val = Box::pin(self.execute_expression(agent_name, condition, ctx)).await;
                if !cond_val.as_bool().unwrap_or(false) {
                    println!("  {} Assertion failed! {}", "[Guardrail]".red(), else_action.as_deref().unwrap_or(""));
                } else {
                    println!("  {} Assertion passed.", "[Guardrail]".green());
                }
                None
            }
            _ => None,
        }
    }

    async fn execute_expression(&mut self, agent_name: &str, expr: &Expression, ctx: &mut HashMap<String, Value>) -> Value {
        match expr {
            Expression::LiteralString(s) => Value::String(s.clone()),
            Expression::LiteralInt(i) => Value::Number((*i).into()),
            Expression::LiteralFloat(f) => Value::Number(serde_json::Number::from_f64(*f).unwrap()),
            Expression::BooleanLiteral(b) => Value::Bool(*b),
            Expression::Variable(id) => {
                ctx.get(id).cloned().unwrap_or(Value::Null)
            }
            Expression::BinaryOp { left, op, right } => {
                let l_val = Box::pin(self.execute_expression(agent_name, left, ctx)).await;
                let r_val = Box::pin(self.execute_expression(agent_name, right, ctx)).await;
                
                // MVP: Only support <= for numbers
                if op == "<=" {
                    let l_num = l_val.as_f64().unwrap_or(0.0);
                    let r_num = r_val.as_f64().unwrap_or(0.0);
                    Value::Bool(l_num <= r_num)
                } else {
                    Value::Null
                }
            }
            Expression::MethodCall { object, method, args } => {
                let mut eval_args = Vec::new();
                for a in args {
                    eval_args.push(Box::pin(self.execute_expression(agent_name, a, ctx)).await);
                }
                
                if let Expression::Variable(obj_name) = &**object {
                    if obj_name == "IO" {
                        if method == "print" {
                            let text = eval_args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                            println!("{} {}", "[IO]".cyan(), text);
                            return Value::Null;
                        } else if method == "read_line" {
                            use std::io::Write;
                            print!("{} ", "[User]".green());
                            std::io::stdout().flush().unwrap();
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            return Value::String(input.trim().to_string());
                        }
                    } else if obj_name == "Logger" {
                        if method == "info" {
                            let text = eval_args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                            println!("{} {}", "[Logger]".blue(), text);
                            return Value::Null;
                        }
                    } else if obj_name == "self" {
                        let (is_intent, guard_info, is_tool) = {
                            let agent_decl = self.ast_program.as_ref().unwrap().agents.iter()
                                .find(|a| a.name == agent_name).unwrap();
                            let is_intent = agent_decl.intents.iter().any(|i| i.name == *method);
                            let guard_info = agent_decl.guardrails.iter().find(|g| g.name == *method).map(|g| (g.args.clone(), g.body.clone()));
                            let is_tool = agent_decl.tools.iter().any(|t| t.name == *method);
                            (is_intent, guard_info, is_tool)
                        };
                        
                        if is_intent {
                            return Box::pin(self.execute_intent(agent_name, method)).await;
                        } else if let Some((guard_args, guard_body)) = guard_info {
                            println!("  {} Checking guardrail `{}`", "[Guardrail]".magenta(), method);
                            let mut g_ctx = ctx.clone(); // basic scope
                            // Bind arguments if we passed them (MVP assumes 1 arg matches 1 param)
                            if let (Some(arg_expr), Some(param)) = (eval_args.get(0), guard_args.get(0)) {
                                g_ctx.insert(param.name.clone(), arg_expr.clone());
                            }
                            Box::pin(self.execute_statements(agent_name, &guard_body, &mut g_ctx)).await;
                            return Value::Null;
                        } else if is_tool {
                            println!("  {} Executing tool `{}`", "[Tool]".magenta(), method);
                            return Value::Null; // Mock tool execution
                        } else {
                            return Value::Null;
                        }
                    } else {
                        let instance = self.agents.get(agent_name).unwrap();
                        if instance.memories.contains_key(obj_name) {
                            if method == "store" {
                                println!("  {} Storing to `{}`", "[Memory]".blue(), obj_name);
                                return Value::Null;
                            } else if method == "search" {
                                println!("  {} Searching `{}`", "[Memory]".blue(), obj_name);
                                return Value::Null;
                            }
                        }
                    }
                }
                Value::Null
            }
            Expression::FieldAccess { object, field } => {
                let obj_val = Box::pin(self.execute_expression(agent_name, object, ctx)).await;
                if let Some(obj) = obj_val.as_object() {
                    obj.get(field).cloned().unwrap_or(Value::Null)
                } else {
                    Value::Null
                }
            }
            _ => Value::Null,
        }
    }
}
