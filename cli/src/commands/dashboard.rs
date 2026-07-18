use colored::*;
use axum::{
    routing::{get, post},
    Router,
    response::{Json, sse::{Event as SseEvent, Sse}},
    extract::{State, Path},
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use std::convert::Infallible;
use tower_http::cors::CorsLayer;
use srishti_runtime::process_table::{ProcessTable, ProcessIdentity};
use srishti_runtime::cluster::cluster_manager::ClusterManager;
use srishti_runtime::governance::approval_store::{ApprovalStore, PendingApproval};
use srishti_runtime::Event;
use std::collections::HashMap;

#[derive(Clone)]
struct AgentState {
    identity: ProcessIdentity,
    status: String,
    cpu_usage: String,
    memory_usage: String,
    token_usage: String,
    started_at: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct MemoryStat {
    collection: String,
    entries: usize,
    encrypted: bool,
    last_action: String,
    timestamp: u64,
}

#[derive(Clone)]
struct AppState {
    process_table: Arc<ProcessTable>,
    _cluster_manager: Arc<ClusterManager>,
    approval_store: Arc<ApprovalStore>,
    tx: broadcast::Sender<Event>,
    agents: Arc<RwLock<HashMap<String, AgentState>>>,
    memory_stats: Arc<RwLock<HashMap<String, MemoryStat>>>,
}

pub fn execute(demo: bool) {
    println!("{} Starting Srishti Control Plane...", "[DASHBOARD]".green().bold());
    println!("Web UI available at http://localhost:5173");
    println!("API Server running at http://localhost:3000");
    if demo {
        println!("{} Running in Simulated Demonstration Mode", "[DEMO]".yellow().bold());
    }

    let (tx, _) = broadcast::channel(100);

    let state = AppState {
        process_table: Arc::new(ProcessTable::new()),
        _cluster_manager: Arc::new(ClusterManager::new()),
        approval_store: Arc::new(ApprovalStore::new()),
        tx,
        agents: Arc::new(RwLock::new(HashMap::new())),
        memory_stats: Arc::new(RwLock::new(HashMap::new())),
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // Demo mode is now fully disabled as per Srishti OS integration goals
            let app = Router::new()
                .route("/", get(|| async { "Srishti OS Control Plane API" }))
                .route("/api/status", get(status_handler))
                .route("/api/agents", get(agents_handler))
                .route("/api/cluster", get(cluster_handler))
                .route("/api/approvals", get(approvals_handler))
                .route("/api/approvals/:id/approve", post(approve_handler))
                .route("/api/approvals/:id/reject", post(reject_handler))
                .route("/api/events/stream", get(sse_handler))
                .route("/api/memory", get(memory_handler))
                .route("/api/policies", get(policies_handler))
                .route("/api/workflows", get(workflows_handler))
                // Internal API
                .route("/api/internal/events", post(internal_event_handler))
                .route("/api/internal/process", post(internal_process_handler))
                .route("/api/internal/process_remove", post(internal_process_remove_handler))
                .route("/api/internal/approval", post(internal_approval_handler))
                .route("/api/internal/approval_update", post(internal_approval_update_handler))
                .route("/api/internal/memory", post(internal_memory_handler))
                .layer(CorsLayer::permissive())
                .with_state(state);

            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });
}

async fn status_handler(State(state): State<AppState>) -> Json<Value> {
    let agents = state.agents.read().await;
    let running = agents.values().filter(|a| a.status == "Running").count();
    let suspended = agents.values().filter(|a| a.status == "Suspended").count();
    let approvals = state.approval_store.get_pending().await.len();
    
    Json(json!({ 
        "status": "ok", 
        "version": "1.0 Moksha", 
        "uptime": "running",
        "running_agents": running,
        "suspended_agents": suspended,
        "cluster_nodes": 1,
        "active_policies": 0,
        "pending_approvals": approvals
    }))
}

async fn agents_handler(State(state): State<AppState>) -> Json<Value> {
    let agents = state.agents.read().await;
    let mut agent_list = Vec::new();
    for (_, agent) in agents.iter() {
        agent_list.push(json!({
            "pid": agent.identity.id,
            "name": agent.identity.name,
            "version": agent.identity.version,
            "permissions": agent.identity.permissions,
            "role": "Agent",
            "status": agent.status,
            "node": "nd-local",
            "cpu_usage": agent.cpu_usage,
            "memory_usage": agent.memory_usage,
            "token_usage": agent.token_usage,
            "started_at": agent.started_at,
        }));
    }
    Json(json!({ "agents": agent_list }))
}

async fn cluster_handler(State(_state): State<AppState>) -> Json<Value> {
    // Expose dynamic metrics in the future; return static structure to ensure UI compatibility for now
    Json(json!({ "nodes": 1, "quorum": true, "leader": "self" }))
}

async fn approvals_handler(State(state): State<AppState>) -> Json<Value> {
    let pending = state.approval_store.get_pending().await;
    Json(json!({ "pending": pending }))
}

async fn memory_handler(State(state): State<AppState>) -> Json<Value> {
    let stats = state.memory_stats.read().await;
    let entries: Vec<&MemoryStat> = stats.values().collect();
    Json(json!({ "stats": entries }))
}

async fn policies_handler() -> Json<Value> {
    Json(json!({ "policies": [] }))
}

async fn workflows_handler() -> Json<Value> {
    Json(json!({ "workflows": [] }))
}

async fn approve_handler(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    let _ = state.approval_store.update_status_local(&id, "approved").await;
    // Broadcast event
    let mut payload = HashMap::new();
    payload.insert("approval_id".to_string(), Value::String(id.clone()));
    let _ = state.tx.send(Event {
        name: "ApprovalGranted".to_string(),
        source_agent: "Dashboard".to_string(),
        target_agent: None,
        payload,
    });
    Json(json!({"status": "approved"}))
}

async fn reject_handler(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    let _ = state.approval_store.update_status_local(&id, "rejected").await;
    Json(json!({"status": "rejected"}))
}

async fn sse_handler(State(state): State<AppState>) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let mut rx = state.tx.subscribe();
    let stream = async_stream::stream! {
        while let Ok(event) = rx.recv().await {
            let json_str = serde_json::to_string(&event).unwrap();
            println!("[SSE] Broadcasting event: {}", json_str);
            yield Ok(SseEvent::default().event("message").data(json_str));
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

async fn internal_event_handler(State(state): State<AppState>, axum::extract::Json(event): axum::extract::Json<Event>) -> Json<Value> {
    // Update agent status based on event
    if event.name == "AgentSuspended" {
        let mut agents = state.agents.write().await;
        if let Some(agent) = agents.get_mut(&event.source_agent) {
            agent.status = "Suspended".to_string();
        }
    } else if event.name == "AgentStarted" {
        let mut agents = state.agents.write().await;
        if let Some(agent) = agents.get_mut(&event.source_agent) {
            agent.status = "Running".to_string();
        }
    }
    
    let _ = state.tx.send(event.clone());
    println!("[DASHBOARD] Received event: {}", event.name);
    Json(json!({"status": "ok"}))
}

async fn internal_process_handler(State(state): State<AppState>, axum::extract::Json(process): axum::extract::Json<ProcessIdentity>) -> Json<Value> {
    let _ = state.process_table.register_local(process.clone()).await;
    let mut agents = state.agents.write().await;
    agents.insert(process.id.clone(), AgentState {
        identity: process.clone(),
        status: "Init".to_string(),
        cpu_usage: "0.0%".to_string(),
        memory_usage: "15 MB".to_string(),
        token_usage: "0".to_string(),
        started_at: chrono::Utc::now().to_rfc3339(),
    });
    
    let current_count = agents.len();
    println!("[DASHBOARD] Received process: {}", process.name);
    println!("  Current agent count: {}", current_count);
    
    Json(json!({"status": "ok"}))
}

#[derive(serde::Deserialize)]
struct ProcessRemoveReq {
    id: String,
}

async fn internal_process_remove_handler(State(state): State<AppState>, axum::extract::Json(req): axum::extract::Json<ProcessRemoveReq>) -> Json<Value> {
    let mut agents = state.agents.write().await;
    agents.remove(&req.id);
    Json(json!({"status": "ok"}))
}

async fn internal_approval_handler(State(state): State<AppState>, axum::extract::Json(approval): axum::extract::Json<PendingApproval>) -> Json<Value> {
    state.approval_store.add_approval_local(approval.id.clone(), approval).await;
    
    let pending_count = state.approval_store.get_pending().await.len();
    println!("[DASHBOARD] Received approval request");
    println!("  Current approvals: {}", pending_count);
    
    // Notify frontend
    let mut payload = HashMap::new();
    payload.insert("data".to_string(), Value::String("New approval required".to_string()));
    let _ = state.tx.send(Event {
        name: "ApprovalRequested".to_string(),
        source_agent: "System".to_string(),
        target_agent: None,
        payload,
    });
    Json(json!({"status": "ok"}))
}

async fn internal_approval_update_handler(State(state): State<AppState>, axum::extract::Json(approval): axum::extract::Json<PendingApproval>) -> Json<Value> {
    let _ = state.approval_store.update_status_local(&approval.id, &approval.status).await;
    Json(json!({"status": "ok"}))
}

async fn internal_memory_handler(State(state): State<AppState>, axum::extract::Json(stat): axum::extract::Json<MemoryStat>) -> Json<Value> {
    let mut stats = state.memory_stats.write().await;
    stats.insert(stat.collection.clone(), stat.clone());
    
    let total_records: usize = stats.values().map(|s| s.entries).sum();
    println!("[DASHBOARD] Received memory update for collection: {}", stat.collection);
    println!("  Current memory records (total): {}", total_records);
    
    let mut payload = HashMap::new();
    payload.insert("collection".to_string(), Value::String(stat.collection.clone()));
    let _ = state.tx.send(Event {
        name: "MemoryStored".to_string(),
        source_agent: "System".to_string(),
        target_agent: None,
        payload,
    });
    Json(json!({"status": "ok"}))
}

