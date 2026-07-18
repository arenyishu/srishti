use srishti_runtime::network::rpc_server::start_rpc_server;
use srishti_runtime::network::rpc_client::send_remote_intent;
use std::time::Duration;
use tokio::time::sleep;

pub async fn verify() -> bool {
    // Start server in background
    let addr = "127.0.0.1:50052";
    tokio::spawn(async move {
        let _ = start_rpc_server(addr).await;
    });

    // Wait for server to bind
    sleep(Duration::from_millis(500)).await;

    // Send remote intent
    let result = send_remote_intent("http://127.0.0.1:50052", "RemoteAgent", "hello", "{}").await;
    
    result.is_ok()
}
