use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use colored::Colorize;

/// Trait for LLM provider implementations
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Send a completion request to the LLM
    async fn complete(
        &self,
        prompt: &str,
        context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error>;

    /// Get structured output from the LLM
    async fn structured_output(&self, prompt: &str, schema: &str) -> Result<Value, anyhow::Error>;

    /// Get the provider name
    fn provider_name(&self) -> &str;
}

/// Gemini provider
pub struct GeminiProvider {
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LLMProvider for GeminiProvider {
    async fn complete(
        &self,
        prompt: &str,
        context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error> {
        let mut full_prompt = String::new();
        if !context.is_empty() {
            full_prompt.push_str(&format!("Context: {}\n\n", serde_json::to_string(context).unwrap_or_default()));
        }
        full_prompt.push_str("Task: ");
        full_prompt.push_str(prompt);
        full_prompt.push_str("\nRespond concisely.");

        let client = reqwest::Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", self.model, self.api_key);
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "contents": [{
                    "parts": [{"text": full_prompt}]
                }]
            }))
            .send()
            .await?;

        let body: Value = response.json().await?;

        if let Some(err) = body.get("error") {
            let msg = err["message"].as_str().unwrap_or("Unknown API Error");
            return Err(anyhow::anyhow!("Gemini API Error: {}", msg));
        }

        let content = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response")
            .to_string();

        Ok(content)
    }

    async fn structured_output(
        &self,
        prompt: &str,
        schema: &str,
    ) -> Result<Value, anyhow::Error> {
        let full_prompt = format!(
            "{}\n\nRespond with valid JSON matching this schema:\n{}",
            prompt, schema
        );
        let result = self.complete(&full_prompt, &HashMap::new()).await?;
        let text = result.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        let value: Value = serde_json::from_str(text).unwrap_or(Value::String(result));
        Ok(value)
    }

    fn provider_name(&self) -> &str {
        "gemini"
    }
}

/// Claude (Anthropic) provider
pub struct ClaudeProvider {
    api_key: String,
    model: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LLMProvider for ClaudeProvider {
    async fn complete(
        &self,
        prompt: &str,
        context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error> {
        let mut system_prompt = String::from("You are an AI agent executing a task. Respond concisely.");
        if !context.is_empty() {
            system_prompt.push_str(&format!("\nContext: {}", serde_json::to_string(context).unwrap_or_default()));
        }

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "system": system_prompt,
                "max_tokens": 1024,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await?;

        let body: Value = response.json().await?;
        
        if let Some(err) = body.get("error") {
            let msg = err["message"].as_str().unwrap_or("Unknown API Error");
            return Err(anyhow::anyhow!("Claude API Error: {}", msg));
        }

        let content = body["content"][0]["text"]
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
        let text = result.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        let value: Value = serde_json::from_str(text).unwrap_or(Value::String(result));
        Ok(value)
    }

    fn provider_name(&self) -> &str {
        "claude"
    }
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
    async fn complete(
        &self,
        prompt: &str,
        context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error> {
        // Build the request with context
        let mut messages = vec![serde_json::json!({
            "role": "system",
            "content": "You are an AI agent executing a task. Respond concisely."
        })];

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
        let response = client
            .post(&self.endpoint)
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
        
        if let Some(err) = body.get("error") {
            let msg = err["message"].as_str().unwrap_or("Unknown API Error");
            return Err(anyhow::anyhow!("OpenAI API Error: {}", msg));
        }

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
        let value: Value = serde_json::from_str(&result).unwrap_or(Value::String(result));
        Ok(value)
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}

/// Mock provider for testing and offline development
pub struct MockProvider;

impl MockProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LLMProvider for MockProvider {
    async fn complete(
        &self,
        prompt: &str,
        _context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error> {
        let response = if prompt.contains("Classify ticket") {
            "Decision: Category is 'Refund'. Route to RefundAgent."
        } else if prompt.contains("Process refund requests") {
            "Response: \"I have processed your $50 refund. It should appear on your statement in 3-5 days.\""
        } else if prompt.contains("Handle high-risk situations") {
            "Action: Escalating ticket to human support tier 2."
        } else {
            "Mock response processed successfully."
        };
        
        // Simulate LLM latency
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
        Ok(response.to_string())
    }

    async fn structured_output(
        &self,
        _prompt: &str,
        _schema: &str,
    ) -> Result<Value, anyhow::Error> {
        Ok(serde_json::json!({ "decision": "proceed", "confidence": 0.95 }))
    }

    fn provider_name(&self) -> &str {
        "mock"
    }
}

/// Ollama-compatible provider (Local LLM)
pub struct OllamaProvider {
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: String) -> Self {
        Self {
            endpoint: "http://localhost:11434/api/generate".to_string(),
            model,
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn complete(
        &self,
        prompt: &str,
        context: &HashMap<String, Value>,
    ) -> Result<String, anyhow::Error> {
        let mut full_prompt = String::new();
        if !context.is_empty() {
            full_prompt.push_str(&format!("Context: {}\n\n", serde_json::to_string(context).unwrap_or_default()));
        }
        full_prompt.push_str(prompt);

        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": full_prompt,
                "stream": false
            }))
            .send()
            .await?;

        let body: Value = response.json().await?;
        let content = body["response"]
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
        let value: Value = serde_json::from_str(&result).unwrap_or(Value::String(result));
        Ok(value)
    }

    fn provider_name(&self) -> &str {
        "ollama"
    }
}

/// The main Semantic Engine that bridges deterministic and AI execution
pub struct SemanticEngine {
    provider: Box<dyn LLMProvider>,
}

impl SemanticEngine {
    pub fn new(api_key: String, provider_name: String) -> Self {
        let provider: Box<dyn LLMProvider> = match provider_name.as_str() {
            "gemini" => Box::new(GeminiProvider::new(api_key, "gemini-2.5-flash".to_string())),
            "claude" => Box::new(ClaudeProvider::new(api_key, "claude-3-5-sonnet-latest".to_string())),
            "grok" => Box::new(OpenAIProvider::new(api_key, "grok-2-latest".to_string()).with_endpoint("https://api.x.ai/v1/chat/completions".to_string())),
            "openai" => Box::new(OpenAIProvider::new(api_key, "gpt-4o".to_string())),
            "ollama" => Box::new(OllamaProvider::new("llama3".to_string())),
            _ => Box::new(MockProvider::new()),
        };
        Self { provider }
    }

    pub fn with_provider(provider: Box<dyn LLMProvider>) -> Self {
        Self { provider }
    }

    pub fn mock() -> Self {
        Self {
            provider: Box::new(MockProvider::new()),
        }
    }

    pub fn from_env() -> Self {
        if let Ok(key) = std::env::var("GEMINI_API_KEY") {
            if !key.is_empty() {
                println!("  {} Using Gemini Semantic Engine", "[System]".blue());
                return Self::new(key, "gemini".to_string());
            }
        }

        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                println!("  {} Using Claude Semantic Engine", "[System]".blue());
                return Self::new(key, "claude".to_string());
            }
        }

        if let Ok(key) = std::env::var("XAI_API_KEY") {
            if !key.is_empty() {
                println!("  {} Using Grok Semantic Engine", "[System]".blue());
                return Self::new(key, "grok".to_string());
            }
        }
        
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            if !key.is_empty() {
                println!("  {} Using OpenAI Semantic Engine", "[System]".blue());
                return Self::new(key, "openai".to_string());
            }
        }
        
        println!("  {} Using Mock Semantic Engine (No API Key set)", "[System]".blue());
        Self::mock()
    }

    /// Achieve a natural language intent
    pub async fn achieve(
        &self,
        intent_description: &str,
        context: &HashMap<String, Value>,
    ) -> Result<Value, anyhow::Error> {
        println!("  [SemanticEngine] Achieving: {}", intent_description);
        match self.provider.complete(intent_description, context).await {
            Ok(result) => {
                println!("    [SemanticEngine] Output: {}", result);
                Ok(Value::String(result))
            }
            Err(e) => {
                println!("    [SemanticEngine] Error: {}", e);
                Err(e)
            }
        }
    }

    /// Extract structured data from text
    pub async fn extract<T: serde::de::DeserializeOwned>(
        &self,
        source_text: &str,
        schema: &str,
    ) -> Result<T, anyhow::Error> {
        let value = self.provider.structured_output(source_text, schema).await?;
        let result: T = serde_json::from_value(value)?;
        Ok(result)
    }

    pub fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }
}
