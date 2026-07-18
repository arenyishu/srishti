use srishti_runtime::audit_logger::AuditLogger;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

pub async fn verify() -> bool {
    let log_file = "srishti_verification_audit.jsonl";
    
    // Cleanup previous run
    let _ = fs::remove_file(log_file);

    let logger = AuditLogger::new(log_file.to_string());
    
    // Simulate events
    logger.log("AgentStarted", "agent_123", "Started normally").await.unwrap();
    logger.log("ToolCalled", "agent_123", "Called calculate_tax").await.unwrap();
    logger.log("AgentFinished", "agent_123", "Finished task").await.unwrap();

    // Allow time for async file writing
    sleep(Duration::from_millis(200)).await;

    let content = match fs::read_to_string(log_file) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let _ = fs::remove_file(log_file); // Cleanup

    content.contains("AgentStarted") && content.contains("ToolCalled") && content.contains("AgentFinished")
}
