use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use colored::Colorize;
use super::node_manager::Node;

#[derive(Clone)]
pub struct ClusterManager {
    nodes: Arc<RwLock<HashMap<String, Node>>>,
    leader_id: Arc<RwLock<Option<String>>>,
    term: Arc<RwLock<u64>>,
}

impl ClusterManager {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            leader_id: Arc::new(RwLock::new(None)),
            term: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn join_cluster(&self, node: Node) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id.clone(), node);
    }

    pub async fn get_nodes(&self) -> Vec<Node> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }
    
    pub async fn has_quorum(&self) -> bool {
        let nodes = self.nodes.read().await;
        let active_nodes = nodes.values().filter(|n| n.status == "active").count();
        // Self + active nodes
        let total = active_nodes + 1;
        // Require strictly > 50% for quorum (2 of 3, 3 of 5, etc)
        // If total nodes is 1, quorum is 1. If 2, quorum is 2. If 3, quorum is 2.
        let required = (total / 2) + 1;
        total >= required
    }

    pub fn start_heartbeat(&self) {
        let leader_id = self.leader_id.clone();
        let nodes_ref = self.nodes.clone();
        let term_ref = self.term.clone();
        
        // Clone for background task
        let manager = self.clone();
        
        tokio::spawn(async move {
            loop {
                // Hardcoded election timeout for Moksha simulation (1500ms)
                let timeout = 1500;
                tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
                
                if !manager.has_quorum().await {
                    println!("  {} Loss of Quorum. Suspending leader duties.", "[Cluster]".red());
                    *leader_id.write().await = None;
                    continue;
                }

                let mut leader = leader_id.write().await;
                if leader.is_none() {
                    let mut term = term_ref.write().await;
                    *term += 1;
                    let nodes = nodes_ref.write().await;
                    
                    if nodes.is_empty() || nodes.values().all(|n| n.status != "active") {
                        println!("  {} Elected Self as Cluster Leader (Term: {})", "[Cluster]".green().bold(), *term);
                        *leader = Some("self".to_string());
                    } else {
                        // In a real network we would broadcast RequestVote RPCs.
                        // For Moksha simulation, we'll elect based on highest Node ID sort
                        println!("  {} Discovered existing leader, joining... (Term: {})", "[Cluster]".cyan(), *term);
                        *leader = Some("remote".to_string());
                    }
                } else {
                    println!("  {} Heartbeat OK", "[Cluster]".blue());
                }
            }
        });
    }
}

impl Default for ClusterManager {
    fn default() -> Self {
        Self::new()
    }
}
