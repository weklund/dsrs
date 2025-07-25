use crate::errors::DSRSError;
use dotenvy::dotenv;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration; // Import from errors module

const DEFAULT_LLM_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
#[allow(dead_code)]
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
#[allow(dead_code)]
const DEFAULT_MAX_TOKENS: u32 = 1000;
#[allow(dead_code)]
const DEFAULT_TEMPERATURE: f32 = 0.7;
const MAX_PROMPT_LENGTH: usize = 32000; // ~8k tokens ≈ 32k chars
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// Request payload for the LLM API.
#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // New: Optional temperature for creativity
}

/// A single message in a chat conversation.
#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Response from LLM Chat Completions API.
#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    #[serde(default)] // Handle cases where error might be present
    pub error: Option<ApiError>, // New: Parse error field if present
}

/// A single choice/completion from the API response.
#[derive(Deserialize)]
pub struct Choice {
    pub message: MessageResponse,
}

/// The message content within a choice.
#[derive(Deserialize)]
pub struct MessageResponse {
    pub content: String,
}

#[derive(Deserialize)]
pub struct ApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
}

/// HTTP client for interacting with LLM providers via OpenAI-compatible API.
#[derive(Debug)]
pub struct LLMClient {
    client: Client,
}

impl Default for LLMClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMClient {
    /// Creates a new LLM client with default HTTP settings.
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client }
    }

    /// Sends a prompt to the LLM provider and returns the completion.
    pub async fn complete(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: Option<u32>,
        temperature: Option<f32>, // New param: Defaults to 0.7 if None
    ) -> Result<String, DSRSError> {
        dotenv().ok();
        let api_key = std::env::var("LLM_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .map_err(|err| {
                DSRSError::ConfigError(format!("LLM_API_KEY or OPENAI_API_KEY not set: {err}"))
            })?;

        if prompt.len() > MAX_PROMPT_LENGTH {
            return Err(DSRSError::PromptTooLong(prompt.len(), MAX_PROMPT_LENGTH));
        }
        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens,
            temperature,
        };

        let endpoint = std::env::var("LLM_ENDPOINT")
            .or_else(|_| std::env::var("OPENAI_API_ENDPOINT"))
            .unwrap_or_else(|_| DEFAULT_LLM_ENDPOINT.to_string());

        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|err| DSRSError::NetworkError(format!("Request failed: {err}")))?;

        if !response.status().is_success() {
            return Err(DSRSError::ApiError(format!("HTTP {}", response.status())));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|err| DSRSError::ApiError(format!("Failed to parse response: {err}")))?;

        // New: Check for embedded error in JSON
        if let Some(err) = chat_response.error {
            return Err(DSRSError::ApiError(format!(
                "{} (type: {}, code: {:?})",
                err.message, err.error_type, err.code
            )));
        }

        if chat_response.choices.is_empty() {
            return Err(DSRSError::ApiError(
                "No response choices returned".to_string(),
            ));
        }

        Ok(chat_response.choices[0].message.content.clone())
    }
}
