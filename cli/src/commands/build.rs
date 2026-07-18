use crate::project;
use colored::*;
use srishti_compiler::lexer::Lexer;
use srishti_compiler::parser::Parser;
use std::fs;
use std::path::Path;

pub fn execute(filepath: Option<&str>, target_arg: Option<&str>) {
    let loaded_project = project::load_project();
    let file = filepath
        .map(str::to_owned)
        .or_else(|| {
            loaded_project
                .as_ref()
                .and_then(|p| p.project.entry.clone())
        })
        .unwrap_or_else(|| "src/main.srishti".to_string());
    let output_dir = loaded_project
        .as_ref()
        .and_then(|p| p.build.as_ref())
        .and_then(|build| build.output.clone())
        .unwrap_or_else(|| "build".to_string());
    let build_target = target_arg.unwrap_or_else(|| {
        loaded_project
            .as_ref()
            .and_then(|p| p.build.as_ref())
            .and_then(|build| build.target.as_deref())
            .unwrap_or("binary")
    });

    if let Some(project) = &loaded_project {
        println!(
            "{} {} v{}",
            "Project:".cyan().bold(),
            project.project.name,
            project.project.version
        );

        if let Some(authors) = &project.project.authors {
            if !authors.is_empty() {
                println!("{} {}", "Authors:".cyan().bold(), authors.join(", "));
            }
        }

        if let Some(runtime) = &project.runtime {
            println!(
                "{} provider={}, model={}, memory={}",
                "Runtime:".cyan().bold(),
                runtime.provider.as_deref().unwrap_or("mock"),
                runtime.model.as_deref().unwrap_or("default"),
                runtime.memory_backend.as_deref().unwrap_or("in-memory")
            );
        }

        println!("{} {}", "Target:".cyan().bold(), build_target);
    }

    println!("{} {}", "Compiling".green().bold(), file);

    let source = match fs::read_to_string(&file) {
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

            let build_dir = Path::new(&output_dir);
            if !build_dir.exists() {
                fs::create_dir_all(build_dir).expect("Failed to create build directory");
            }

            let output_file = build_dir.join("generated.rs");
            fs::write(&output_file, rust_code).expect("Failed to write generated code");

            println!("{} {}", "Code generated:".green(), output_file.display());
            
            if build_target == "docker" {
                let cargo_toml = r#"[package]
name = "srishti-agent"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
"#;
                let cargo_path = build_dir.join("Cargo.toml");
                fs::write(&cargo_path, cargo_toml).expect("Failed to write Cargo.toml");

                let main_rs = r#"
fn main() {
    println!("Srishti Agent Booting...");
}
"#;
                let src_dir = build_dir.join("src");
                fs::create_dir_all(&src_dir).unwrap_or_default();
                fs::write(src_dir.join("main.rs"), main_rs).expect("Failed to write main.rs");

                let dockerfile = r#"FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/srishti-agent .
CMD ["./srishti-agent"]
"#;
                let docker_path = build_dir.join("Dockerfile");
                fs::write(&docker_path, dockerfile).expect("Failed to write Dockerfile");
                println!("{} {}", "Cargo.toml and Dockerfile generated:".green(), build_dir.display());
            }

            println!("{}", "Build complete.".green().bold());
        }
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
