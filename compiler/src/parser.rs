use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<Token> {
        let t = self.peek();
        self.current += 1;
        t
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if let Some(t) = self.advance() {
            if t == expected {
                Ok(())
            } else {
                Err(format!("Expected {:?}, got {:?}", expected, t))
            }
        } else {
            Err(format!("Expected {:?}, got EOF", expected))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        if let Some(Token::Identifier(id)) = self.advance() {
            Ok(id)
        } else {
            Err("Expected identifier".to_string())
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut agents = Vec::new();
        
        while let Some(t) = self.peek() {
            match t {
                Token::Agent => {
                    agents.push(self.parse_agent()?);
                }
                Token::EOF => break,
                _ => return Err(format!("Unexpected top level token {:?}", t)),
            }
        }
        Ok(Program { agents })
    }

    fn parse_agent(&mut self) -> Result<AgentDecl, String> {
        self.expect(Token::Agent)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;
        
        let mut tools = Vec::new();
        let mut guardrails = Vec::new();
        let mut intents = Vec::new();
        let mut memories = Vec::new();

        while let Some(t) = self.peek() {
            if t == Token::CloseBrace {
                self.advance();
                break;
            }
            match t {
                Token::Memory => memories.push(self.parse_memory()?),
                Token::Intent => intents.push(self.parse_intent()?),
                Token::Tool => tools.push(self.parse_tool()?),
                Token::Guardrail => guardrails.push(self.parse_guardrail()?),
                _ => return Err(format!("Unexpected token inside agent: {:?}", t)),
            }
        }
        Ok(AgentDecl { name, tools, guardrails, intents, memories })
    }

    fn parse_memory(&mut self) -> Result<MemoryDecl, String> {
        self.expect(Token::Memory)?;
        let name = self.expect_ident()?;
        Ok(MemoryDecl { name })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let t = self.expect_ident()?;
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
            if t == Token::CloseParen {
                self.advance();
                break;
            }
            let name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let typ = self.parse_type()?;
            args.push(Argument { name, typ });
            // normally would handle commas, for this MVP we might just loop
        }
        Ok(args)
    }

    fn parse_tool(&mut self) -> Result<ToolDecl, String> {
        self.expect(Token::Tool)?;
        let name = self.expect_ident()?;
        let args = self.parse_args()?;
        
        let mut body = None;
        if self.peek() == Some(Token::OpenBrace) {
            self.advance();
            // skip contents for MVP, except CloseBrace
            while let Some(t) = self.peek() {
                if t == Token::CloseBrace {
                    self.advance();
                    break;
                }
                self.advance();
            }
            body = Some(vec![]);
        }

        Ok(ToolDecl { name, args, body })
    }

    fn parse_guardrail(&mut self) -> Result<GuardrailDecl, String> {
        self.expect(Token::Guardrail)?;
        let name = self.expect_ident()?;
        let args = self.parse_args()?;
        self.expect(Token::OpenBrace)?;
        
        let mut body = Vec::new();
        while let Some(t) = self.peek() {
            if t == Token::CloseBrace {
                self.advance();
                break;
            }
            if t == Token::Assert {
                body.push(self.parse_assert()?);
            } else {
                return Err(format!("Unexpected token inside guardrail: {:?}", t));
            }
        }
        Ok(GuardrailDecl { name, args, body })
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
        if self.peek() == Some(Token::Else) {
            self.advance();
            if let Some(Token::StringLiteral(s)) = self.advance() {
                else_action = Some(s);
            } else {
                return Err("Expected string literal after else".to_string());
            }
        }

        Ok(Statement::Assert { condition, else_action })
    }

    fn parse_operator(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::OpLessThanOrEqual) => Ok("<=".to_string()),
            Some(Token::OpGreaterThanOrEqual) => Ok(">=".to_string()),
            Some(Token::OpLessThan) => Ok("<".to_string()),
            Some(Token::OpGreaterThan) => Ok(">".to_string()),
            Some(Token::OpEquals) => Ok("==".to_string()),
            Some(Token::OpNotEquals) => Ok("!=".to_string()),
            _ => Err("Expected comparison operator".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Some(Token::Identifier(id)) => Ok(Expression::Variable(id)),
            Some(Token::FloatLiteral(f)) => Ok(Expression::LiteralFloat(f)),
            Some(Token::StringLiteral(s)) => Ok(Expression::LiteralString(s)),
            _ => Err("Expected expression".to_string()),
        }
    }

    fn parse_intent(&mut self) -> Result<IntentDecl, String> {
        self.expect(Token::Intent)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;
        
        let mut body = Vec::new();
        while let Some(t) = self.peek() {
            if t == Token::CloseBrace {
                self.advance();
                break;
            }
            if t == Token::Achieve {
                self.advance(); // consume 'achieve'
                if let Some(Token::StringLiteral(s)) = self.advance() {
                    body.push(Statement::Achieve { goal: s });
                } else {
                    return Err("Expected string literal after achieve".to_string());
                }
            } else {
                return Err(format!("Unexpected token inside intent: {:?}", t));
            }
        }
        Ok(IntentDecl { name, body })
    }
}
