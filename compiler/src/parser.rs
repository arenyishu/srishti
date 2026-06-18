use crate::ast::*;
use crate::lexer::{SpannedToken, Token, Span};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<SpannedToken> {
        self.tokens.get(self.current).cloned()
    }

    fn peek_token(&self) -> Option<Token> {
        self.peek().map(|t| t.token)
    }

    fn current_span(&self) -> Span {
        self.peek().map(|t| t.span).unwrap_or_default()
    }

    fn advance(&mut self) -> Option<SpannedToken> {
        let t = self.peek();
        self.current += 1;
        t
    }

    fn expect(&mut self, expected: Token) -> Result<Span, String> {
        if let Some(t) = self.advance() {
            if t.token == expected {
                Ok(t.span)
            } else {
                Err(format!("Expected {:?}, got {:?} at line {}", expected, t.token, t.span.line))
            }
        } else {
            Err(format!("Expected {:?}, got EOF", expected))
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), String> {
        if let Some(t) = self.advance() {
            if let Token::Identifier(id) = t.token {
                Ok((id, t.span))
            } else {
                Err(format!("Expected identifier, got {:?} at line {}", t.token, t.span.line))
            }
        } else {
            Err("Expected identifier, got EOF".to_string())
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut agents = Vec::new();
        let mut imports = Vec::new();
        let mut messages = Vec::new();
        let mut policies = Vec::new();
        let mut workflows = Vec::new();
        let start_span = self.current_span();

        while let Some(t) = self.peek() {
            match t.token {
                Token::Import => imports.push(self.parse_import()?),
                Token::Message => messages.push(self.parse_message()?),
                Token::Policy => policies.push(self.parse_policy()?),
                Token::Workflow => workflows.push(self.parse_workflow()?),
                Token::Agent => agents.push(self.parse_agent()?),
                Token::EOF => break,
                _ => return Err(format!("Unexpected top level token {:?} at line {}", t.token, t.span.line)),
            }
        }
        
        let mut end_span = self.current_span();
        if self.current > 0 {
            end_span = self.tokens[self.current - 1].span.clone();
        }

        Ok(Program {
            imports,
            messages,
            policies,
            agents,
            workflows,
            span: Span::new(start_span.line, start_span.column, start_span.offset),
        })
    }

    fn parse_import(&mut self) -> Result<ImportDecl, String> {
        let start_span = self.expect(Token::Import)?;
        
        let mut items = Vec::new();
        let mut path = String::new();
        let mut alias = None;

        // Either `import * from "..."` or `import Agent from "..."`
        if self.peek_token() == Some(Token::Star) {
            self.advance();
            self.expect(Token::From)?;
            if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                path = s;
            } else {
                return Err("Expected string literal after 'from'".to_string());
            }
        } else {
            let (id, _) = self.expect_ident()?;
            items.push(id);
            self.expect(Token::From)?;
            if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                path = s;
            } else {
                return Err("Expected string literal after 'from'".to_string());
            }
        }

        if self.peek_token() == Some(Token::As) {
            self.advance();
            let (id, _) = self.expect_ident()?;
            alias = Some(id);
        }

        Ok(ImportDecl {
            path,
            items,
            alias,
            span: start_span,
        })
    }

    fn parse_message(&mut self) -> Result<MessageDecl, String> {
        let start_span = self.expect(Token::Message)?;
        let (name, _) = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;
        
        let mut fields = Vec::new();
        while self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::EOF) {
            let (field_name, field_span) = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let typ = self.parse_type()?;
            fields.push(Argument { name: field_name, typ, span: field_span });
            
            // Optional comma
            if self.peek_token() == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::CloseBrace)?;

        Ok(MessageDecl { name, fields, span: start_span })
    }

    fn parse_workflow(&mut self) -> Result<WorkflowDecl, String> {
        let start_span = self.expect(Token::Workflow)?;
        let (name, _) = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;

        let mut steps = Vec::new();
        while self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::EOF) {
            let (step_name, step_span) = self.expect_ident()?;
            self.expect(Token::Arrow)?;
            
            let (agent, _) = self.expect_ident()?;
            self.expect(Token::Dot)?;
            let (intent, _) = self.expect_ident()?;
            
            steps.push(WorkflowStep {
                name: step_name,
                agent,
                intent,
                span: step_span,
            });

            if self.peek_token() == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::CloseBrace)?;

        Ok(WorkflowDecl { name, steps, span: start_span })
    }

    fn parse_policy(&mut self) -> Result<PolicyDecl, String> {
        let span = self.expect(Token::Policy)?;
        let (name, _) = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_statements()?;
        Ok(PolicyDecl { name, body, span })
    }

    fn parse_dotted_ident(&mut self) -> Result<String, String> {
        let mut res = String::new();
        let (id, _) = self.expect_ident()?;
        res.push_str(&id);
        while self.peek_token() == Some(Token::Dot) {
            self.advance();
            let (id, _) = self.expect_ident()?;
            res.push('.');
            res.push_str(&id);
        }
        Ok(res)
    }

    fn parse_agent(&mut self) -> Result<AgentDecl, String> {
        let start_span = self.expect(Token::Agent)?;
        let (name, _) = self.expect_ident()?;
        
        let mut role = None;
        if self.peek_token() == Some(Token::Role) {
            self.advance();
            let (r, _) = self.expect_ident()?;
            role = Some(r);
        }
        
        self.expect(Token::OpenBrace)?;

        let mut id = None;
        let mut permissions = Vec::new();
        let mut secrets = Vec::new();
        let mut tools = Vec::new();
        let mut guardrails = Vec::new();
        let mut intents = Vec::new();
        let mut memories = Vec::new();
        let mut states = Vec::new();
        let mut event_handlers = Vec::new();

        while let Some(t) = self.peek() {
            if t.token == Token::CloseBrace {
                self.advance();
                break;
            }
            match t.token {
                Token::Id => {
                    self.advance();
                    self.expect(Token::Colon)?;
                    if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                        id = Some(s);
                    } else {
                        return Err("Expected string literal for id".to_string());
                    }
                }
                Token::Permission => {
                    self.advance();
                    permissions.push(self.parse_dotted_ident()?);
                }
                Token::Use => {
                    self.advance();
                    self.expect(Token::Secret)?;
                    let (sec, _) = self.expect_ident()?;
                    secrets.push(sec);
                }
                Token::Memory => memories.push(self.parse_memory()?),
                Token::Intent => intents.push(self.parse_intent()?),
                Token::Tool => tools.push(self.parse_tool()?),
                Token::Guardrail => guardrails.push(self.parse_guardrail()?),
                Token::State => states.push(self.parse_state()?),
                Token::On => event_handlers.push(self.parse_event_handler()?),
                _ => return Err(format!("Unexpected token inside agent: {:?} at line {}", t.token, t.span.line)),
            }
        }
        Ok(AgentDecl {
            name,
            id,
            role,
            permissions,
            secrets,
            tools,
            guardrails,
            intents,
            memories,
            states,
            event_handlers,
            span: start_span,
        })
    }

    fn parse_state(&mut self) -> Result<StateDecl, String> {
        let start_span = self.expect(Token::State)?;
        let (name, _) = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;

        let mut transitions = Vec::new();
        while self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::EOF) {
            let (from, t_span) = self.expect_ident()?;
            self.expect(Token::Arrow)?;
            let (to, _) = self.expect_ident()?;
            self.expect(Token::Colon)?;
            self.expect(Token::On)?;
            let (on_event, _) = self.expect_ident()?;

            transitions.push(TransitionDecl {
                from,
                to,
                on_event,
                span: t_span,
            });

            if self.peek_token() == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::CloseBrace)?;

        Ok(StateDecl { name, transitions, span: start_span })
    }

    fn parse_event_handler(&mut self) -> Result<EventHandler, String> {
        let start_span = self.expect(Token::On)?;
        let (event_name, _) = self.expect_ident()?;
        let params = if self.peek_token() == Some(Token::OpenParen) {
            self.parse_args()?
        } else {
            Vec::new()
        };
        
        self.expect(Token::OpenBrace)?;
        let body = self.parse_statements()?;

        Ok(EventHandler { event_name, params, body, span: start_span })
    }

    fn parse_memory(&mut self) -> Result<MemoryDecl, String> {
        let span = self.expect(Token::Memory)?;
        let (name, _) = self.expect_ident()?;
        
        let mut scope = None;
        let mut retention = None;
        let mut deletion = None;
        let mut encryption = None;
        let mut index = None;

        if self.peek_token() == Some(Token::OpenBrace) {
            self.expect(Token::OpenBrace)?;
            while self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::EOF) {
                let t = self.peek_token().unwrap();
                match t {
                    Token::Scope => {
                        self.advance();
                        self.expect(Token::Colon)?;
                        let (val, _) = self.expect_ident()?;
                        scope = Some(val);
                    }
                    Token::Retention => {
                        self.advance();
                        self.expect(Token::Colon)?;
                        if let Some(SpannedToken { token: Token::IntegerLiteral(n), .. }) = self.advance() {
                            if let Some(SpannedToken { token: Token::Identifier(id), .. }) = self.peek() {
                                self.advance(); // consume it
                                retention = Some(format!("{}{}", n, id));
                            } else {
                                retention = Some(n.to_string());
                            }
                        } else {
                            return Err("Expected integer literal for retention".to_string());
                        }
                    }
                    Token::Deletion => {
                        self.advance();
                        self.expect(Token::Colon)?;
                        let (val, _) = self.expect_ident()?;
                        deletion = Some(val);
                    }
                    Token::Encryption => {
                        self.advance();
                        self.expect(Token::Colon)?;
                        let val = self.parse_dotted_ident()?; // handles aes_256_gcm (which is actually just identifiers with underscores, expect_ident handles it!)
                        encryption = Some(val);
                    }
                    Token::Index => {
                        self.advance();
                        self.expect(Token::Colon)?;
                        let (val, _) = self.expect_ident()?;
                        index = Some(val);
                    }
                    _ => return Err(format!("Unexpected token in memory block: {:?}", t)),
                }
            }
            self.expect(Token::CloseBrace)?;
        }

        Ok(MemoryDecl { name, scope, retention, deletion, encryption, index, span })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let (t, _) = self.expect_ident()?;
        match t.as_str() {
            "Float" => Ok(Type::Float),
            "String" => Ok(Type::String),
            "Boolean" => Ok(Type::Boolean),
            "Integer" => Ok(Type::Integer),
            _ => Ok(Type::Custom(t)),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Argument>, String> {
        self.expect(Token::OpenParen)?;
        let mut args = Vec::new();
        while let Some(t) = self.peek() {
            if t.token == Token::CloseParen {
                self.advance();
                break;
            }
            let (name, span) = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let typ = self.parse_type()?;
            args.push(Argument { name, typ, span });
            
            if self.peek_token() == Some(Token::Comma) {
                self.advance();
            } else if self.peek_token() != Some(Token::CloseParen) {
                return Err("Expected comma or ) in arguments".to_string());
            }
        }
        Ok(args)
    }

    fn parse_tool(&mut self) -> Result<ToolDecl, String> {
        let span = self.expect(Token::Tool)?;
        let (name, _) = self.expect_ident()?;
        let args = self.parse_args()?;

        let mut body = None;
        if self.peek_token() == Some(Token::OpenBrace) {
            self.expect(Token::OpenBrace)?;
            body = Some(self.parse_statements()?);
        }

        Ok(ToolDecl { name, args, body, span })
    }

    fn parse_guardrail(&mut self) -> Result<GuardrailDecl, String> {
        let span = self.expect(Token::Guardrail)?;
        let (name, _) = self.expect_ident()?;
        let args = self.parse_args()?;
        self.expect(Token::OpenBrace)?;

        let mut body = Vec::new();
        while let Some(t) = self.peek() {
            if t.token == Token::CloseBrace {
                self.advance();
                break;
            }
            if t.token == Token::Assert {
                body.push(self.parse_assert()?);
            } else {
                return Err(format!("Unexpected token inside guardrail: {:?}", t.token));
            }
        }
        Ok(GuardrailDecl { name, args, body, span })
    }

    fn parse_assert(&mut self) -> Result<Statement, String> {
        self.expect(Token::Assert)?;
        let left = self.parse_expression()?;
        let op = self.parse_operator()?;
        let right = self.parse_expression()?;

        let condition = Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };

        let mut else_action = None;
        if self.peek_token() == Some(Token::Else) {
            self.advance();
            if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                else_action = Some(s);
            } else {
                return Err("Expected string literal after else".to_string());
            }
        }

        Ok(Statement::Assert {
            condition,
            else_action,
        })
    }

    fn parse_operator(&mut self) -> Result<String, String> {
        match self.advance().map(|t| t.token) {
            Some(Token::OpLessThanOrEqual) => Ok("<=".to_string()),
            Some(Token::OpGreaterThanOrEqual) => Ok(">=".to_string()),
            Some(Token::OpLessThan) => Ok("<".to_string()),
            Some(Token::OpGreaterThan) => Ok(">".to_string()),
            Some(Token::OpEquals) => Ok("==".to_string()),
            Some(Token::OpNotEquals) => Ok("!=".to_string()),
            Some(Token::Equals) => Ok("=".to_string()),
            _ => Err("Expected operator".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        // Very basic expression parsing. MVP.
        let mut expr = match self.advance().map(|t| t.token) {
            Some(Token::Identifier(id)) => Expression::Variable(id),
            Some(Token::FloatLiteral(f)) => Expression::LiteralFloat(f),
            Some(Token::IntegerLiteral(i)) => Expression::LiteralInt(i),
            Some(Token::StringLiteral(s)) => Expression::LiteralString(s),
            Some(Token::True) => Expression::BooleanLiteral(true),
            Some(Token::False) => Expression::BooleanLiteral(false),
            _ => return Err("Expected expression".to_string()),
        };

        // Handle dot access (methods / fields)
        while self.peek_token() == Some(Token::Dot) {
            self.advance();
            let (field, _) = self.expect_ident()?;
            
            if self.peek_token() == Some(Token::OpenParen) {
                // Method call
                self.expect(Token::OpenParen)?;
                let mut args = Vec::new();
                while self.peek_token() != Some(Token::CloseParen) && self.peek_token() != Some(Token::EOF) {
                    args.push(self.parse_expression()?);
                    if self.peek_token() == Some(Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(Token::CloseParen)?;
                expr = Expression::MethodCall {
                    object: Box::new(expr),
                    method: field,
                    args,
                };
            } else {
                // Field access
                expr = Expression::FieldAccess {
                    object: Box::new(expr),
                    field,
                };
            }
        }

        // Handle function call on the identifier itself (e.g., classify_ticket())
        if self.peek_token() == Some(Token::OpenParen) {
            if let Expression::Variable(name) = expr.clone() {
                self.expect(Token::OpenParen)?;
                let mut args = Vec::new();
                while self.peek_token() != Some(Token::CloseParen) && self.peek_token() != Some(Token::EOF) {
                    args.push(self.parse_expression()?);
                    if self.peek_token() == Some(Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(Token::CloseParen)?;
                
                // Represent function call as a method call on a dummy object, or just a special AST node?
                // Wait, Srishti AST doesn't have FunctionCall. I'll just use a Variable, but the parser will consume the parens, 
                // OR I can use a MethodCall where object is the Variable.
                // Let's use a MethodCall with object=Variable("self"), method=name.
                expr = Expression::MethodCall {
                    object: Box::new(Expression::Variable("self".to_string())),
                    method: name,
                    args,
                };
            }
        }

        Ok(expr)
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = Vec::new();
        while self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::EOF) {
            stmts.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek_token() {
            Some(Token::Let) => {
                self.advance();
                let (name, _) = self.expect_ident()?;
                self.expect(Token::Equals)?;
                let value = self.parse_expression()?;
                Ok(Statement::LetBinding { name, value: Box::new(value) })
            }
            Some(Token::Return) => {
                self.advance();
                let mut val = None;
                // If the next token isn't a closing brace/semicolon, try to parse expr
                if self.peek_token() != Some(Token::CloseBrace) && self.peek_token() != Some(Token::Semicolon) {
                    val = Some(Box::new(self.parse_expression()?));
                }
                Ok(Statement::ReturnStmt { value: val })
            }
            Some(Token::Emit) => {
                self.advance();
                let (event_name, _) = self.expect_ident()?;
                let mut args = Vec::new();
                if self.peek_token() == Some(Token::OpenParen) {
                    self.expect(Token::OpenParen)?;
                    while self.peek_token() != Some(Token::CloseParen) && self.peek_token() != Some(Token::EOF) {
                        args.push(self.parse_expression()?);
                        if self.peek_token() == Some(Token::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(Token::CloseParen)?;
                }
                Ok(Statement::EmitEvent { event_name, args })
            }
            Some(Token::If) => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(Token::OpenBrace)?;
                let then_body = self.parse_statements()?;
                let mut else_body = None;
                if self.peek_token() == Some(Token::Else) {
                    self.advance();
                    self.expect(Token::OpenBrace)?;
                    else_body = Some(self.parse_statements()?);
                }
                Ok(Statement::IfStmt {
                    condition: Box::new(condition),
                    then_body,
                    else_body,
                })
            }
            Some(Token::Allow) => {
                self.advance();
                self.expect(Token::Role)?;
                let (role, _) = self.expect_ident()?;
                Ok(Statement::AllowRole { role })
            }
            Some(Token::Achieve) => {
                self.advance();
                if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                    Ok(Statement::Achieve { goal: s })
                } else {
                    Err("Expected string literal after achieve".to_string())
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::ExprStmt(expr))
            }
        }
    }

    fn parse_intent(&mut self) -> Result<IntentDecl, String> {
        let span = self.expect(Token::Intent)?;
        let (name, _) = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;

        let mut body = Vec::new();
        while let Some(t) = self.peek() {
            if t.token == Token::CloseBrace {
                self.advance();
                break;
            }
            if t.token == Token::Achieve {
                self.advance(); // consume 'achieve'
                if let Some(SpannedToken { token: Token::StringLiteral(s), .. }) = self.advance() {
                    body.push(Statement::Achieve { goal: s });
                } else {
                    return Err("Expected string literal after achieve".to_string());
                }
            } else {
                return Err(format!("Unexpected token inside intent: {:?}", t.token));
            }
        }
        Ok(IntentDecl { name, body, span })
    }
}
