use super::agent_rpc::agent_service_client::AgentServiceClient;
use super::agent_rpc::IntentRequest;
use std::error::Error;

pub async fn send_remote_intent(endpoint: &str, agent_name: &str, intent_name: &str, payload: &str) -> Result<String, Box<dyn Error>> {
    let endpoint = tonic::transport::Endpoint::from_shared(endpoint.to_string())?
        .timeout(std::time::Duration::from_secs(5));
        
    let mut client = AgentServiceClient::connect(endpoint).await?;

    let request = tonic::Request::new(IntentRequest {
        agent_name: agent_name.to_string(),
        intent_name: intent_name.to_string(),
        payload_json: payload.to_string(),
    });

    let response = client.send_intent(request).await?;
    let resp_inner = response.into_inner();
    
    if resp_inner.status == "success" {
        Ok(resp_inner.result_json)
    } else {
        Err(resp_inner.error_message.into())
    }
}
