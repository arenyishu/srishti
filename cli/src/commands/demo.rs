use clap::Subcommand;
use colored::*;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Subcommand)]
pub enum DemoCommands {
    /// Run the default end-to-end simulated workflow
    Default,
    /// Run the approval demonstration workflow
    Approval,
    /// Run the distributed cluster demonstration
    Cluster,
}

pub async fn execute(cmd: &Option<DemoCommands>) {
    match cmd {
        Some(DemoCommands::Approval) => run_approval_demo().await,
        Some(DemoCommands::Cluster) => run_cluster_demo().await,
        _ => run_default_demo().await,
    }
}

async fn run_default_demo() {
    println!("{}", "==================================================".cyan());
    println!("{}", "       Srishti OS End-to-End Demonstration        ".cyan().bold());
    println!("{}", "==================================================\n".cyan());

    let steps = [
        ("[System]", "Starting Control Plane...", 500),
        ("[System]", "Starting Runtime...", 300),
        ("[System]", "Loading Policies...", 200),
        ("[System]", "Loading Memory...", 400),
        ("[System]", "Loading Cluster...", 300),
        ("[System]", "Loading Workflow Engine...", 200),
        ("\n[Agent:RouterAgent]", "State transition: Init -> Ready", 600),
        ("[System]", "Booted agent: RouterAgent", 100),
        ("[EventBus]", "Publishing event 'start_chat' from System", 400),
        ("[Agent:RouterAgent]", "State transition: Ready -> Running", 300),
        ("[Agent]", "RouterAgent handling event `start_chat`", 500),
        ("[IO]", "Received new support ticket.", 400),
        ("[SemanticEngine]", "Achieving: Classify ticket", 800),
        ("[SemanticEngine]", "Output: Decision: Category is 'Refund'.", 400),
        ("[EventBus]", "Publishing event 'route_to_refund' from RouterAgent", 300),
        ("\n[Agent:RefundAgent]", "State transition: Init -> Ready", 400),
        ("[Agent:RefundAgent]", "State transition: Ready -> Running", 300),
        ("[Agent]", "RefundAgent handling event `route_to_refund`", 500),
        ("[Guardrail]", "Checking guardrail `refund_limit`", 300),
        ("[Guardrail]", "Assertion passed.", 200),
        ("[Memory]", "Storing to `refund_history`", 400),
        ("[SemanticEngine]", "Achieving: Process refund requests", 900),
        ("[IO]", "Response: \"I have processed your $50 refund.\"", 400),
        ("[EventBus]", "Publishing event 'workflow_completed' from RefundAgent", 300),
        ("\n[System]", "Execution Complete. Srishti OS proves reliability.", 500),
    ];

    for (prefix, message, delay) in steps.iter() {
        sleep(Duration::from_millis(*delay)).await;
        let colored_prefix = match *prefix {
            "[System]" => prefix.blue().bold(),
            "[Agent]" | "[Agent:RouterAgent]" | "[Agent:RefundAgent]" => prefix.green().bold(),
            "[EventBus]" => prefix.magenta().bold(),
            "[SemanticEngine]" => prefix.cyan().bold(),
            "[IO]" => prefix.yellow().bold(),
            "[Guardrail]" => prefix.red().bold(),
            "[Memory]" => prefix.truecolor(255, 165, 0).bold(),
            _ => prefix.white(),
        };
        if prefix.starts_with('\n') {
            println!("\n{} {}", colored_prefix.to_string().replace('\n', ""), message);
        } else {
            println!("  {} {}", colored_prefix, message);
        }
    }
}

async fn run_approval_demo() {
    println!("{}", "\n[Demo: Human-in-the-Loop Approval]".cyan().bold());
    println!("  {} Booting RefundAgent...", "[System]".blue());
    sleep(Duration::from_millis(800)).await;
    println!("  {} Evaluating refund request for $500...", "[Agent:RefundAgent]".green());
    sleep(Duration::from_millis(800)).await;
    println!("  {} Triggered `FinancialTransactionLimit`", "[Policy]".red().bold());
    sleep(Duration::from_millis(500)).await;
    println!("  {} Agent execution suspended. Requesting human approval.", "[System]".yellow().bold());
    println!("  {} Waiting for Dashboard interaction...", "[IO]".yellow());
    
    // Simulate wait
    for _ in 0..3 {
        sleep(Duration::from_millis(1000)).await;
        println!("  ...");
    }
    
    println!("  {} Dashboard sent 'Approve' signal.", "[API]".magenta().bold());
    sleep(Duration::from_millis(600)).await;
    println!("  {} Agent execution resumed.", "[System]".green().bold());
    sleep(Duration::from_millis(400)).await;
    println!("  {} Processed $500 refund.", "[Agent:RefundAgent]".green());
}

async fn run_cluster_demo() {
    println!("{}", "\n[Demo: Distributed Cluster & Failover]".cyan().bold());
    println!("  {} Starting Leader Node [nd-alpha]...", "[Cluster]".blue().bold());
    sleep(Duration::from_millis(600)).await;
    println!("  {} Starting Worker Node 1 [nd-beta]...", "[Cluster]".blue());
    sleep(Duration::from_millis(400)).await;
    println!("  {} Starting Worker Node 2 [nd-gamma]...", "[Cluster]".blue());
    sleep(Duration::from_millis(800)).await;
    
    println!("  {} Quorum established. Distributing RPC traffic.", "[Network]".magenta());
    sleep(Duration::from_millis(1000)).await;
    
    println!("  {} SIGTERM received on nd-alpha.", "[System]".red().bold());
    println!("  {} Leader Node killed.", "[Cluster]".red());
    sleep(Duration::from_millis(600)).await;
    
    println!("  {} Heartbeat timeout detected.", "[Network]".yellow());
    sleep(Duration::from_millis(800)).await;
    println!("  {} Initiating Raft leader election...", "[Cluster]".cyan());
    sleep(Duration::from_millis(1200)).await;
    
    println!("  {} Node [nd-beta] elected as new Leader.", "[Cluster]".green().bold());
    println!("  {} Cluster stabilized. Traffic resumed.", "[Network]".green());
}
