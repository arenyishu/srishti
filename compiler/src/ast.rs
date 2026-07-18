use crate::lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    String,
    Boolean,
    Integer,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub typ: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(String),
    LiteralFloat(f64),
    LiteralString(String),
    LiteralInt(i64),
    BooleanLiteral(bool),
    BinaryOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assert {
        condition: Expression,
        else_action: Option<String>,
    },
    Achieve {
        goal: String,
    },
    RawRust(String),
    LetBinding {
        name: String,
        value: Box<Expression>,
    },
    ReturnStmt {
        value: Option<Box<Expression>>,
    },
    IfStmt {
        condition: Box<Expression>,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    EmitEvent {
        event_name: String,
        args: Vec<Expression>,
    },
    RequireHumanApprovalAbove {
        limit: f64,
    },
    AllowRole {
        role: String,
    },
    ExprStmt(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryDecl {
    pub name: String,
    pub typ: Option<String>,
    pub storage: Option<String>,
    pub scope: Option<String>,
    pub retention: Option<String>,
    pub deletion: Option<String>,
    pub encryption: Option<String>,
    pub index: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyDecl {
    pub name: String,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolDecl {
    pub name: String,
    pub args: Vec<Argument>,
    pub body: Option<Vec<Statement>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuardrailDecl {
    pub name: String,
    pub args: Vec<Argument>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntentDecl {
    pub name: String,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub path: String,
    pub items: Vec<String>,
    pub alias: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageDecl {
    pub name: String,
    pub fields: Vec<Argument>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowDecl {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowStep {
    pub name: String,
    pub agent: String,
    pub intent: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateDecl {
    pub name: String,
    pub transitions: Vec<TransitionDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionDecl {
    pub from: String,
    pub to: String,
    pub on_event: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventHandler {
    pub event_name: String,
    pub params: Vec<Argument>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleDecl {
    pub name: String,
    pub cron: String,
    pub trigger_agent: String,
    pub trigger_intent: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuotaDecl {
    pub memory_mb: Option<i64>,
    pub cpu_percent: Option<i64>,
    pub tokens_per_hour: Option<i64>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentDecl {
    pub name: String,
    pub id: Option<String>,
    pub version: Option<String>,
    pub role: Option<String>,
    pub enforced_policies: Vec<String>,
    pub permissions: Vec<String>,
    pub audit: Option<String>,
    pub monitor: Vec<String>,
    pub alert_on_failure: Option<String>,
    pub secrets: Vec<String>,
    pub quota: Option<QuotaDecl>,
    pub endpoint: Option<String>,
    pub memories: Vec<MemoryDecl>,
    pub tools: Vec<ToolDecl>,
    pub guardrails: Vec<GuardrailDecl>,
    pub intents: Vec<IntentDecl>,
    pub states: Vec<StateDecl>,
    pub event_handlers: Vec<EventHandler>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub imports: Vec<ImportDecl>,
    pub messages: Vec<MessageDecl>,
    pub policies: Vec<PolicyDecl>,
    pub schedules: Vec<ScheduleDecl>,
    pub agents: Vec<AgentDecl>,
    pub workflows: Vec<WorkflowDecl>,
    pub span: Span,
}
