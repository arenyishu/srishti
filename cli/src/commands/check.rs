use colored::*;
use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use srishti_compiler::typechecker::TypeChecker;
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
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let typechecker = TypeChecker::new();
    let diagnostics = typechecker.check(&ast);
    
    let has_errors = diagnostics.iter().any(|d| matches!(d.severity, srishti_compiler::typechecker::Severity::Error));
    
    if !diagnostics.is_empty() {
        for diag in &diagnostics {
            let color = match diag.severity {
                srishti_compiler::typechecker::Severity::Error => "Error".red().bold(),
                srishti_compiler::typechecker::Severity::Warning => "Warning".yellow().bold(),
                srishti_compiler::typechecker::Severity::Info => "Info".blue().bold(),
            };
            eprintln!("{} [{}]: {} at line {}", color, diag.code, diag.message, diag.span.line);
            if let Some(sug) = &diag.suggestion {
                eprintln!("  {} {}", "Suggestion:".green(), sug);
            }
        }
    }

    if has_errors {
        eprintln!("{} {} failed typechecking.", "Failure:".red().bold(), filepath);
        std::process::exit(1);
    }

    println!("{} {} looks good.", "Success:".green().bold(), filepath);
}
