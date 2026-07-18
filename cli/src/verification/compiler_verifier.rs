use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use srishti_compiler::typechecker::TypeChecker;

pub async fn verify() -> bool {
    let source = r#"
        agent TestAgent {
            intent hello {
                achieve "hello"
            }
        }
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
    let diagnostics = checker.check(&ast);
    if !diagnostics.is_empty() {
        return false;
    }

    true
}
