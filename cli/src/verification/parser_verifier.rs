use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;

pub async fn verify() -> bool {
    let source = r#"
        agent {
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return false,
    };

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Verification succeeds if the parser RETURNS an error (rejects invalid syntax)
    result.is_err()
}
