use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Agent,
    Intent,
    Tool,
    Guardrail,
    Achieve,
    Extract,
    From,
    Let,
    Persistent,
    Memory,
    Assert,
    Else,
    Trigger,
    Return,
    
    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),
    IntLiteral(i64),
    
    Colon,
    Semicolon,
    Comma,
    Arrow, // ->
    Equals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    
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

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(&c) = self.peek() {
            match c {
                '{' => { self.advance(); Token::OpenBrace }
                '}' => { self.advance(); Token::CloseBrace }
                '(' => { self.advance(); Token::OpenParen }
                ')' => { self.advance(); Token::CloseParen }
                '<' => { 
                    self.advance(); 
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::LessThanOrEqual
                    } else {
                        Token::LessThan 
                    }
                }
                '>' => { 
                    self.advance(); 
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Token::GreaterThanOrEqual
                    } else {
                        Token::GreaterThan 
                    }
                }
                ':' => { self.advance(); Token::Colon }
                ';' => { self.advance(); Token::Semicolon }
                ',' => { self.advance(); Token::Comma }
                '=' => { self.advance(); Token::Equals }
                '-' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        panic!("Unexpected token '-'");
                    }
                }
                '"' => {
                    self.advance(); // consume open quote
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
                        Token::FloatLiteral(num.parse().unwrap())
                    } else {
                        Token::IntLiteral(num.parse().unwrap())
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
                        "intent" => Token::Intent,
                        "tool" => Token::Tool,
                        "guardrail" => Token::Guardrail,
                        "achieve" => Token::Achieve,
                        "extract" => Token::Extract,
                        "from" => Token::From,
                        "let" => Token::Let,
                        "persistent" => Token::Persistent,
                        "memory" => Token::Memory,
                        "assert" => Token::Assert,
                        "else" => Token::Else,
                        "trigger" => Token::Trigger,
                        "return" => Token::Return,
                        _ => Token::Identifier(ident),
                    }
                }
                '/' => {
                    self.advance();
                    if self.peek() == Some(&'/') {
                        while let Some(ch) = self.advance() {
                            if ch == '\n' { break; }
                        }
                        self.next_token()
                    } else {
                        panic!("Unexpected character: /");
                    }
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        } else {
            Token::EOF
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::EOF {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}
