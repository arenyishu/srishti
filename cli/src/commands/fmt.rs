use colored::*;
use std::fs;

pub fn execute(filepath: &str) {
    println!("Formatting {}...", filepath);

    let source = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{} reading file {}: {}", "Error".red().bold(), filepath, err);
            std::process::exit(1);
        }
    };

    // Very basic formatter stub: just fix indentation levels
    // A proper formatter would use the AST
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            formatted.push_str("\n");
            continue;
        }
        
        if trimmed.starts_with('}') {
            indent_level = indent_level.saturating_sub(1);
        }
        
        formatted.push_str(&"    ".repeat(indent_level));
        formatted.push_str(trimmed);
        formatted.push_str("\n");
        
        if trimmed.ends_with('{') {
            indent_level += 1;
        }
    }

    fs::write(filepath, formatted).expect("Failed to write formatted code");
    println!("{} {}", "Formatted".green().bold(), filepath);
}
