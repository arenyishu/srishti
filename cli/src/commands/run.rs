use colored::*;
use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use std::fs;

pub async fn execute(filepath: &str) {
    println!("{} {}", "Compiling".green().bold(), filepath);

    let source = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(err) => {
            eprintln!(
                "{} reading file {}: {}",
                "Error".red().bold(),
                filepath,
                err
            );
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} {}", "Lexer error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("{}", "AST parsed successfully.".green());

            // Note: Interpreter execution will happen here in Phase 3
            // For now, we'll still show the codegen output to verify it works
        let mut interpreter = srishti_compiler::interpreter::Interpreter::new();
        
        if let Err(e) = interpreter.execute(&program).await {
            eprintln!("{} {}", "Runtime Error:".red().bold(), e);
            std::process::exit(1);
        }
        }
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
