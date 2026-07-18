use srishti_runtime::kernel::resource_manager::ResourceManager;

pub async fn verify() -> bool {
    let manager = ResourceManager::new();

    manager.register_process("agent_1", None, None, Some(100)).await;

    // Consume 50 tokens - Should succeed
    if manager.consume_tokens("agent_1", 50).await.is_err() {
        return false;
    }

    // Consume 100 more tokens - Should fail (50 + 100 > 100)
    let result = manager.consume_tokens("agent_1", 100).await;
    
    // We expect an error (QuotaExceeded)
    result.is_err()
}
