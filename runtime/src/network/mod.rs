pub mod agent_registry;
pub mod rpc_server;
pub mod rpc_client;
pub mod tls;
pub mod health;

pub mod agent_rpc {
    tonic::include_proto!("agent_rpc");
}
