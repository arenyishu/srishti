use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose::STANDARD as b64};
use std::fs;

/// A simple in-memory key-value store for agent memory
/// In production, this would be backed by a vector database
#[derive(Clone)]
pub struct VectorMemory {
    collection_name: String,
    store: Arc<RwLock<HashMap<String, Value>>>,
    encrypted: bool,
    key: Option<Key<Aes256Gcm>>,
}

impl VectorMemory {
    pub fn new(collection_name: &str, encrypted: bool) -> Self {
        let key = if encrypted {
            // Get root key from env, or use a fallback for local development so data survives restart
            let mut key_bytes = [0u8; 32];
            let env_key = std::env::var("SRISHTI_MASTER_KEY").unwrap_or_else(|_| "default_local_dev_key_32_bytes!!".to_string());
            let bytes = env_key.as_bytes();
            let len = bytes.len().min(32);
            key_bytes[..len].copy_from_slice(&bytes[..len]);
            Some(Key::<Aes256Gcm>::from_slice(&key_bytes).clone())
        } else {
            None
        };

        let mut store = HashMap::new();
        
        // Load from disk if it exists
        let filename = format!("srishti_memory_{}.json", collection_name);
        if let Ok(data) = fs::read_to_string(&filename) {
            if let Ok(parsed) = serde_json::from_str(&data) {
                store = parsed;
            }
        }

        Self {
            collection_name: collection_name.to_string(),
            store: Arc::new(RwLock::new(store)),
            encrypted,
            key,
        }
    }

    fn persist(&self, store: &HashMap<String, Value>) {
        let filename = format!("srishti_memory_{}.json", self.collection_name);
        if let Ok(json) = serde_json::to_string(store) {
            let _ = fs::write(&filename, json);
        }
    }

    /// Store a value with a key
    pub async fn store(&self, key: &str, data: Value) -> Result<(), anyhow::Error> {
        let mut store = self
            .store
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
            
        let mut data_to_store = data;
        
        if self.encrypted {
            if let Some(ref k) = self.key {
                let cipher = Aes256Gcm::new(k);
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits
                let pt = serde_json::to_vec(&data_to_store)?;
                let ct = cipher.encrypt(&nonce, pt.as_ref()).map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;
                
                let nonce_b64 = b64.encode(nonce);
                let ct_b64 = b64.encode(ct);
                data_to_store = serde_json::json!({
                    "encrypted": true,
                    "nonce": nonce_b64,
                    "ciphertext": ct_b64
                });
            }
        }
            
        println!("  [Memory:{}] Stored key: {}", self.collection_name, key);
        store.insert(key.to_string(), data_to_store);
        
        self.persist(&store);
        
        // Notify dashboard of memory update
        let entry_count = store.len();
        let collection_name = self.collection_name.clone();
        let encrypted = self.encrypted;
        let action_msg = format!("Stored key: {}", key);
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "collection": collection_name,
            "entries": entry_count,
            "encrypted": encrypted,
            "last_action": action_msg,
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        });
        
        println!("[TELEMETRY] Sending POST /api/internal/memory");
        println!("  URL: http://127.0.0.1:3000/api/internal/memory");
        println!("  Payload: {:?}", payload);
        match client.post("http://127.0.0.1:3000/api/internal/memory")
            .json(&payload)
            .send()
            .await {
            Ok(res) => println!("  HTTP status code: {}\n  Success: true", res.status()),
            Err(e) => println!("  Failure: {}", e),
        }
        
        Ok(())
    }

    /// Retrieve a value by key
    pub async fn get(&self, key: &str) -> Result<Option<Value>, anyhow::Error> {
        let store = self
            .store
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
            
        if let Some(val) = store.get(key) {
            if self.encrypted {
                if let Some(ref k) = self.key {
                    if val.get("encrypted").and_then(|v| v.as_bool()).unwrap_or(false) {
                        let nonce_b64 = val.get("nonce").and_then(|v| v.as_str()).unwrap_or("");
                        let ct_b64 = val.get("ciphertext").and_then(|v| v.as_str()).unwrap_or("");
                        
                        let nonce_bytes = b64.decode(nonce_b64)?;
                        let ct_bytes = b64.decode(ct_b64)?;
                        
                        let cipher = Aes256Gcm::new(k);
                        let pt = cipher.decrypt(Nonce::from_slice(&nonce_bytes), ct_bytes.as_ref())
                            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;
                            
                        let decoded_val: Value = serde_json::from_slice(&pt)?;
                        return Ok(Some(decoded_val));
                    }
                }
            }
            return Ok(Some(val.clone()));
        }
        
        Ok(None)
    }

    /// Search for entries (simple substring match on keys for now)
    pub async fn search(
        &self,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<(String, Value)>, anyhow::Error> {
        let store = self
            .store
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        println!(
            "  [Memory:{}] Searching: {} (top {})",
            self.collection_name, query, top_k
        );
        let results: Vec<(String, Value)> = store
            .iter()
            .filter(|(k, _)| k.contains(query))
            .take(top_k)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Ok(results)
    }

    /// List all entries
    pub async fn list_all(&self) -> Result<Vec<(String, Value)>, anyhow::Error> {
        let store = self
            .store
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(store.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    /// Clear all entries
    pub async fn clear(&self) -> Result<(), anyhow::Error> {
        let mut store = self
            .store
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        store.clear();
        self.persist(&store);
        Ok(())
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }
}
