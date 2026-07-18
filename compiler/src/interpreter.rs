use crate::ast::*;
use colored::*;
use srishti_runtime::{AgentRuntime, SemanticEngine, VectorMemory, AgentState};
use serde_json::Value;
use std::collections::HashMap;

pub struct Interpreter {
    agents: HashMap<String, AgentInstance>,
    ast_program: Option<Program>,
    policy_engine: srishti_runtime::policy_engine::PolicyEngine,
    process_table: std::sync::Arc<srishti_runtime::process_table::ProcessTable>,
    audit_logger: srishti_runtime::audit_logger::AuditLogger,
    resource_manager: std::sync::Arc<srishti_runtime::kernel::resource_manager::ResourceManager>,
    agent_registry: std::sync::Arc<srishti_runtime::network::agent_registry::AgentRegistry>,
}

pub struct AgentInstance {
    pub name: String,
    pub memories: HashMap<String, VectorMemory>,
    pub semantic_engine: SemanticEngine,
    pub runtime: AgentRuntime,
}

impl Interpreter {
    pub fn new() -> Self {
        let approval_store = std::sync::Arc::new(srishti_runtime::governance::approval_store::ApprovalStore::new());
        Self {
            agents: HashMap::new(),
            ast_program: None,
            policy_engine: srishti_runtime::policy_engine::PolicyEngine::new(approval_store),
            process_table: std::sync::Arc::new(srishti_runtime::process_table::ProcessTable::new()),
            audit_logger: srishti_runtime::audit_logger::AuditLogger::new("srishti_audit.jsonl".to_string()),
            resource_manager: std::sync::Arc::new(srishti_runtime::kernel::resource_manager::ResourceManager::new()),
            agent_registry: std::sync::Arc::new(srishti_runtime::network::agent_registry::AgentRegistry::new()),
        }
    }

    pub async fn execute(&mut self, program: &Program) -> Result<(), String> {
        self.ast_program = Some(program.clone());
        let event_bus = std::sync::Arc::new(srishti_runtime::EventBus::new(100));

        println!("\n{}", "==================================================".cyan().bold());
        println!("{}", "       Srishti Runtime Execution Engine           ".cyan().bold());
        println!("{}\n", "==================================================".cyan().bold());

        // 0. Register Policies
        for policy in &program.policies {
            let mut limit = None;
            let mut allowed_roles = Vec::new();
            for stmt in &policy.body {
                if let Statement::RequireHumanApprovalAbove { limit: l } = stmt {
                    limit = Some(*l);
                } else if let Statement::AllowRole { role } = stmt {
                    allowed_roles.push(role.clone());
                }
            }
            self.policy_engine.register_policy(srishti_runtime::policy_engine::Policy {
                name: policy.name.clone(),
                allowed_roles,
                human_approval_limit: limit,
            });
            println!("{} Registered policy: {}", "[System]".blue(), policy.name.cyan());
        }

        // 1. Instantiate all agents
        for agent_decl in &program.agents {
            let mut memories = HashMap::new();
            for mem_decl in &agent_decl.memories {
                let encrypted = mem_decl.encryption.as_deref() == Some("AES256") || mem_decl.encryption.as_deref() == Some("true");
                let mem = VectorMemory::new(&format!("{}_{}", agent_decl.name, mem_decl.name), encrypted);
                memories.insert(mem_decl.name.clone(), mem);
            }

            let semantic_engine = SemanticEngine::from_env();
            let mut runtime = AgentRuntime::new(agent_decl.name.clone(), event_bus.clone());
            runtime.set_state(AgentState::Ready).await;

            let instance = AgentInstance {
                name: agent_decl.name.clone(),
                memories,
                semantic_engine,
                runtime,
            };

            // Register in OS Process Table
            let pid = agent_decl.id.clone().unwrap_or_else(|| format!("{}-pid-gen", agent_decl.name));
            let ver = agent_decl.version.clone().unwrap_or_else(|| "1.0.0".to_string());
            let permissions = agent_decl.permissions.clone();
            
            Box::pin(self.process_table.register(srishti_runtime::process_table::ProcessIdentity {
                name: agent_decl.name.clone(),
                id: pid.clone(),
                version: ver,
                permissions,
            })).await.expect("Process registration failed (Duplicate PID?)");

            let memory_mb = agent_decl.quota.as_ref().and_then(|q| q.memory_mb);
            let cpu_percent = agent_decl.quota.as_ref().and_then(|q| q.cpu_percent);
            let tokens_per_hour = agent_decl.quota.as_ref().and_then(|q| q.tokens_per_hour);
            Box::pin(self.resource_manager.register_process(&pid, memory_mb, cpu_percent, tokens_per_hour)).await;

            if let Some(endpoint) = &agent_decl.endpoint {
                Box::pin(self.agent_registry.register_endpoint(&agent_decl.name, endpoint)).await;
                // Parse srishti://host:port -> host:port
                if let Some(addr) = endpoint.strip_prefix("srishti://") {
                    let addr_string = addr.to_string();
                    tokio::spawn(async move {
                        if let Err(e) = srishti_runtime::network::rpc_server::start_rpc_server(&addr_string).await {
                            eprintln!("Failed to start RPC server: {}", e);
                        }
                    });
                }
            }

            self.agents.insert(agent_decl.name.clone(), instance);
            let _ = self.audit_logger.log(
                "AgentStarted",
                &pid,
                &format!("Agent {} booted", agent_decl.name),
            ).await;
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
        } else if program.schedules.is_empty() {
            // For Chatbot or event-driven systems, emit an initial event
            println!("\n{} No workflows or schedules found. Emitting `start_chat` to bootstrap...", "[System]".blue());
            let agent_name = program.agents.first().map(|a| a.name.clone()).unwrap_or_else(|| "DefaultAgent".to_string());
            let _ = event_bus.publish(srishti_runtime::Event {
                name: format!("{}_Started", agent_name),
                source_agent: agent_name.to_string(),
                target_agent: None,
                payload: HashMap::new(),
            }).await;
        }

        // Initialize OS Job Scheduler
        let sched = tokio_cron_scheduler::JobScheduler::new().await.unwrap();
        for schedule in &program.schedules {
            let event_bus_clone = event_bus.clone();
            let a_name = schedule.trigger_agent.clone();
            let intent_print = schedule.trigger_intent.clone();
            let cron_str = schedule.cron.clone();
            
            sched.add(
                tokio_cron_scheduler::Job::new_async(cron_str.as_str(), move |_uuid, _l| {
                    let eb = event_bus_clone.clone();
                    let a_name = a_name.clone();
                    Box::pin(async move {
                        let payload = HashMap::new();
                        let _ = eb.publish(srishti_runtime::Event {
                            name: "CronTrigger".to_string(),
                            source_agent: "System".to_string(),
                            target_agent: Some(a_name.clone()),
                            payload,
                        }).await;
                    })
                }).map_err(|e| format!("Cron error: {}", e))?
            ).await.map_err(|e| format!("Cron add error: {}", e))?;
            
            println!("{} Scheduled intent {} on {} with cron '{}'", "[System]".blue(), intent_print.magenta(), schedule.trigger_agent.green(), schedule.cron.cyan());
        }

        if !program.schedules.is_empty() {
            sched.start().await.map_err(|e| format!("Cron start error: {}", e))?;
        }

        let timeout_secs = if program.schedules.is_empty() { 2 } else { 3600 * 24 * 365 }; // Wait up to 1 year if daemon
        
        // The Event Loop
        loop {
            // Wait for an event with a timeout. If idle, we break.
            let event = match tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), rx.recv()).await {
                Ok(Ok(ev)) => ev,
                _ => break, // Timeout or channel closed
            };

            // Avoid double printing if we just booted
            if event.source_agent != "System" {
                println!("\n{} Event Triggered: {}", "[EventBus]".yellow(), event.name.bold());
            }

            // Handle OS Cron Events
            if event.name.starts_with("sys_cron_") {
                let parts: Vec<&str> = event.name.split('_').collect();
                if parts.len() == 4 {
                    Box::pin(self.execute_intent(parts[2], parts[3])).await;
                    continue;
                }
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

        for agent_decl in &program.agents {
            let pid = agent_decl.id.clone().unwrap_or_else(|| format!("{}-pid-gen", agent_decl.name));
            let _ = self.audit_logger.log(
                "AgentFinished",
                &pid,
                &format!("Agent {} execution completed", agent_decl.name),
            ).await;
        }

        Ok(())
    }

    async fn execute_event_handler(&mut self, agent_name: &str, handler: &EventHandler, event: &srishti_runtime::Event) {
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Running).await;
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
        
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Waiting).await;
    }

    async fn execute_intent(&mut self, agent_name: &str, intent_name: &str) -> Value {
        let agent_decl = self.ast_program.as_ref().unwrap().agents.iter()
            .find(|a| a.name == agent_name).unwrap();

        let intent = agent_decl.intents.iter().find(|i| i.name == intent_name).unwrap();
        let body = intent.body.clone();
        
        let pid = agent_decl.id.clone().unwrap_or_else(|| format!("{}-pid-gen", agent_name));
        let mock_tokens = 150; // Mock LLM token usage
        if let Err(e) = self.resource_manager.consume_tokens(&pid, mock_tokens).await {
            println!("  {} {}", "[Kernel]".red().bold(), e);
            let _ = self.audit_logger.log(
                "AgentSuspended",
                &pid,
                &format!("Agent {} suspended: {}", agent_name, e),
            ).await;
            return Value::Null;
        }

        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Running).await;
        println!("{} {} executing intent `{}`", "[Agent]".green(), agent_name.bold(), intent_name.cyan());
        
        let mut ctx = HashMap::new();
        let result = Box::pin(self.execute_statements(agent_name, &body, &mut ctx)).await;
        
        self.agents.get_mut(agent_name).unwrap().runtime.set_state(AgentState::Waiting).await;

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
                instance.runtime.emit(event_name.clone(), serde_json::to_value(&payload).unwrap()).await;
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
            Statement::RequireHumanApprovalAbove { limit: _ } => {
                // Usually handled via PolicyEngine, but if used directly in block:
                None
            }
            Statement::AllowRole { role: _ } => {
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
                            
                            let agent_decl = self.ast_program.as_ref().unwrap().agents.iter()
                                .find(|a| a.name == agent_name).unwrap();

                            // OS Permission Check
                            let pid = agent_decl.id.clone().unwrap_or_else(|| format!("{}-pid-gen", agent_name));
                            let has_perm = Box::pin(self.process_table.check_permission(&pid, method)).await;
                            if !has_perm {
                                let _ = self.audit_logger.log(
                                    "ToolFailed",
                                    &pid,
                                    &format!("Agent {} permission denied for tool {}", agent_name, method),
                                ).await;
                                println!("  {} {} OS blocked tool `{}` execution! Lack of permission.", "[Security]".red().bold(), agent_name, method);
                                return Value::String(format!("SecurityError: Permission denied to run tool {}", method));
                            }

                            // OS Policy Check - Using a mock cost for evaluation to prevent dead code
                            if let Some(agent_decl) = self.ast_program.as_ref().unwrap().agents.iter().find(|a| a.name == agent_name) {
                                let pid = agent_decl.id.clone().unwrap_or_else(|| format!("{}-pid-gen", agent_name));
                                for policy_name in &agent_decl.enforced_policies {
                                    let mock_cost = Some(150.0);
                                    let res = self.policy_engine.evaluate(&pid, policy_name, agent_decl.role.as_deref(), mock_cost).await;
                                    match res {
                                        srishti_runtime::policy_engine::PolicyAction::Allow => {}
                                        srishti_runtime::policy_engine::PolicyAction::Deny => {
                                            let _ = self.audit_logger.log(
                                                "PolicyEnforced",
                                                &pid,
                                                &format!("Agent {} policy {} denied tool {}", agent_name, policy_name, method),
                                            ).await;
                                            println!("  {} {} OS blocked tool `{}` execution! Policy '{}' denied action.", "[Security]".red().bold(), agent_name, method, policy_name);
                                            return Value::String(format!("SecurityError: Policy {} denied action", policy_name));
                                        }
                                        srishti_runtime::policy_engine::PolicyAction::SuspendForApproval { limit: _ } => {
                                            let _ = self.audit_logger.log(
                                                "AgentSuspended",
                                                &pid,
                                                &format!("Agent {} policy {} suspended tool {} for human approval", agent_name, policy_name, method),
                                            ).await;
                                            println!("  {} {} OS suspended tool `{}` execution! Policy '{}' requires human approval.", "[Governance]".yellow().bold(), agent_name, method, policy_name);
                                            return Value::String(format!("Suspended: Policy {} requires approval", policy_name));
                                        }
                                    }
                                }
                            }

                            let _ = self.audit_logger.log(
                                "ToolCalled",
                                &pid,
                                &format!("Agent {} executing tool {}", agent_name, method),
                            ).await;
                            return Value::Null; // Mock tool execution
                        } else {
                            return Value::Null;
                        }
                    } else if let Some(endpoint) = Box::pin(self.agent_registry.resolve_endpoint(obj_name)).await {
                        println!("  {} Resolving remote agent {} at {}...", "[Network]".cyan().bold(), obj_name, endpoint);
                        
                        // Pass a dummy payload for now since eval_args are Value types
                        let payload = "{}";
                        
                        // Parse endpoint from srishti://host:port -> http://host:port
                        let http_endpoint = endpoint.replace("srishti://", "http://");
                        
                        match srishti_runtime::network::rpc_client::send_remote_intent(&http_endpoint, obj_name, method, payload).await {
                            Ok(res) => {
                                println!("  {} Remote intent executed successfully", "[Success]".green());
                                return Value::String(res);
                            }
                            Err(e) => {
                                println!("  {} Remote execution failed: {}", "[Error]".red(), e);
                                return Value::String(format!("RPCError: {}", e));
                            }
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
        }
    }
}
