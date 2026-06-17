use colored::*;
use std::fs;
use std::path::Path;

pub fn execute(package: Option<&str>) {
    if let Some(pkg) = package {
        println!("{} {}...", "Installing".green().bold(), pkg);
        println!("(Package registry coming soon)");
    } else {
        println!(
            "{} dependencies from srishti.toml...",
            "Installing".green().bold()
        );
        println!("(Package registry coming soon)");
    }

    let modules_dir = Path::new("srishti_modules");
    if !modules_dir.exists() {
        fs::create_dir(modules_dir).expect("Failed to create srishti_modules directory");
    }
}
