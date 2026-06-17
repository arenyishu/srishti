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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(String),
    LiteralFloat(f64),
    LiteralString(String),
    BinaryOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
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
    RawRust(String), // For // deterministic block
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryDecl {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolDecl {
    pub name: String,
    pub args: Vec<Argument>,
    pub body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuardrailDecl {
    pub name: String,
    pub args: Vec<Argument>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntentDecl {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentDecl {
    pub name: String,
    pub memories: Vec<MemoryDecl>,
    pub tools: Vec<ToolDecl>,
    pub guardrails: Vec<GuardrailDecl>,
    pub intents: Vec<IntentDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub agents: Vec<AgentDecl>,
}
