
pub async fn register_health_service(mut reporter: tonic_health::server::HealthReporter) {
    reporter.set_serving::<crate::network::agent_rpc::agent_service_server::AgentServiceServer<crate::network::rpc_server::RpcServerImpl>>().await;
}
