use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::future::Future;

#[derive(Clone)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

pub struct Supervisor {
    restarts: Arc<RwLock<HashMap<String, u32>>>,
    max_retries: u32,
}

impl Supervisor {
    pub fn new(max_retries: u32) -> Self {
        Self {
            restarts: Arc::new(RwLock::new(HashMap::new())),
            max_retries,
        }
    }

    pub async fn supervise<F, Fut>(&self, process_id: &str, policy: RestartPolicy, mut task: F) -> Result<(), String>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), String>> + Send + 'static,
    {
        let mut attempt = 0;
        loop {
            // We use AssertUnwindSafe to catch panics from the spawned agent task
            let res = tokio::task::spawn(std::panic::AssertUnwindSafe(task())).await;
            
            let failed = match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(e)) => {
                    println!("Supervisor: Agent {} failed with error: {}", process_id, e);
                    true
                }
                Err(e) => {
                    println!("Supervisor: Agent {} crashed (panic): {:?}", process_id, e);
                    true
                }
            };

            if failed {
                match policy {
                    RestartPolicy::Never => return Err(format!("Agent {} failed and RestartPolicy is Never", process_id)),
                    RestartPolicy::OnFailure | RestartPolicy::Always => {
                        if attempt >= self.max_retries {
                            return Err(format!("Agent {} exceeded max retries ({})", process_id, self.max_retries));
                        }
                        attempt += 1;
                        
                        let mut restarts = self.restarts.write().await;
                        *restarts.entry(process_id.to_string()).or_insert(0) += 1;
                        
                        println!("Supervisor: Restarting Agent {} (attempt {})", process_id, attempt);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}
