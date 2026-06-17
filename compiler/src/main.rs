use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use srishti_compiler::codegen::Codegen;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] != "run" {
        eprintln!("Usage: srishti run <file.srishti>");
        std::process::exit(1);
    }

    let filepath = &args[2];
    let source = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filepath, err);
            std::process::exit(1);
        }
    };

    println!("Compiling Srishti File: {}", filepath);

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("AST successfully parsed.");
            let codegen = Codegen::new();
            let rust_code = codegen.generate(&program);
            
            // Output to console for now, as it's easier to verify
            println!("\n// ---------------- Generated Rust Code ---------------- //\n");
            println!("{}", rust_code);
            println!("// ----------------------------------------------------- //\n");
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
