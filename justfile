# DSRS Development Commands
# Run `just --list` to see all available commands

# Default recipe - show help
default:
    @just --list

# == Development Commands ==

# Run all checks (format, lint, test) - use before committing
check: format lint test
    @echo "✅ All checks passed!"

# Format code with rustfmt
format:
    @echo "🎨 Formatting code..."
    cargo fmt

# Check if code is formatted (for CI)
format-check:
    @echo "🔍 Checking code formatting..."
    cargo fmt --check

# Run clippy linter with strict settings
lint:
    @echo "🧹 Running clippy linter..."
    cargo clippy -- -D warnings

# Run all tests
test:
    @echo "🧪 Running tests..."
    cargo test

# Run tests with output
test-verbose:
    @echo "🧪 Running tests with output..."
    cargo test -- --nocapture

# Build the project
build:
    @echo "🔨 Building project..."
    cargo build

# Build in release mode
build-release:
    @echo "🔨 Building project in release mode..."
    cargo build --release

# Clean build artifacts
clean:
    @echo "🧽 Cleaning build artifacts..."
    cargo clean

# == Documentation Commands ==

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cargo doc --no-deps --open

# Generate documentation without opening
docs-build:
    @echo "📚 Generating documentation..."
    cargo doc --no-deps

# == Example Usage Commands ==

# Run a simple question
example-simple:
    @echo "💬 Running simple example..."
    cargo run -- --prompt "What is the capital of France?"

# Run with custom model
example-gpt4:
    @echo "💬 Running GPT-4 example..."
    cargo run -- --prompt "Explain quantum computing in simple terms" --model "gpt-4"

# Run with token limit
example-short:
    @echo "💬 Running example with token limit..."
    cargo run -- --prompt "Write a haiku about programming" --max-tokens 50

# Run a coding question
example-code:
    @echo "💬 Running coding example..."
    cargo run -- --prompt "Write a Python function to reverse a string" --max-tokens 200

# Show help
example-help:
    @echo "💬 Showing CLI help..."
    cargo run -- --help

# == CI/CD Commands ==

# Full CI pipeline (what runs in continuous integration)
ci: format-check lint test build
    @echo "🚀 CI pipeline completed successfully!"

# Pre-commit hook simulation
pre-commit: format lint test
    @echo "✅ Pre-commit checks passed!"

# Security audit
audit:
    @echo "🔒 Running security audit..."
    cargo audit

# Check for outdated dependencies
outdated:
    @echo "📦 Checking for outdated dependencies..."
    cargo outdated

# Update dependencies
update:
    @echo "📦 Updating dependencies..."
    cargo update

# == Installation Commands ==

# Install the binary locally
install:
    @echo "📦 Installing dsrs locally..."
    cargo install --path .

# Uninstall the binary
uninstall:
    @echo "🗑️  Uninstalling dsrs..."
    cargo uninstall dsrs

# == Utility Commands ==

# Watch for changes and run tests
watch-test:
    @echo "👀 Watching for changes and running tests..."
    cargo watch -x test

# Show project statistics
stats:
    @echo "📊 Project statistics:"
    @echo "Lines of code:"
    @find src -name "*.rs" | xargs wc -l | tail -1
    @echo "Dependencies:"
    @cargo tree --depth 1
    @echo "Binary size (debug):"
    @ls -lh target/debug/dsrs 2>/dev/null || echo "Not built yet (run 'just build')"
    @echo "Binary size (release):"
    @ls -lh target/release/dsrs 2>/dev/null || echo "Not built yet (run 'just build-release')"

# Benchmark the CLI (requires API key)
benchmark:
    @echo "⏱️  Running simple benchmark..."
    @echo "Testing response time for simple queries..."
    time cargo run --release -- --prompt "Hello" --max-tokens 10

# == Environment Setup ==

# Check if all required tools are installed
check-env:
    @echo "🔧 Checking development environment..."
    @which cargo > /dev/null || echo "❌ cargo not found"
    @which rustc > /dev/null || echo "❌ rustc not found" 
    @which clippy-driver > /dev/null || echo "❌ clippy not found (run: rustup component add clippy)"
    @which rustfmt > /dev/null || echo "❌ rustfmt not found (run: rustup component add rustfmt)"
    @echo "✅ Environment check completed"

# Setup development environment
setup: check-env
    @echo "🔧 Setting up development environment..."
    rustup component add clippy rustfmt
    @echo "✅ Development environment ready!"