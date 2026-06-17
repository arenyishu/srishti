use colored::*;
use std::fs;
use std::path::Path;

pub fn execute(name: Option<&str>) {
    let project_name = name.unwrap_or("srishti-project");
    
    let toml_content = format!(r#"[project]
name = "{}"
version = "0.1.0"
entry = "src/main.srishti"
authors = []

[agents]
sources = ["src/**/*.srishti"]

[dependencies]

[runtime]
provider = "openai"
model = "gpt-4o"
memory_backend = "in-memory"

[build]
output = "build/"
target = "binary"
"#, project_name);

    let hello_agent = r#"agent HelloAgent {
    intent say_hello {
        achieve "Say hello to the world"
    }
}
"#;

    let gitignore = r#"build/
target/
srishti_modules/
"#;

    // Create directories
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        fs::create_dir(src_dir).expect("Failed to create src directory");
        println!("{} src/", "Created".green().bold());
    }

    // Create files
    fs::write("srishti.toml", toml_content).expect("Failed to write srishti.toml");
    println!("{} srishti.toml", "Created".green().bold());

    fs::write("src/main.srishti", hello_agent).expect("Failed to write main.srishti");
    println!("{} src/main.srishti", "Created".green().bold());

    fs::write(".gitignore", gitignore).expect("Failed to write .gitignore");
    println!("{} .gitignore", "Created".green().bold());

    println!("\n{} {} initialized.", "Successfully".green().bold(), project_name);
}
