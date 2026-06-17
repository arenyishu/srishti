use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "srishti")]
#[command(about = "Srishti Agent-Oriented Programming Language Toolchain", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a Srishti agent file via the tree-walking interpreter
    Run {
        /// The .srishti file to run
        file: PathBuf,
    },
    /// Compile a Srishti project into a standalone Rust binary
    Build {
        /// The .srishti file or project directory to build
        file: Option<PathBuf>,
    },
    /// Parse and type-check a file without executing
    Check {
        /// The .srishti file to check
        file: PathBuf,
    },
    /// Scaffold a new Srishti project
    Init {
        /// Project name
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => run_command(file)?,
        Commands::Build { file } => build_command(file)?,
        Commands::Check { file } => check_command(file)?,
        Commands::Init { name } => init_command(name)?,
    }

    Ok(())
}

fn run_command(file: &PathBuf) -> anyhow::Result<()> {
    println!("{} {}", "Running".green().bold(), file.display());
    // Stub for phase 1
    // Parse, AST, Interpret (Phase 3)
    let source = fs::read_to_string(file)?;
    
    // For Phase 1 we'll just demonstrate the parser works
    use srishti_compiler::lexer::Lexer;
    use srishti_compiler::parser::Parser;
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| anyhow::anyhow!("Lexer error: {}", e))?;
    
    let mut parser = Parser::new(tokens);
    let _program = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    println!("{} Agent interpreted successfully (stub)", "Success".green().bold());
    Ok(())
}

fn build_command(file: &Option<PathBuf>) -> anyhow::Result<()> {
    let file = file.clone().unwrap_or(PathBuf::from("src/main.srishti"));
    println!("{} {}", "Building".green().bold(), file.display());
    // Stub for phase 1
    use srishti_compiler::lexer::Lexer;
    use srishti_compiler::parser::Parser;
    use srishti_compiler::codegen::Codegen;
    
    let source = fs::read_to_string(&file)?;
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| anyhow::anyhow!("Lexer error: {}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    let codegen = Codegen::new();
    let rust_code = codegen.generate(&program);
    
    println!("{} Code generation complete (stub output)", "Success".green().bold());
    println!("{}", rust_code);
    Ok(())
}

fn check_command(file: &PathBuf) -> anyhow::Result<()> {
    println!("{} {}", "Checking".green().bold(), file.display());
    // Stub for phase 1
    let source = fs::read_to_string(file)?;
    use srishti_compiler::lexer::Lexer;
    use srishti_compiler::parser::Parser;
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| anyhow::anyhow!("Lexer error: {}", e))?;
    let mut parser = Parser::new(tokens);
    let _program = parser.parse().map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    println!("{} No syntax errors found", "Success".green().bold());
    Ok(())
}

fn init_command(name: &str) -> anyhow::Result<()> {
    println!("{} new Srishti project `{}`", "Initializing".green().bold(), name);
    
    fs::create_dir_all(name)?;
    fs::create_dir_all(format!("{}/src", name))?;
    
    let toml_content = format!(r#"[project]
name = "{}"
version = "0.1.0"
entry = "src/main.srishti"

[runtime]
provider = "openai"
model = "gpt-4o"
"#, name);

    fs::write(format!("{}/srishti.toml", name), toml_content)?;
    
    let main_srishti = r#"agent HelloAgent {
    intent say_hello {
        achieve "Say hello to the world!"
    }
}
"#;
    fs::write(format!("{}/src/main.srishti", name), main_srishti)?;
    
    let gitignore = "/build\n/srishti_modules\n";
    fs::write(format!("{}/.gitignore", name), gitignore)?;
    
    println!("{} Created project template", "Success".green().bold());
    Ok(())
}
