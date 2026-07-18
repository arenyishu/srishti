use tonic::{transport::Server, Request, Response, Status};
use super::agent_rpc::agent_service_server::{AgentService, AgentServiceServer};
use super::agent_rpc::{IntentRequest, IntentResponse};

pub struct RpcServerImpl {
    // In a real implementation, this would hold a reference to the AgentKernel
    // to dispatch the intent locally.
}

#[tonic::async_trait]
impl AgentService for RpcServerImpl {
    async fn send_intent(&self, request: Request<IntentRequest>) -> Result<Response<IntentResponse>, Status> {
        let req = request.into_inner();
        println!("Received remote intent: {} for agent {}", req.intent_name, req.agent_name);
        
        let response = IntentResponse {
            status: "success".to_string(),
            result_json: "{}".to_string(),
            error_message: "".to_string(),
        };

        Ok(Response::new(response))
    }
}

pub async fn start_rpc_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;
    let server = RpcServerImpl {};

    println!("gRPC Agent Server listening on {}", addr);

    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(async move {
        super::health::register_health_service(health_reporter).await;
    });

    Server::builder()
        .add_service(health_service)
        .add_service(AgentServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
