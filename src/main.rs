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
use dsrs::{client::LLMClient, errors::DSRSError};

// Configuration constants
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const DEFAULT_MAX_TOKENS: u32 = 1000;

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

#[tokio::main]
async fn main() -> Result<(), DSRSError> {
    let args = Args::parse();

    let client = LLMClient::new();
    let response = client
        .complete(&args.prompt, &args.model, Some(args.max_tokens), None)
        .await?;
    println!("Response: {response}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[allow(unsafe_code)]
    async fn test_complete_with_long_prompt() {
        // Set a dummy API key for testing so we get to the prompt length check
        unsafe {
            std::env::set_var("LLM_API_KEY", "test-key-for-prompt-length-validation");
        }

        let client = LLMClient::new();
        let long_prompt = "a".repeat(35000); // Exceeds 32k limit
        let result = client
            .complete(&long_prompt, "gpt-3.5-turbo", Some(100), None)
            .await;

        match result {
            Err(DSRSError::PromptTooLong(len, max)) => {
                assert_eq!(len, 35000);
                assert_eq!(max, 32000);
            }
            other => {
                panic!("Expected PromptTooLong error, got: {:?}", other);
            }
        }

        // Clean up
        unsafe {
            std::env::remove_var("LLM_API_KEY");
        }
    }

    #[test]
    fn test_error_display() {
        // We can't easily test the actual API response parsing without mocking,
        // but we can verify the error type exists and displays correctly
        let error = DSRSError::ApiError("No response choices returned".to_string());
        assert_eq!(error.to_string(), "API error: No response choices returned");
    }
}
