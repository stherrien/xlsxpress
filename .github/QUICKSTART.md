# CI/CD Quick Start Guide

This guide helps you quickly test and verify the CI/CD setup locally before committing.

## Quick Validation

### 1. Validate Workflow YAML Files

```bash
# Using Python (works everywhere)
python3 -c "
import yaml
import glob

for file in glob.glob('.github/workflows/*.yml'):
    with open(file) as f:
        yaml.safe_load(f)
    print(f'✓ {file}')
print('All workflows valid!')
"
```

### 2. Test Local Build

```bash
# Install maturin if not already installed
pip install maturin[patchelf]

# Build the wheel
maturin build --release

# Check what was built
ls -lh target/wheels/

# Install the wheel
pip install target/wheels/*.whl --force-reinstall

# Test import
python -c "import xlsxpress; print('Import successful!')"
```

### 3. Run Comprehensive Local Tests

```bash
# Run Rust tests
cargo test

# Run Rust formatting check
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Build Python package in development mode
maturin develop --release

# Run Python tests (if they exist)
if [ -d "tests/python" ]; then
    pytest tests/python -v
fi
```

### 4. Test Smoke Tests

```python
# test_smoke.py
import xlsxpress

# Test basic writing
writer = xlsxpress.Writer()
sheet = writer.add_worksheet('Test')
writer.write_string(sheet, 0, 0, 'Hello')
writer.write_number(sheet, 1, 0, 42)
writer.write_boolean(sheet, 2, 0, True)
writer.save('test_basic.xlsx')
print('✓ Basic write test passed')

# Test with styles
from xlsxpress import Font, Fill, Style
font = Font().name('Arial').size(14.0).bold(True)
fill = Fill.solid('#FFFF00')
style = Style().font(font).fill(fill)
writer2 = xlsxpress.Writer()
sheet2 = writer2.add_worksheet('Styled')
writer2.write_string_with_style(sheet2, 0, 0, 'Styled', style)
writer2.save('test_styled.xlsx')
print('✓ Style test passed')

# Test with charts
from xlsxpress import LineChart, DataSeries, ChartPosition
writer3 = xlsxpress.Writer()
sheet3 = writer3.add_worksheet('Chart')
for i in range(10):
    writer3.write_number(sheet3, i, 0, i * 10)
chart = LineChart().title('Test Chart')
series = DataSeries('Chart!$A$1:$A$10').name('Data')
chart = chart.add_series(series)
position = ChartPosition(5, 2).width(640).height(480)
chart = chart.position(position)
writer3.insert_line_chart(sheet3, chart)
writer3.save('test_chart.xlsx')
print('✓ Chart test passed')

print('\n✓ All smoke tests passed!')
```

Run it:
```bash
python test_smoke.py
```

## Testing Workflows Locally with Act

[Act](https://github.com/nektos/act) allows you to run GitHub Actions locally.

### Install Act

```bash
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Windows (via Chocolatey)
choco install act-cli
```

### Run Workflows

```bash
# List all workflows
act -l

# Test the CI workflow (dry run)
act pull_request --dry-run

# Run CI workflow for real (uses Docker)
act pull_request

# Run specific job
act -j rust-test

# Run with secrets (create .secrets file first)
act -s GITHUB_TOKEN=your_token
```

**Note:** Act runs workflows in Docker containers, so it's close to the real GitHub Actions environment but not identical.

## Pre-Commit Checklist

Before pushing your changes, run:

```bash
#!/bin/bash
# save as check.sh and run: chmod +x check.sh && ./check.sh

set -e

echo "🔍 Running pre-commit checks..."

echo "\n📝 Checking Rust formatting..."
cargo fmt -- --check

echo "\n🔎 Running clippy..."
cargo clippy -- -D warnings

echo "\n🧪 Running Rust tests..."
cargo test

echo "\n🐍 Building Python package..."
maturin develop --release

echo "\n✨ Running smoke tests..."
python -c "
import xlsxpress
writer = xlsxpress.Writer()
sheet = writer.add_worksheet('Test')
writer.write_string(sheet, 0, 0, 'Test')
writer.save('test.xlsx')
print('Smoke test passed!')
"
rm -f test.xlsx

echo "\n✅ All checks passed! Ready to commit."
```

## Testing Release Process

### Test Build Locally

```bash
# Build for current platform
maturin build --release

# Build for multiple Python versions (if installed)
maturin build --release --interpreter python3.10 python3.11 python3.12

# Build sdist
maturin sdist

# Check artifacts
ls -lh target/wheels/
```

### Test Installation from Wheel

```bash
# Create fresh virtual environment
python -m venv test_env
source test_env/bin/activate  # or test_env\Scripts\activate on Windows

# Install from local wheel
pip install target/wheels/xlsxpress-*.whl

# Test
python -c "import xlsxpress; print(xlsxpress.__version__)"

# Deactivate and clean up
deactivate
rm -rf test_env
```

### Test PyPI Upload (Dry Run)

```bash
# Install twine
pip install twine

# Check package
twine check target/wheels/*

# Upload to Test PyPI (dry run)
# Note: This requires Test PyPI account and API token
twine upload --repository testpypi target/wheels/* --verbose

# Test installation from Test PyPI
pip install --index-url https://test.pypi.org/simple/ xlsxpress
```

## Common Issues and Solutions

### Issue: "maturin not found"
```bash
pip install maturin[patchelf]
# or
cargo install maturin
```

### Issue: "Rust compiler not found"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: "Import fails after build"
```bash
# Make sure you're in the right environment
which python
pip list | grep xlsxpress

# Rebuild and reinstall
maturin develop --release
```

### Issue: "Tests fail on import"
```bash
# Make sure package is installed in development mode
maturin develop --release

# Or install the wheel
pip install target/wheels/*.whl --force-reinstall
```

## Performance Testing

Test performance before release:

```python
import xlsxpress
import time

# Test write performance
start = time.time()
writer = xlsxpress.Writer()
sheet = writer.add_worksheet('Performance')

# Write 10,000 rows
for i in range(10000):
    writer.write_string(sheet, i, 0, f'Row {i}')
    writer.write_number(sheet, i, 1, i * 100)

writer.save('performance_test.xlsx')
elapsed = time.time() - start

print(f'Wrote 10,000 rows in {elapsed:.2f} seconds')
print(f'Rate: {10000/elapsed:.0f} rows/second')

# Clean up
import os
os.remove('performance_test.xlsx')
```

Expected performance: >10,000 rows/second

## Security Checks

```bash
# Run cargo audit
cargo install cargo-audit
cargo audit

# Check for outdated dependencies
cargo outdated

# Security scan with cargo-deny (optional)
cargo install cargo-deny
cargo deny check
```

## Next Steps

1. ✅ Validate workflows locally
2. ✅ Test builds on all platforms
3. ✅ Run smoke tests
4. 📤 Push to feature branch
5. 🔄 Create PR and wait for CI
6. 🎉 Merge when green
7. 🚀 Create release

## Resources

- [Maturin Guide](https://www.maturin.rs/)
- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Act (Local Testing)](https://github.com/nektos/act)
- [PyPI Publishing Guide](https://packaging.python.org/guides/publishing-package-distribution-releases-using-github-actions-ci-cd-workflows/)

---

**Ready to commit?** Make sure all checks pass!
