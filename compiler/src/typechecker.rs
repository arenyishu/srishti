use crate::ast::*;
use crate::lexer::Span;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub code: String,
    pub suggestion: Option<String>,
}

pub struct TypeChecker {
    diagnostics: Vec<Diagnostic>,
    agent_names: HashSet<String>,
    pid_names: HashSet<String>,
    policy_names: HashSet<String>,
    schedule_names: HashSet<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            agent_names: HashSet::new(),
            pid_names: HashSet::new(),
            policy_names: HashSet::new(),
            schedule_names: HashSet::new(),
        }
    }

    pub fn check(mut self, program: &Program) -> Vec<Diagnostic> {
        for policy in &program.policies {
            if !self.policy_names.insert(policy.name.clone()) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("duplicate policy name `{}`", policy.name),
                    span: policy.span.clone(),
                    code: "E010".to_string(),
                    suggestion: Some("rename the policy".to_string()),
                });
            }
        }

        for schedule in &program.schedules {
            if !self.schedule_names.insert(schedule.name.clone()) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("duplicate schedule name `{}`", schedule.name),
                    span: schedule.span.clone(),
                    code: "E011".to_string(),
                    suggestion: Some("rename the schedule".to_string()),
                });
            }
            if schedule.cron.split_whitespace().count() != 5 {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("invalid cron expression `{}`: must have 5 fields", schedule.cron),
                    span: schedule.span.clone(),
                    code: "E012".to_string(),
                    suggestion: Some("use standard 5-field cron syntax".to_string()),
                });
            }
        }

        for agent in &program.agents {
            self.check_agent(agent);
        }
        self.diagnostics
    }

    fn check_agent(&mut self, agent: &AgentDecl) {
        if !self.agent_names.insert(agent.name.clone()) {
            self.diagnostics.push(Diagnostic {
                severity: Severity::Error,
                message: format!("duplicate agent name `{}`", agent.name),
                span: agent.span.clone(),
                code: "E001".to_string(),
                suggestion: Some("rename one of the agents".to_string()),
            });
        }

        if let Some(id) = &agent.id {
            if !self.pid_names.insert(id.clone()) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("duplicate agent PID `{}`", id),
                    span: agent.span.clone(),
                    code: "E014".to_string(),
                    suggestion: Some("change the agent's ID to be unique".to_string()),
                });
            }
        }

        if let Some(quota) = &agent.quota {
            if let Some(cpu) = quota.cpu_percent {
                if cpu < 1 || cpu > 100 {
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        message: format!("invalid cpu_percent `{}` in agent `{}`", cpu, agent.name),
                        span: quota.span.clone(),
                        code: "E015".to_string(),
                        suggestion: Some("cpu_percent must be between 1 and 100".to_string()),
                    });
                }
            }
            if let Some(mem) = quota.memory_mb {
                if mem < 1 {
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        message: format!("invalid memory_mb `{}` in agent `{}`", mem, agent.name),
                        span: quota.span.clone(),
                        code: "E016".to_string(),
                        suggestion: Some("memory_mb must be greater than 0".to_string()),
                    });
                }
            }
            if let Some(tok) = quota.tokens_per_hour {
                if tok < 1 {
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        message: format!("invalid tokens_per_hour `{}` in agent `{}`", tok, agent.name),
                        span: quota.span.clone(),
                        code: "E017".to_string(),
                        suggestion: Some("tokens_per_hour must be greater than 0".to_string()),
                    });
                }
            }
        }

        if let Some(endpoint) = &agent.endpoint {
            if !endpoint.starts_with("srishti://") {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("invalid endpoint `{}` in agent `{}`", endpoint, agent.name),
                    span: agent.span.clone(),
                    code: "E018".to_string(),
                    suggestion: Some("endpoints must start with srishti://".to_string()),
                });
            }
        }

        for policy_ref in &agent.enforced_policies {
            if !self.policy_names.contains(policy_ref) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("agent `{}` references undefined policy `{}`", agent.name, policy_ref),
                    span: agent.span.clone(),
                    code: "E013".to_string(),
                    suggestion: Some("define the policy first".to_string()),
                });
            }
        }

        let mut tool_names = HashSet::new();
        for tool in &agent.tools {
            if !tool_names.insert(tool.name.clone()) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("duplicate tool name `{}` in agent `{}`", tool.name, agent.name),
                    span: tool.span.clone(),
                    code: "E002".to_string(),
                    suggestion: Some("rename the tool".to_string()),
                });
            }
        }

        let mut memory_names = HashSet::new();
        for mem in &agent.memories {
            if !memory_names.insert(mem.name.clone()) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: format!("duplicate memory name `{}` in agent `{}`", mem.name, agent.name),
                    span: mem.span.clone(),
                    code: "E003".to_string(),
                    suggestion: Some("rename the memory".to_string()),
                });
            }
        }

        for intent in &agent.intents {
            if intent.body.is_empty() {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    message: format!("empty intent `{}`", intent.name),
                    span: intent.span.clone(),
                    code: "W002".to_string(),
                    suggestion: Some("add `achieve` statements".to_string()),
                });
            }
        }
    }
}
