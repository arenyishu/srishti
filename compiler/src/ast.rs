#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Float,
    Integer,
    Boolean,
    Vector(Box<Type>),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    LiteralString(String),
    LiteralFloat(f64),
    LiteralInt(i64),
    Variable(String),
    Extract {
        target_type: String,
        source_expr: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Achieve(String), // e.g., achieve "Find the best flight under $500"
    BinaryOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetDecl {
        name: String,
        is_persistent: bool,
        var_type: Option<Type>,
        value: Option<Expression>,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Expression(Expression),
    Return(Expression),
    Assert {
        condition: Expression,
        else_action: String, // e.g., "trigger human_fallback"
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Guardrail {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Agent {
    pub name: String,
    pub tools: Vec<Tool>,
    pub guardrails: Vec<Guardrail>,
    pub intents: Vec<Intent>,
    pub persistent_memory: Vec<Statement>, // let statements for memory
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub agents: Vec<Agent>,
    // we could also have top-level tools, guardrails, structs, etc.
}
