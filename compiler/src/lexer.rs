use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Agent,
    Memory,
    Tool,
    Guardrail,
    Intent,
    Assert,
    Achieve,
    Else,

    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),

    Colon,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,

    OpLessThanOrEqual,
    OpGreaterThanOrEqual,
    OpLessThan,
    OpGreaterThan,
    OpEquals,
    OpNotEquals,

    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(&c) if c.is_whitespace() => {
                    self.advance();
                }
                Some(&'/') => {
                    self.advance();
                    if self.peek() == Some(&'/') {
                        while let Some(ch) = self.advance() {
                            if ch == '\n' {
                                break;
                            }
                        }
                    } else {
                        // In a real lexer we'd handle single slash. Here we panic for simplicity if unexpected.
                        panic!("Unexpected single /");
                    }
                }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        if let Some(&c) = self.peek() {
            match c {
                '{' => {
                    self.advance();
                    Ok(Token::OpenBrace)
                }
                '}' => {
                    self.advance();
                    Ok(Token::CloseBrace)
                }
                '(' => {
                    self.advance();
                    Ok(Token::OpenParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::CloseParen)
                }
                ':' => {
                    self.advance();
                    Ok(Token::Colon)
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::OpLessThanOrEqual)
                    } else {
                        Ok(Token::OpLessThan)
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::OpGreaterThanOrEqual)
                    } else {
                        Ok(Token::OpGreaterThan)
                    }
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::OpEquals)
                    } else {
                        Err("Expected ==, got =".to_string())
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::OpNotEquals)
                    } else {
                        Err("Expected !=, got !".to_string())
                    }
                }
                '"' => {
                    self.advance(); // consume quote
                    let mut string = String::new();
                    while let Some(ch) = self.advance() {
                        if ch == '"' {
                            break;
                        }
                        string.push(ch);
                    }
                    Ok(Token::StringLiteral(string))
                }
                c if c.is_ascii_digit() => {
                    let mut num = String::new();
                    while let Some(&ch) = self.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            num.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    Ok(Token::FloatLiteral(num.parse().unwrap_or(0.0)))
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
                        "agent" => Ok(Token::Agent),
                        "memory" => Ok(Token::Memory),
                        "tool" => Ok(Token::Tool),
                        "guardrail" => Ok(Token::Guardrail),
                        "intent" => Ok(Token::Intent),
                        "assert" => Ok(Token::Assert),
                        "achieve" => Ok(Token::Achieve),
                        "else" => Ok(Token::Else),
                        _ => Ok(Token::Identifier(ident)),
                    }
                }
                _ => Err(format!("Unexpected character: {}", c)),
            }
        } else {
            Ok(Token::EOF)
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok == Token::EOF {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }
}
