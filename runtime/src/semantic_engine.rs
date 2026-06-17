use std::collections::HashMap;
use serde_json::Value;
use async_trait::async_trait;

/// Trait for LLM provider implementations
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Send a completion request to the LLM
    async fn complete(&self, prompt: &str, context: &HashMap<String, Value>) -> Result<String, anyhow::Error>;
    
    /// Get structured output from the LLM
    async fn structured_output(&self, prompt: &str, schema: &str) -> Result<Value, anyhow::Error>;
    
    /// Get the provider name
    fn provider_name(&self) -> &str;
}

/// OpenAI-compatible provider
pub struct OpenAIProvider {
    api_key: String,
    model: String,
    endpoint: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
        }
    }
    
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn complete(&self, prompt: &str, context: &HashMap<String, Value>) -> Result<String, anyhow::Error> {
        // Build the request with context
        let mut messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are an AI agent executing a task. Respond concisely."
            }),
        ];
        
        if !context.is_empty() {
            messages.push(serde_json::json!({
                "role": "system", 
                "content": format!("Context: {}", serde_json::to_string(context).unwrap_or_default())
            }));
        }
        
        messages.push(serde_json::json!({
            "role": "user",
            "content": prompt
        }));
        
        let client = reqwest::Client::new();
        let response = client.post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": messages,
                "temperature": 0.7
            }))
            .send()
            .await?;
        
        let body: Value = response.json().await?;
        let content = body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string();
        
        Ok(content)
    }
    
    async fn structured_output(&self, prompt: &str, schema: &str) -> Result<Value, anyhow::Error> {
        let full_prompt = format!(
            "{}\n\nRespond with valid JSON matching this schema:\n{}", 
            prompt, schema
        );
        let result = self.complete(&full_prompt, &HashMap::new()).await?;
        let value: Value = serde_json::from_str(&result)
            .unwrap_or_else(|_| Value::String(result));
        Ok(value)
    }
    
    fn provider_name(&self) -> &str {
        "openai"
    }
}

/// Mock provider for testing and offline development
pub struct MockProvider;

impl MockProvider {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LLMProvider for MockProvider {
    async fn complete(&self, prompt: &str, _context: &HashMap<String, Value>) -> Result<String, anyhow::Error> {
        println!("  [MockLLM] Processing: {}", prompt);
        Ok(format!("Mock response for: {}", prompt))
    }
    
    async fn structured_output(&self, _prompt: &str, _schema: &str) -> Result<Value, anyhow::Error> {
        Ok(serde_json::json!({ "decision": "proceed", "confidence": 0.95 }))
    }
    
    fn provider_name(&self) -> &str {
        "mock"
    }
}

/// The main Semantic Engine that bridges deterministic and AI execution
pub struct SemanticEngine {
    provider: Box<dyn LLMProvider>,
}

impl SemanticEngine {
    pub fn new(api_key: String, provider_name: String) -> Self {
        let provider: Box<dyn LLMProvider> = match provider_name.as_str() {
            "openai" => Box::new(OpenAIProvider::new(api_key, "gpt-4o".to_string())),
            _ => Box::new(MockProvider::new()),
        };
        Self { provider }
    }
    
    pub fn with_provider(provider: Box<dyn LLMProvider>) -> Self {
        Self { provider }
    }
    
    pub fn mock() -> Self {
        Self { provider: Box::new(MockProvider::new()) }
    }
    
    /// Achieve a natural language intent
    pub async fn achieve(&self, intent_description: &str, context: &HashMap<String, Value>) -> Result<Value, anyhow::Error> {
        println!("  [SemanticEngine] Achieving: {}", intent_description);
        let result = self.provider.complete(intent_description, context).await?;
        Ok(Value::String(result))
    }
    
    /// Extract structured data from text
    pub async fn extract<T: serde::de::DeserializeOwned>(&self, source_text: &str, schema: &str) -> Result<T, anyhow::Error> {
        let value = self.provider.structured_output(source_text, schema).await?;
        let result: T = serde_json::from_value(value)?;
        Ok(result)
    }
    
    pub fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }
}
