use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use srishti_compiler::typechecker::TypeChecker;

pub async fn verify() -> bool {
    let source = r#"
        agent A { id: "same" }
        agent B { id: "same" }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return false,
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(_) => return false,
    };

    let checker = TypeChecker::new();
    let result = checker.check(&ast);

    // Should return diagnostics because of duplicate ID
    !result.is_empty()
}
