# DSRS - DSPy-like Rust CLI

A command-line tool for interacting with LLM providers via OpenAI-compatible APIs, built with Rust for performance and safety.

**This is a phased approach for porting DSPy functionality to DSRS.  This is still very much in active development and it's not expected to cover DSPy use cases today.**

- [x] Phase 1: Refresher on Rust by making super basic LLM client w/ github action ci.
- [ ] Phase 2: DS signatures and basic prediction
- [ ] Phase 3: DS modules and composition
- [ ] Phase 4: DS metrics and evaulation
- [ ] Phase 5: DS basic opitimization
- [ ] Phase 6: DS advanced optimization

## Features

- 🚀 **Fast & Safe** - Built in Rust with memory safety and performance
- 🎛️ **Configurable** - Support for different models, token limits, and endpoints
- 📖 **Well-documented** - Full API documentation and usage examples

## Quick Start

### Prerequisites

- Rust 1.74+ (install via [rustup](https://rustup.rs/))
- API key from your preferred LLM provider

### Installation

```bash
git clone https://github.com/weklund/dsrs
cd dsrs
```

### Setup API Key

Set your LLM provider API key as an environment variable:

```bash
export LLM_API_KEY='your-api-key-here'
```

Or create a `.env` file:

```bash
echo "LLM_API_KEY=your-api-key-here" > .env
```

**For different providers:**
```bash
# OpenAI (default)
LLM_API_KEY=sk-your-openai-key
LLM_ENDPOINT=https://api.openai.com/v1/chat/completions

# Together AI
LLM_API_KEY=your-together-key
LLM_ENDPOINT=https://api.together.xyz/v1/chat/completions

# OpenRouter
LLM_API_KEY=sk-or-your-openrouter-key
LLM_ENDPOINT=https://openrouter.ai/api/v1/chat/completions

# Local model (e.g., LM Studio)
LLM_API_KEY=not-needed
LLM_ENDPOINT=http://localhost:1234/v1/chat/completions
```

### Development Setup

If you have [just](https://github.com/casey/just) installed:

```bash
just setup    # Install development tools
just check    # Run all checks (format, lint, test)
```

Otherwise:

```bash
rustup component add clippy rustfmt
cargo build
```

## Usage

### Basic Examples

```bash
# Simple question
cargo run -- --prompt "What is the capital of France?"

# With custom model
cargo run -- --prompt "Explain quantum computing" --model "gpt-4"

# With token limit
cargo run -- --prompt "Write a haiku about programming" --max-tokens 50
```

### Using Just Commands (Recommended)

```bash
# Quick examples
just example-simple     # Run a simple question
just example-gpt4       # Use GPT-4 model
just example-code       # Ask a coding question  
just example-help       # Show CLI help

# See all available commands
just --list
```

### CLI Options

```bash
dsrs --help
```

```
Usage: dsrs [OPTIONS] --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>          The prompt to send to the AI model
      --max-tokens <MAX_TOKENS>  Maximum number of tokens in the response [default: 1000]
      --model <MODEL>            AI model to use (e.g., gpt-3.5-turbo, gpt-4) [default: gpt-3.5-turbo]
  -h, --help                     Print help
```

## Development

### Common Commands

```bash
# Development workflow
just check              # Run format, lint, and tests
just format             # Format code
just lint               # Run clippy linter
just test               # Run tests

# Building
just build              # Debug build
just build-release      # Release build

# Documentation
just docs               # Generate and open docs
```

### Testing

```bash
just test               # Run all tests
just test-verbose       # Run tests with output
```

### Code Quality

The project uses strict linting rules to ensure code quality:

- **Clippy** with `correctness` and `suspicious` as errors
- **No unsafe code** (except in tests)
- **Comprehensive error handling**
- **Performance and complexity lints**

## Configuration

### Environment Variables

**Primary (recommended):**
- `LLM_API_KEY` - Your LLM provider API key (required)
- `LLM_ENDPOINT` - API endpoint URL (optional, defaults to OpenAI)


### Supported Providers & Models

**OpenAI:**
- `gpt-3.5-turbo` (default)
- `gpt-4`, `gpt-4-turbo`, `gpt-4o`

**Together AI:**
- `meta-llama/Llama-2-70b-chat-hf`
- `mistralai/Mixtral-8x7B-Instruct-v0.1`
- `NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO`

**OpenRouter:**
- `anthropic/claude-3-sonnet`
- `google/gemini-pro`
- Any model available on OpenRouter

**Local Models:**
- Any model running locally (e.g., via LM Studio, Ollama with OpenAI compatibility)

*Any provider that supports the OpenAI Chat Completions API format*

## Security

- API keys are loaded just-in-time, never stored in memory
- Custom error types prevent sensitive information leakage
- Input validation prevents expensive/malicious requests
- No unsafe code in production paths

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just check` to ensure all tests pass
5. Submit a pull request

