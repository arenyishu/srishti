use colored::*;
use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use std::fs;

pub fn execute(filepath: &str) {
    println!("Checking {}...", filepath);

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
        Ok(_) => {
            println!("{} {} looks good.", "Success:".green().bold(), filepath);
        }
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
