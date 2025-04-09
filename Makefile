.PHONY: help
help: ## Display this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help

test: ## Run tests
	@echo "Running tests..."

lint: ## Run linter
	@echo "Running linter..."

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."

wasm: ## Build WebAssembly
	@echo "Building WebAssembly..."
	@rm -rf pkg 
	@wasm-pack build --target web --out-dir pkg