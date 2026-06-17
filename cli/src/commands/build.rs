use colored::*;
use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use std::fs;
use std::path::Path;

pub fn execute(filepath: Option<&str>) {
    let file = filepath.unwrap_or("src/main.srishti");
    println!("{} {}", "Compiling".green().bold(), file);

    let source = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{} reading file {}: {}", "Error".red().bold(), file, err);
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
            let codegen = srishti_compiler::codegen::Codegen::new();
            let rust_code = codegen.generate(&program);
            
            let build_dir = Path::new("build");
            if !build_dir.exists() {
                fs::create_dir(build_dir).expect("Failed to create build directory");
            }
            
            let output_file = build_dir.join("generated.rs");
            fs::write(&output_file, rust_code).expect("Failed to write generated code");
            
            println!("{} {}", "Code generated:".green(), output_file.display());
            println!("{}", "Build complete.".green().bold());
        }
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
