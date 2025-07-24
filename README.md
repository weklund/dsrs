# DSRS - DSPy-like Rust CLI

A command-line tool for interacting with OpenAI's Chat Completions API, built with Rust for performance and safety.

**This is a phased approach for porting DSPy functionality to DSRS.  This is still very much in active development and it's not expected to cover DSPy use cases today.**

- [ ] Phase 1: Refresher on Rust by making super basic LLM client w/ github action ci.
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
- OpenAI API key

### Installation

```bash
git clone https://github.com/weklund/dsrs
cd dsrs
```

### Setup API Key

Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY='sk-your-key-here'
```

Or create a `.env` file:

```bash
echo "OPENAI_API_KEY=sk-your-key-here" > .env
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

- `OPENAI_API_KEY` - Your OpenAI API key (required)
- `OPENAI_API_ENDPOINT` - Custom API endpoint (optional, defaults to OpenAI)

### Supported Models

- `gpt-3.5-turbo` (default)
- `gpt-4`
- `gpt-4-turbo`
- Any OpenAI Chat Completions compatible model

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

