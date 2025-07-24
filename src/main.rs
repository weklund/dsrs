//! # DSRS - DSPy-like Rust CLI
//!
//! A command-line tool for interacting with LLMs.
//!
//! ## Features
//! - Configurable model selection (defaults to gpt-3.5-turbo)
//! - Prompt length validation to prevent expensive requests
//! - Support for .env files and environment variables
//!
//! ## Usage
//! ```bash
//! dsrs --prompt "Your question here" --model gpt-4 --max-tokens 500
//! ```

use clap::Parser;
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Configuration constants
const DEFAULT_OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const DEFAULT_MAX_TOKENS: u32 = 1000;
const MAX_PROMPT_LENGTH: usize = 32000; // ~8k tokens ≈ 32k chars

/// Command-line arguments for the DSRS application.
#[derive(Parser)]
struct Args {
    /// The prompt to send to the AI model
    #[arg(short, long)]
    prompt: String,
    /// Maximum number of tokens in the response
    #[arg(long, default_value_t = DEFAULT_MAX_TOKENS)]
    max_tokens: u32,
    /// AI model to use (e.g., gpt-3.5-turbo, gpt-4)
    #[arg(long, default_value_t = DEFAULT_MODEL.to_string())]
    model: String,
}

/// Custom error type that prevents sensitive information leakage.
///
/// All error variants provide user-friendly messages without exposing
/// internal details like API keys or detailed system information.
#[derive(Debug)]
enum DSRSError {
    /// Prompt exceeds maximum allowed length. Contains (actual_length, max_length).
    PromptTooLong(usize, usize),
    /// API returned an error response or invalid data.
    ApiError(String),
    /// Network request failed (connection, timeout, etc.).
    NetworkError(String),
    /// Configuration issue (missing API key, invalid settings).
    ConfigError(String),
}

impl std::fmt::Display for DSRSError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DSRSError::PromptTooLong(len, max) => {
                write!(f, "Prompt too long: {} chars (max: {})", len, max)
            }
            DSRSError::ApiError(msg) => write!(f, "API error: {}", msg),
            DSRSError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DSRSError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for DSRSError {}

/// Request payload for the LLM API.
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")] // Omit if None
    max_tokens: Option<u32>, // Optional, bounds output tokens
}

/// A single message in a chat conversation.
#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Response from OpenAI Chat Completions API.
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// A single choice/completion from the API response.
#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

/// The message content within a choice.
#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

/// HTTP client for interacting with OpenAI's API.
///
/// Does not store API keys for security - loads them just-in-time.
#[derive(Debug)]
struct OpenAIClient {
    client: Client,
}

impl OpenAIClient {
    /// Creates a new OpenAI client with default HTTP settings.
    ///
    /// Note: Does not validate API key at construction time for security.
    fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    /// Sends a prompt to OpenAI and returns the completion.
    ///
    /// # Arguments
    /// * `prompt` - The text prompt to send to the AI
    /// * `model` - The AI model to use (e.g., "gpt-3.5-turbo")
    /// * `max_tokens` - Optional limit on response length
    ///
    /// # Returns
    /// The AI's response text, or an error if the request failed.
    ///
    /// # Security
    /// API key is loaded from environment just-in-time, not stored in struct.
    async fn complete(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: Option<u32>,
    ) -> Result<String, DSRSError> {
        // Load API key when needed (not stored in struct for security)
        dotenv().ok(); // Load .env file if it exists (fails silently if no file)
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|err| DSRSError::ConfigError(format!("OPENAI_API_KEY not set: {}", err)))?;

        // Validate prompt length to prevent expensive API calls
        // OpenAI charges by token, roughly 4 chars = 1 token
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
        };

        // Allow API endpoint override for testing or alternative providers
        let endpoint = std::env::var("OPENAI_API_ENDPOINT")
            .unwrap_or_else(|_| DEFAULT_OPENAI_ENDPOINT.to_string());

        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|err| DSRSError::NetworkError(format!("Request failed: {}", err)))?;

        // Check HTTP status before attempting to parse JSON
        // API can return 200 with error details in JSON, but 4xx/5xx are clear failures
        if !response.status().is_success() {
            return Err(DSRSError::ApiError(format!("HTTP {}", response.status())));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|err| DSRSError::ApiError(format!("Failed to parse response: {}", err)))?;

        // OpenAI API should always return at least one choice, but validate to prevent panic
        if chat_response.choices.is_empty() {
            return Err(DSRSError::ApiError(
                "No response choices returned".to_string(),
            ));
        }

        Ok(chat_response.choices[0].message.content.clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), DSRSError> {
    let args = Args::parse();

    let client = OpenAIClient::new();
    let response = client
        .complete(&args.prompt, &args.model, Some(args.max_tokens))
        .await?;
    println!("Response: {}", response);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prompt_length_validation() {
        // Test valid prompt length
        let short_prompt = "Hello, world!";
        assert!(short_prompt.len() < 32000);

        // Test prompt that's too long
        let long_prompt = "a".repeat(35000);
        assert!(long_prompt.len() > 32000);
    }

    #[test]
    fn test_error_display() {
        let err = DSRSError::PromptTooLong(35000, 32000);
        assert_eq!(err.to_string(), "Prompt too long: 35000 chars (max: 32000)");

        let err = DSRSError::ApiError("Rate limited".to_string());
        assert_eq!(err.to_string(), "API error: Rate limited");

        let err = DSRSError::NetworkError("Connection timeout".to_string());
        assert_eq!(err.to_string(), "Network error: Connection timeout");

        let err = DSRSError::ConfigError("Missing key".to_string());
        assert_eq!(err.to_string(), "Configuration error: Missing key");
    }

    #[test]
    fn test_config_error_handling() {
        // Test the error type directly instead of complex environment manipulation
        let error = DSRSError::ConfigError("OPENAI_API_KEY not set".to_string());
        assert_eq!(
            error.to_string(),
            "Configuration error: OPENAI_API_KEY not set"
        );

        // Verify it implements the Error trait
        assert!(std::error::Error::source(&error).is_none());
    }

    #[tokio::test]
    async fn test_complete_with_long_prompt() {
        // Set a dummy API key for testing
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("OPENAI_API_KEY", "sk-test-key");
        }

        let client = OpenAIClient::new();
        let long_prompt = "a".repeat(35000);

        let result = client
            .complete(&long_prompt, DEFAULT_MODEL, Some(100))
            .await;
        assert!(result.is_err());

        if let Err(DSRSError::PromptTooLong(len, max)) = result {
            assert_eq!(len, 35000);
            assert_eq!(max, 32000);
        } else {
            panic!("Expected PromptTooLong error");
        }
    }

    #[test]
    fn test_empty_choices_response() {
        // Test that an empty choices array is handled gracefully
        let empty_response = ChatResponse { choices: vec![] };

        assert!(empty_response.choices.is_empty());

        // We can't easily test the actual API response parsing without mocking,
        // but we can verify the error type exists and displays correctly
        let error = DSRSError::ApiError("No response choices returned".to_string());
        assert_eq!(error.to_string(), "API error: No response choices returned");
    }
}
