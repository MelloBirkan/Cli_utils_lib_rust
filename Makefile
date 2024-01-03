SHELL=/bin/zsh
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
clean: ## Clean the project
	cargo clean

build: ## Build the project
	cargo build

lint: ## Lint the project
	@rustup component add clippy 2> /dev/null
	cargo clippy

format: ## Format the project
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

