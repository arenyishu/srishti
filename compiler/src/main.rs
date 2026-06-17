use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use srishti_compiler::codegen::Codegen;

fn main() {
    println!("Srishti Compiler - Agent-Oriented Programming");

    let source = r#"
agent SupportAgent {
    persistent memory user_history: Vector<Ticket>;

    tool issue_refund(amount: Float) {
        // deterministic block
    }

    guardrail limit(amount: Float) {
        assert amount <= 100 else "trigger human_fallback";
    }

    intent handle_ticket() {
        let decision = achieve "Find the best flight under $500";
    }
}
"#;

    println!("Compiling Srishti Source:\n{}", source);

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    // println!("Tokens: {:#?}", tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("AST successfully parsed.");
            let codegen = Codegen::new();
            let rust_code = codegen.generate(&program);
            
            println!("\nGenerated Rust Code:\n{}", rust_code);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
