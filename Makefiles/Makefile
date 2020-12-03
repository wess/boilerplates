
ROOT_DIR 	   := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_DIR	 := $(notdir $(patsubst %/,%,$(dir $(ROOT_DIR))))
PROJECT 		 := $(lastword $(PROJECT_DIR))
VERSION_FILE 	= VERSION
VERSION			 	= `cat $(VERSION_FILE)`

RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(RUN_ARGS):;@:)

default: watch
### Cargo

RUST_CHANNEL ?= stable
CARGO_FLAGS =
RUSTUP_INSTALLED = $(shell command -v rustup 2> /dev/null)
TEST_FILTER ?=

ifndef RUSTUP_INSTALLED
  CARGO = cargo
else
  ifdef CI
    CARGO = cargo
  else
    CARGO = rustup run $(RUST_CHANNEL) cargo
  endif
endif

.PHONY: help
help: ## Print all the available commands
	@echo "" \
	&& grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' \
	&& echo ""

build: ## Build the project in debug mode
	@echo "Build project."

release: buid ## Build the project for release
	@echo "Build project for release"


.PHONY: test
test: ## Run Tests
	@echo "Test your code"

dev: ## Run development
	@echo "Run in development"

docs: ## Generate documentation
	@echo "Generate documentation"

clean: ## Clean project
	@echo "Clean project"

.PHONY: watch
watch: ## Watch project and build on change
	@echo "Run file watching to build on change."

.PHONY: setup
setup: ## Setup requirements needed for project
	@echo "Setup requirements needed for project, i.e. install anything."
	

