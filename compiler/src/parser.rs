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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let t = self.peek().cloned();
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
            Ok(id.clone())
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

    fn parse_agent(&mut self) -> Result<Agent, String> {
        self.expect(Token::Agent)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenBrace)?;
        
        let mut tools = Vec::new();
        let mut guardrails = Vec::new();
        let mut intents = Vec::new();
        let mut persistent_memory = Vec::new();

        while let Some(t) = self.peek() {
            if *t == Token::CloseBrace {
                self.advance();
                break;
            }
            
            match t {
                Token::Intent => intents.push(self.parse_intent()?),
                Token::Tool => tools.push(self.parse_tool()?),
                Token::Guardrail => guardrails.push(self.parse_guardrail()?),
                Token::Persistent => persistent_memory.push(self.parse_persistent_memory()?),
                _ => return Err(format!("Unexpected token inside agent: {:?}", t)),
            }
        }
        
        Ok(Agent {
            name,
            tools,
            guardrails,
            intents,
            persistent_memory,
        })
    }
    
    // Stub methods to keep it brief for MVP
    fn parse_intent(&mut self) -> Result<Intent, String> {
        self.expect(Token::Intent)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenParen)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseParen { break; }
            self.advance();
        }
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseBrace { break; }
            self.advance();
        }
        self.expect(Token::CloseBrace)?; 
        Ok(Intent { name, args: vec![], body: vec![] })
    }

    fn parse_tool(&mut self) -> Result<Tool, String> {
        self.expect(Token::Tool)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenParen)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseParen { break; }
            self.advance();
        }
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseBrace { break; }
            self.advance();
        }
        self.expect(Token::CloseBrace)?; 
        Ok(Tool { name, args: vec![], return_type: None, body: vec![] })
    }

    fn parse_guardrail(&mut self) -> Result<Guardrail, String> {
        self.expect(Token::Guardrail)?;
        let name = self.expect_ident()?;
        self.expect(Token::OpenParen)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseParen { break; }
            self.advance();
        }
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        while let Some(t) = self.peek() {
            if *t == Token::CloseBrace { break; }
            self.advance();
        }
        self.expect(Token::CloseBrace)?; 
        Ok(Guardrail { name, args: vec![], body: vec![] })
    }

    fn parse_persistent_memory(&mut self) -> Result<Statement, String> {
        self.expect(Token::Persistent)?;
        self.expect(Token::Memory)?;
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        self.expect_ident()?; // type
        self.expect(Token::LessThan)?;
        self.expect_ident()?; // inner type
        self.expect(Token::GreaterThan)?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::LetDecl {
            name,
            is_persistent: true,
            var_type: None, // skip parsing type detail for stub
            value: None,
        })
    }
}
