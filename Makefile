# StellarPOS Smart Contracts Makefile

.PHONY: help build test clean deploy-testnet deploy-mainnet docs format lint

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build all contracts"
	@echo "  build-release  - Build optimized release version"
	@echo "  test          - Run all tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  deploy-testnet - Deploy contracts to testnet"
	@echo "  deploy-mainnet - Deploy contracts to mainnet"
	@echo "  docs          - Generate documentation"
	@echo "  format        - Format code"
	@echo "  lint          - Run linter"
	@echo "  setup         - Setup development environment"

# Build all contracts
build:
	@echo "Building all contracts..."
	cargo build --target wasm32-unknown-unknown --release

# Build optimized release
build-release:
	@echo "Building optimized release..."
	cargo build --target wasm32-unknown-unknown --release --profile release

# Run all tests
test:
	@echo "Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Deploy to testnet
deploy-testnet:
	@echo "Deploying to testnet..."
	./scripts/deploy-testnet.sh

# Deploy to mainnet  
deploy-mainnet:
	@echo "Deploying to mainnet..."
	./scripts/deploy-mainnet.sh

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --no-deps --open

# Format code
format:
	@echo "Formatting code..."
	cargo fmt

# Run linter
lint:
	@echo "Running linter..."
	cargo clippy -- -D warnings

# Setup development environment
setup:
	@echo "Setting up development environment..."
	rustup target add wasm32-unknown-unknown
	cargo install soroban-cli
	./scripts/setup-env.sh