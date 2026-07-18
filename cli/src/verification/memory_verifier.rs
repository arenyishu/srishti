use srishti_runtime::memory::VectorMemory;
use serde_json::json;

pub async fn verify() -> bool {
    let memory_name = "test_persistence_agent";
    
    // Step 1: Initialize and store
    {
        let memory = VectorMemory::new(memory_name, false);
        let _ = memory.store("customer_id", json!("123")).await;
    }

    // Step 2: Re-initialize (simulating restart) and read
    {
        let memory = VectorMemory::new(memory_name, false);
        let value = memory.get("customer_id").await.unwrap_or(None);
        
        // Clean up
        let _ = std::fs::remove_file(format!("srishti_memory_{}.json", memory_name));
        
        if let Some(val) = value {
            return val.as_str() == Some("123");
        }
        
        false
    }
}
