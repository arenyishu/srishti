use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: String,
}

#[derive(Clone)]
pub struct StateReplicator {
    log: Arc<RwLock<Vec<LogEntry>>>,
    commit_index: Arc<RwLock<u64>>,
}

impl StateReplicator {
    pub fn new() -> Self {
        Self {
            log: Arc::new(RwLock::new(Vec::new())),
            commit_index: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn append_entry(&self, term: u64, command: String) -> u64 {
        let mut log = self.log.write().await;
        let index = log.len() as u64 + 1;
        log.push(LogEntry { term, index, command });
        index
    }

    pub async fn commit(&self, index: u64) {
        let mut commit_index = self.commit_index.write().await;
        if index > *commit_index {
            *commit_index = index;
            println!("StateReplicator: Committed index {}", index);
        }
    }
}

impl Default for StateReplicator {
    fn default() -> Self {
        Self::new()
    }
}
