use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Agent, Memory, Tool, Guardrail, Intent, Assert, Achieve, Else,
    Import, From, As, Workflow, Message, Channel, On, Emit,
    State, Transition, Fn, Let, Return, If, True, False,
    Policy, Role, Permission, Use, Secret, Id, Scope, Retention, Deletion, Encryption, Index, Allow,
    Enforce, Schedule, Cron, Trigger, Monitor, AlertOnFailure, Audit, Version, RequireHumanApprovalAbove,
    Quota, MemoryMb, CpuPercent, TokensPerHour, Endpoint,
    
    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),
    IntegerLiteral(i64),

    Colon, OpenBrace, CloseBrace, OpenParen, CloseParen,
    Comma, Dot, Semicolon, Star, Arrow,

    OpLessThanOrEqual, OpGreaterThanOrEqual, OpLessThan, OpGreaterThan,
    OpEquals, OpNotEquals, Equals,

    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
            offset: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.next()?;
        self.offset += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), String> {
        loop {
            match self.peek() {
                Some(&c) if c.is_whitespace() => {
                    self.advance();
                }
                Some(&'/') => {
                    self.advance(); // consume first /
                    if self.peek() == Some(&'/') {
                        // Single-line comment
                        while let Some(ch) = self.advance() {
                            if ch == '\n' { break; }
                        }
                    } else {
                        return Err(format!("Unexpected character '/' at line {}", self.line));
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    pub fn next_token(&mut self) -> Result<SpannedToken, String> {
        self.skip_whitespace_and_comments()?;

        let span = Span::new(self.line, self.column, self.offset);

        if let Some(&c) = self.peek() {
            let token = match c {
                '{' => { self.advance(); Token::OpenBrace }
                '}' => { self.advance(); Token::CloseBrace }
                '(' => { self.advance(); Token::OpenParen }
                ')' => { self.advance(); Token::CloseParen }
                ':' => { self.advance(); Token::Colon }
                ',' => { self.advance(); Token::Comma }
                ';' => { self.advance(); Token::Semicolon }
                '.' => { self.advance(); Token::Dot }
                '*' => { self.advance(); Token::Star }
                '-' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        return Err(format!("Expected ->, got - at line {}", self.line));
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::OpLessThanOrEqual
                    } else {
                        Token::OpLessThan
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::OpGreaterThanOrEqual
                    } else {
                        Token::OpGreaterThan
                    }
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::OpEquals
                    } else {
                        Token::Equals
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::OpNotEquals
                    } else {
                        return Err(format!("Expected !=, got ! at line {}", self.line));
                    }
                }
                '"' => {
                    self.advance(); // consume quote
                    let mut string = String::new();
                    while let Some(ch) = self.advance() {
                        if ch == '"' { break; }
                        string.push(ch);
                    }
                    Token::StringLiteral(string)
                }
                c if c.is_ascii_digit() => {
                    let mut num = String::new();
                    let mut is_float = false;
                    while let Some(&ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            num.push(ch);
                            self.advance();
                        } else if ch == '.' {
                            is_float = true;
                            num.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        Token::FloatLiteral(num.parse().unwrap_or(0.0))
                    } else {
                        Token::IntegerLiteral(num.parse().unwrap_or(0))
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(&ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "agent" => Token::Agent,
                        "memory" => Token::Memory,
                        "tool" => Token::Tool,
                        "guardrail" => Token::Guardrail,
                        "intent" => Token::Intent,
                        "assert" => Token::Assert,
                        "achieve" => Token::Achieve,
                        "else" => Token::Else,
                        "import" => Token::Import,
                        "from" => Token::From,
                        "as" => Token::As,
                        "workflow" => Token::Workflow,
                        "message" => Token::Message,
                        "channel" => Token::Channel,
                        "on" => Token::On,
                        "emit" => Token::Emit,
                        "state" => Token::State,
                        "transition" => Token::Transition,
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "return" => Token::Return,
                        "if" => Token::If,
                        "true" => Token::True,
                        "false" => Token::False,
                        "policy" => Token::Policy,
                        "role" => Token::Role,
                        "permission" => Token::Permission,
                        "use" => Token::Use,
                        "secret" => Token::Secret,
                        "id" => Token::Id,
                        "scope" => Token::Scope,
                        "retention" => Token::Retention,
                        "deletion" => Token::Deletion,
                        "encryption" => Token::Encryption,
                        "index" => Token::Index,
                        "allow" => Token::Allow,
                        "enforce" => Token::Enforce,
                        "schedule" => Token::Schedule,
                        "cron" => Token::Cron,
                        "trigger" => Token::Trigger,
                        "monitor" => Token::Monitor,
                        "alert_on_failure" => Token::AlertOnFailure,
                        "audit" => Token::Audit,
                        "version" => Token::Version,
                        "require_human_approval_above" => Token::RequireHumanApprovalAbove,
                        "quota" => Token::Quota,
                        "memory_mb" => Token::MemoryMb,
                        "cpu_percent" => Token::CpuPercent,
                        "tokens_per_hour" => Token::TokensPerHour,
                        "endpoint" => Token::Endpoint,
                        _ => Token::Identifier(ident),
                    }
                }
                _ => return Err(format!("Unexpected character: {}", c)),
            };
            Ok(SpannedToken { token, span })
        } else {
            Ok(SpannedToken { token: Token::EOF, span })
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = tok.token == Token::EOF;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}
