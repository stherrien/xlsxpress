# XlsXpress Makefile
# High-performance Excel processing library built with Rust for Python

# Detect Python - prefer venv, fallback to system
VENV_PYTHON := $(shell [ -f .venv/bin/python ] && echo .venv/bin/python || echo python)
PYTHON := $(VENV_PYTHON)

.PHONY: help
help: ## Show this help message
	@echo "XlsXpress - Development Commands"
	@echo "=================================="
	@echo ""
	@echo "Python: $(PYTHON)"
	@echo ""
	@echo "Usage: make [command]"
	@echo ""
	@echo "Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Note: If using a virtual environment, activate it first or ensure .venv/bin/python exists"
	@echo ""

.PHONY: venv
venv: ## Create virtual environment
	@echo "Creating virtual environment..."
	python3 -m venv .venv
	@echo "✓ Virtual environment created at .venv/"
	@echo ""
	@echo "To activate:"
	@echo "  source .venv/bin/activate  # On macOS/Linux"
	@echo "  .venv\\Scripts\\activate     # On Windows"

.PHONY: install
install: ## Install Rust and Python dependencies
	@echo "Installing Rust dependencies..."
	rustup update stable
	@echo "Installing maturin..."
	$(PYTHON) -m pip install maturin
	@echo "✓ Dependencies installed"

.PHONY: build
build: ## Build the Rust library (debug mode)
	@echo "Building Rust library..."
	cargo build
	@echo "✓ Build complete"

.PHONY: build-release
build-release: ## Build the Rust library (release mode)
	@echo "Building Rust library (release)..."
	cargo build --release
	@echo "✓ Release build complete"

.PHONY: develop
develop: ## Build and install Python package for development
	@echo "Building and installing Python package..."
	maturin develop --release
	@echo "✓ Package installed for development"

.PHONY: test
test: ## Run all tests (Rust + Python)
	@echo "Running Rust tests..."
	cargo test --lib
	@echo ""
	@echo "Running Python tests..."
	@echo "Using Python: $(PYTHON)"
	$(PYTHON) -m pytest tests/python/ -v
	@echo "✓ All tests passed"

.PHONY: test-rust
test-rust: ## Run Rust tests only
	@echo "Running Rust tests..."
	cargo test --lib
	@echo "✓ Rust tests passed"

.PHONY: test-python
test-python: ## Run Python tests only
	@echo "Running Python tests..."
	@echo "Using Python: $(PYTHON)"
	$(PYTHON) -m pytest tests/python/ -v
	@echo "✓ Python tests passed"

.PHONY: test-cov
test-cov: ## Run tests with coverage report
	@echo "Running Rust tests with coverage..."
	cargo test --lib
	@echo ""
	@echo "Running Python tests with coverage..."
	@echo "Using Python: $(PYTHON)"
	$(PYTHON) -m pytest tests/python/ -v --cov=xlsxpress --cov-report=html --cov-report=term-missing
	@echo "✓ Coverage report generated in htmlcov/"

.PHONY: check
check: ## Run all quality checks (clippy, format, tests)
	@echo "Running clippy..."
	cargo clippy -- -D warnings
	@echo ""
	@echo "Checking Rust formatting..."
	cargo fmt --check
	@echo ""
	@echo "Running tests..."
	cargo test --lib
	@echo "✓ All checks passed"

.PHONY: clippy
clippy: ## Run Rust clippy linter
	@echo "Running clippy..."
	cargo clippy -- -D warnings
	@echo "✓ Clippy passed"

.PHONY: fmt
fmt: ## Format Rust code
	@echo "Formatting Rust code..."
	cargo fmt
	@echo "✓ Code formatted"

.PHONY: fmt-check
fmt-check: ## Check Rust code formatting
	@echo "Checking Rust formatting..."
	cargo fmt --check
	@echo "✓ Formatting is correct"

.PHONY: clean
clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/
	rm -rf htmlcov/
	rm -rf .pytest_cache/
	rm -rf python/xlsxpress/*.so
	rm -rf python/xlsxpress/__pycache__/
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true
	@echo "✓ Clean complete"

.PHONY: fixtures
fixtures: ## Create test fixtures
	@echo "Creating test fixtures..."
	@echo "Using Python: $(PYTHON)"
	$(PYTHON) -m pip install -q maturin
	maturin develop --release --quiet
	$(PYTHON) -c "import xlsxpress; \
		w = xlsxpress.Writer(); \
		s = w.add_worksheet('Sheet1'); \
		w.write_string(s, 0, 0, 'Hello'); \
		w.write_number(s, 0, 1, 42.0); \
		w.write_string(s, 1, 0, 'World'); \
		w.write_number(s, 1, 1, 3.14); \
		w.write_string(s, 2, 0, 'Test'); \
		w.write_number(s, 2, 1, 100.0); \
		w.save('tests/fixtures/test.xlsx')"
	@echo "✓ Test fixtures created"

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	cargo bench
	@echo "✓ Benchmarks complete"

.PHONY: doc
doc: ## Generate Rust documentation
	@echo "Generating Rust documentation..."
	cargo doc --no-deps --open
	@echo "✓ Documentation generated"

.PHONY: wheel
wheel: ## Build Python wheel
	@echo "Building Python wheel..."
	maturin build --release
	@echo "✓ Wheel built in target/wheels/"

.PHONY: install-dev
install-dev: ## Install development dependencies
	@echo "Installing development dependencies..."
	$(PYTHON) -m pip install pytest pytest-cov black ruff mypy bandit pre-commit
	@echo "✓ Development dependencies installed"

.PHONY: lint-python
lint-python: ## Lint Python code
	@echo "Running black..."
	black --check python/
	@echo ""
	@echo "Running ruff..."
	ruff check python/
	@echo "✓ Python linting passed"

.PHONY: format-python
format-python: ## Format Python code
	@echo "Formatting Python code..."
	black python/
	@echo "✓ Python code formatted"

.PHONY: audit
audit: ## Run security audit
	@echo "Running cargo audit..."
	cargo audit
	@echo ""
	@echo "Running bandit..."
	bandit -r python/ -ll
	@echo "✓ Security audit complete"

.PHONY: all
all: clean build test check ## Clean, build, test, and check everything
	@echo "✓ All tasks complete"

.PHONY: ci
ci: check test-cov ## Run CI checks (used in CI/CD)
	@echo "✓ CI checks passed"

.PHONY: release-local
release-local: clean check test wheel ## Prepare for local release
	@echo "✓ Ready for local release"
	@echo ""
	@echo "Wheels available in: target/wheels/"

# Default target
.DEFAULT_GOAL := help
