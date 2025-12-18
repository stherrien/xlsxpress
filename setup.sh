#!/bin/bash
# XlsXpress Development Setup Script

echo "================================================"
echo "XlsXpress Development Environment Setup"
echo "================================================"
echo ""

# 1. Create virtual environment
echo "1. Creating virtual environment..."
python3 -m venv .venv
echo "✓ Virtual environment created"
echo ""

# 2. Activate venv
echo "2. Activating virtual environment..."
source .venv/bin/activate
echo "✓ Virtual environment activated"
echo ""

# 3. Upgrade pip
echo "3. Upgrading pip..."
python -m pip install --upgrade pip
echo "✓ pip upgraded"
echo ""

# 4. Install maturin
echo "4. Installing maturin..."
pip install maturin
echo "✓ maturin installed"
echo ""

# 5. Build and install xlsxpress
echo "5. Building and installing xlsxpress..."
maturin develop --release
echo "✓ xlsxpress built and installed"
echo ""

# 6. Install dev dependencies
echo "6. Installing development dependencies..."
pip install pytest pytest-cov black ruff mypy bandit
echo "✓ Development dependencies installed"
echo ""

# 7. Run tests
echo "7. Running tests..."
cargo test --lib
echo ""
python -m pytest tests/python/ -v
echo ""

echo "================================================"
echo "✅ Setup Complete!"
echo "================================================"
echo ""
echo "To activate the virtual environment in the future:"
echo "  source .venv/bin/activate"
echo ""
echo "Then you can use:"
echo "  make test          # Run all tests"
echo "  make develop       # Rebuild after changes"
echo "  make               # Show all commands"
echo ""
