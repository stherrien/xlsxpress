# Developer Setup Guide

Quick guide to set up your XlsXpress development environment.

## Prerequisites

- **Rust** 1.70+ (install from https://rustup.rs/)
- **Python** 3.10+
- **Git**

## Quick Setup (Automated)

```bash
# Run the setup script
./setup.sh
```

This will:
1. Create a virtual environment at `.venv/`
2. Install maturin (Rust-Python build tool)
3. Build xlsxpress in release mode
4. Install development dependencies
5. Run all tests to verify setup

## Manual Setup

### 1. Create Virtual Environment

```bash
# Create venv
python3 -m venv .venv

# Activate it
source .venv/bin/activate  # macOS/Linux
# or
.venv\Scripts\activate     # Windows
```

### 2. Install Build Tools

```bash
pip install --upgrade pip
pip install maturin
```

### 3. Build and Install XlsXpress

```bash
# Build and install in development mode
maturin develop --release
```

### 4. Install Development Dependencies

```bash
pip install pytest pytest-cov black ruff mypy bandit
```

### 5. Verify Installation

```bash
# Test Python import
python -c "import xlsxpress; print(f'✓ xlsxpress {xlsxpress.__version__}')"

# Run tests
make test
```

## Using Make Commands

Once set up, you can use the Makefile for common tasks:

```bash
make                  # Show all available commands
make test             # Run all tests
make test-rust        # Run Rust tests only
make test-python      # Run Python tests only
make develop          # Rebuild after code changes
make check            # Run all quality checks
make clean            # Clean build artifacts
```

**Important:** Always activate your virtual environment before running make commands:
```bash
source .venv/bin/activate  # macOS/Linux
```

Or the Makefile will try to detect `.venv/bin/python` automatically.

## Development Workflow

1. **Make changes** to Rust code in `src/`
2. **Rebuild** with `make develop`
3. **Test** with `make test`
4. **Check quality** with `make check`
5. **Commit** your changes

### Running Tests

```bash
# All tests
make test

# Just Rust
make test-rust

# Just Python
make test-python

# With coverage
make test-cov
```

### Code Quality

```bash
# Run all checks (clippy + format + tests)
make check

# Just linting
make clippy          # Rust
make lint-python     # Python

# Format code
make fmt             # Rust
make format-python   # Python
```

## Troubleshooting

### "No module named 'xlsxpress'" when running tests

**Solution:** Make sure you've run `maturin develop` and are using the correct Python:

```bash
source .venv/bin/activate
maturin develop --release
python -m pytest tests/python/ -v
```

### Makefile uses wrong Python

The Makefile automatically detects `.venv/bin/python`. If this doesn't exist, it falls back to system Python.

**Solution:** Create the venv at `.venv/`:
```bash
make venv
source .venv/bin/activate
make develop
```

### Rust compilation errors

**Solution:** Make sure you have the latest stable Rust:
```bash
rustup update stable
cargo clean
make build
```

### Tests fail after code changes

**Solution:** Rebuild the Python bindings:
```bash
make develop
make test
```

## Project Structure

```
xlsxpress/
├── src/                  # Rust source code
│   ├── python/          # PyO3 Python bindings
│   ├── reader.rs        # Excel reader
│   ├── writer.rs        # Excel writer
│   └── ...
├── tests/
│   ├── fixtures/        # Test Excel files
│   └── python/          # Python tests
├── python/
│   └── xlsxpress/       # Python package
├── Makefile             # Development commands
└── setup.sh             # Automated setup script
```

## Next Steps

- Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines
- Check [PROJECT_STATUS.md](PROJECT_STATUS.md) for roadmap
- See [TODO.md](TODO.md) for open tasks

## Getting Help

- Run `make` to see all available commands
- Check the main [README.md](README.md) for usage examples
- Open an issue on GitHub for bugs/questions
