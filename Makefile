.DEFAULT_GOAL := help

# Colors for terminal output
RED    := \033[0;31m
GREEN  := \033[0;32m
YELLOW := \033[1;33m
BLUE   := \033[0;34m
NC     := \033[0m

# Load variables from .env and export them to all sub-shells
-include .env
export

# --- Variables ---
# Default arguments to pass to cargo commands. Can be overridden from the command line.
# Example: make test-unit ARGS="my_test_name"
# Example: make test ARGS="-- --nocapture"
ARGS ?=

# Base command for running tests to avoid repetition
TEST_CMD_BASE := cargo test -p sqlk-tui --features "test-utils"

# Declare targets that are not files
.PHONY: clean help fmt fix clippy test quality-check run-app

clean:
	@echo "Clean"
	@cargo clean

help:
	@echo "$(BLUE)Available targets:$(NC) $(filter-out help,$(.PHONY))"

run-app:
	@echo "$(BLUE)🚀 Running app...$(NC)"
	@source ${env} && cargo run -p sqlk -- --env ${env} --file ${file} --toast-level ${level}

fmt:
	@echo "$(BLUE)🔎 Checking formatting...$(NC)"
	@cargo fmt --all -- --check

fix:
	@echo "$(BLUE)🎨 Formatting code...$(NC)"
	@cargo fmt --all
	@echo "$(BLUE)🔧 Fixing with clippy...$(NC)"
	@cargo clippy --fix --all-targets -- -D warnings

clippy:
	@echo "$(BLUE)📎 Checking code with Clippy...$(NC)"
	@cargo clippy --all-targets -- -D warnings

test:
	@echo "$(BLUE)🧪 Running tests...$(NC)"
	@$(TEST_CMD_BASE) $(ARGS)
	@echo "$(GREEN)✅ All primary tests passed!$(NC)"

quality-check: fmt clippy test
	@echo "$(GREEN)✅ All code quality checks passed!$(NC)"
