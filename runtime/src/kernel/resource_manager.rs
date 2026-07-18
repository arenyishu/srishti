use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ProcessQuota {
    pub memory_mb: Option<i64>,
    pub cpu_percent: Option<i64>,
    pub tokens_per_hour: Option<i64>,
    pub tokens_used: i64,
}

#[derive(Clone)]
pub struct ResourceManager {
    quotas: Arc<RwLock<HashMap<String, ProcessQuota>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            quotas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_process(&self, pid: &str, memory_mb: Option<i64>, cpu_percent: Option<i64>, tokens_per_hour: Option<i64>) {
        let mut quotas = self.quotas.write().await;
        quotas.insert(pid.to_string(), ProcessQuota {
            memory_mb,
            cpu_percent,
            tokens_per_hour,
            tokens_used: 0,
        });
    }

    pub async fn consume_tokens(&self, pid: &str, amount: i64) -> Result<(), String> {
        let mut quotas = self.quotas.write().await;
        if let Some(quota) = quotas.get_mut(pid) {
            if let Some(limit) = quota.tokens_per_hour {
                if quota.tokens_used + amount > limit {
                    return Err(format!("QuotaExceeded: Process {} exceeded token limit ({} / {})", pid, quota.tokens_used + amount, limit));
                }
            }
            quota.tokens_used += amount;
            Ok(())
        } else {
            // Process not registered, ignore quota
            Ok(())
        }
    }

    pub async fn get_usage(&self, pid: &str) -> Option<i64> {
        let quotas = self.quotas.read().await;
        quotas.get(pid).map(|q| q.tokens_used)
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
