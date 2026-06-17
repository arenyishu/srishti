use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod project;

#[derive(Parser)]
#[command(name = "srishti")]
#[command(about = "Srishti Agent-Oriented Programming Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and execute a .srishti file
    Run {
        /// The file to run
        file: String,
    },
    /// Compile to Rust code output in build/ directory
    Build {
        /// The file to build (optional if project has entry point)
        file: Option<String>,
    },
    /// Scaffold a new Srishti project
    Init {
        /// Project name (optional, defaults to current directory)
        name: Option<String>,
    },
    /// Validate a .srishti file without running
    Check {
        /// The file to check
        file: String,
    },
    /// Install dependencies
    Install {
        /// The package to install (optional, defaults to all in srishti.toml)
        package: Option<String>,
    },
    /// Format a .srishti file
    Fmt {
        /// The file to format
        file: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            commands::run::execute(file).await;
        }
        Some(Commands::Build { file }) => {
            commands::build::execute(file.as_deref());
        }
        Some(Commands::Init { name }) => {
            commands::init::execute(name.as_deref());
        }
        Some(Commands::Check { file }) => {
            commands::check::execute(file);
        }
        Some(Commands::Install { package }) => {
            commands::install::execute(package.as_deref());
        }
        Some(Commands::Fmt { file }) => {
            commands::fmt::execute(file);
        }
        None => {
            print_banner();
            println!("{}", "Usage: srishti <COMMAND>".yellow());
            println!("Run 'srishti --help' for more information.");
        }
    }
}

fn print_banner() {
    let banner = r#"
 ____       _     _     _   _ 
/ ___| _ __(_)___| |__ | |_(_)
\___ \| '__| / __| '_ \| __| |
 ___) | |  | \__ \ | | | |_| |
|____/|_|  |_|___/_| |_|\__|_|
"#;
    println!("{}", banner.cyan().bold());
    println!("{}", "Srishti v0.1.0 — Agent-Oriented Programming Language\n".green());
}
