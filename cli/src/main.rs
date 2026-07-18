use clap::{Parser, Subcommand};
use colored::*;

pub mod commands;
pub mod project;
pub mod verification;

#[derive(Parser)]
#[command(name = "srishti")]
#[command(version = "0.3 Prarambh", about = "Srishti Agent Runtime Platform for Reliable AI Systems", long_about = None)]
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
        #[arg(short, long)]
        target: Option<String>,
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
        /// Package name to install
        package: Option<String>,
        /// Optional license key for premium marketplace agents
        #[arg(long)]
        license: Option<String>,
    },
    /// Publish an agent package to the registry
    Publish,
    /// Search the agent registry
    Search {
        /// Query string
        query: String,
    },
    /// Start the Srishti Control Plane web dashboard
    Dashboard {
        #[arg(long)]
        demo: bool,
    },
    /// Format a .srishti file
    Fmt {
        /// The file to format
        file: String,
    },
    /// Srishti cluster management
    Cluster {
        #[command(subcommand)]
        cmd: commands::cluster::ClusterCommands,
    },
    /// Manage agent approvals
    Approvals {
        #[command(subcommand)]
        cmd: commands::approvals::ApprovalCommands,
    },
    /// Show OS Agent Activity Monitor
    Top,
    /// Deploy to Kubernetes via Helm or Cloud
    Deploy {
        /// Target deployment (e.g. 'cloud')
        target: Option<String>,
    },
    /// Run OS Validation Suite
    Verify {
        #[arg(long)]
        full: bool,
        #[arg(long)]
        quick: bool,
        #[arg(long)]
        dashboard: bool,
        #[arg(long)]
        sync: bool,
    },
    /// Srishti OS Demonstration
    Demo {
        #[command(subcommand)]
        cmd: Option<commands::demo::DemoCommands>,
    },
    /// Srishti OS Production Readiness Gate
    Readiness,
    /// Generate executable proof of the entire OS system
    Proof,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            commands::run::execute(file).await;
        }
        Some(Commands::Build { file, target }) => {
            commands::build::execute(file.as_deref(), target.as_deref());
        }
        Some(Commands::Init { name }) => {
            commands::init::execute(name.as_deref());
        }
        Some(Commands::Check { file }) => {
            commands::check::execute(file);
        }
        Some(Commands::Install { package, license }) => commands::install::execute(package.as_deref(), license.as_deref()),
        Some(Commands::Publish) => commands::registry::publish(),
        Some(Commands::Search { query }) => commands::registry::search(query),
        Some(Commands::Dashboard { demo }) => commands::dashboard::execute(*demo),
        Some(Commands::Fmt { file }) => commands::fmt::execute(file),
        Some(Commands::Cluster { cmd }) => {
            commands::cluster::execute(cmd);
        }
        Some(Commands::Approvals { cmd }) => {
            commands::approvals::execute(cmd);
        }
        Some(Commands::Top) => {
            if let Err(e) = commands::top::execute() {
                println!("{}", format!("Failed: {}", e).red());
            }
        }
        Some(Commands::Deploy { target }) => {
            commands::deploy::execute(target.as_deref());
        }
        Some(Commands::Verify { full, quick, dashboard, sync }) => {
            commands::verify::execute(*full, *quick, *dashboard, *sync).await;
        }
        Some(Commands::Demo { cmd }) => {
            commands::demo::execute(cmd).await;
        }
        Some(Commands::Readiness) => {
            commands::readiness::execute();
        }
        Some(Commands::Proof) => {
            commands::proof::execute().await;
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
    println!(
        "{}",
        "Srishti v0.3 Prarambh — Agent Runtime Platform for Reliable AI Systems\n".green()
    );
}
