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
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            agent_names: HashSet::new(),
        }
    }

    pub fn check(mut self, program: &Program) -> Vec<Diagnostic> {
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
