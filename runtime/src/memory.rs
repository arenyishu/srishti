use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A simple in-memory key-value store for agent memory
/// In production, this would be backed by a vector database
#[derive(Clone)]
pub struct VectorMemory {
    collection_name: String,
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl VectorMemory {
    pub fn new(collection_name: &str) -> Self {
        Self {
            collection_name: collection_name.to_string(),
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Store a value with a key
    pub async fn store(&self, key: &str, data: Value) -> Result<(), anyhow::Error> {
        let mut store = self.store.write().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        println!("  [Memory:{}] Stored key: {}", self.collection_name, key);
        store.insert(key.to_string(), data);
        Ok(())
    }
    
    /// Retrieve a value by key
    pub async fn get(&self, key: &str) -> Result<Option<Value>, anyhow::Error> {
        let store = self.store.read().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(store.get(key).cloned())
    }
    
    /// Search for entries (simple substring match on keys for now)
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<(String, Value)>, anyhow::Error> {
        let store = self.store.read().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        println!("  [Memory:{}] Searching: {} (top {})", self.collection_name, query, top_k);
        let results: Vec<(String, Value)> = store.iter()
            .filter(|(k, _)| k.contains(query))
            .take(top_k)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Ok(results)
    }
    
    /// List all entries
    pub async fn list_all(&self) -> Result<Vec<(String, Value)>, anyhow::Error> {
        let store = self.store.read().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(store.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
    
    /// Clear all entries
    pub async fn clear(&self) -> Result<(), anyhow::Error> {
        let mut store = self.store.write().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        store.clear();
        Ok(())
    }
    
    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }
}
