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

test: ## Test the project
	cargo test

doc: ## Generate the documentation
	cargo doc --no-deps --open

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"%