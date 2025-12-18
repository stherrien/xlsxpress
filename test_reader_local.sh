#!/bin/bash
# Local Reader MVP Testing Script
# Run this after setting up your development environment

set -e

echo "========================================="
echo "XlsXpress Reader MVP Testing"
echo "========================================="
echo ""

# Check prerequisites
echo "✓ Checking prerequisites..."
command -v cargo >/dev/null 2>&1 || { echo "✗ Rust not installed. Install from https://rustup.rs/"; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "✗ Python 3 not installed"; exit 1; }

echo "  ✓ Rust $(rustc --version | cut -d' ' -f2)"
echo "  ✓ Python $(python3 --version | cut -d' ' -f2)"
echo ""

# Build Rust library
echo "📦 Building Rust library..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "  ✓ Rust build successful"
else
    echo "  ✗ Rust build failed"
    exit 1
fi
echo ""

# Run Rust tests
echo "🧪 Running Rust tests..."
cargo test --release -- --nocapture
if [ $? -eq 0 ]; then
    echo "  ✓ All Rust tests passed"
else
    echo "  ✗ Rust tests failed"
    exit 1
fi
echo ""

# Check for test fixture
if [ ! -f "tests/fixtures/test.xlsx" ]; then
    echo "⚠️  Warning: tests/fixtures/test.xlsx not found"
    echo "   Creating test fixture..."
    mkdir -p tests/fixtures

    # Create a simple test Excel file using Python
    python3 - <<'EOF'
import xlsxpress
writer = xlsxpress.Writer()
sheet = writer.add_worksheet("Sheet1")
writer.write_string(sheet, 0, 0, "Hello")
writer.write_number(sheet, 0, 1, 42)
writer.write_string(sheet, 1, 0, "World")
writer.write_number(sheet, 1, 1, 100)
writer.save("tests/fixtures/test.xlsx")
print("  ✓ Created test fixture")
EOF
fi
echo ""

# Build Python package
echo "🐍 Building Python package with maturin..."
if ! command -v maturin &> /dev/null; then
    echo "  Installing maturin..."
    pip install maturin
fi
maturin develop --release
if [ $? -eq 0 ]; then
    echo "  ✓ Python package built successfully"
else
    echo "  ✗ Python package build failed"
    exit 1
fi
echo ""

# Run Python tests
echo "🧪 Running Python Reader tests..."
pytest tests/python/test_reader.py -v
if [ $? -eq 0 ]; then
    echo "  ✓ All Python tests passed"
else
    echo "  ✗ Python tests failed"
    exit 1
fi
echo ""

# Interactive test
echo "🔍 Running interactive Reader test..."
python3 - <<'EOF'
import xlsxpress

print("Opening test.xlsx...")
reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")

print(f"Sheet names: {reader.sheet_names()}")

ws = reader.worksheet("Sheet1")
rows, cols = ws.dimensions()
print(f"Dimensions: {rows} rows x {cols} columns")

print(f"Cell A1: {ws.get_value(0, 0)}")
print(f"Cell B1: {ws.get_number(0, 1)}")

print("\nAll data:")
data = ws.to_list()
for i, row in enumerate(data):
    print(f"  Row {i}: {row}")

print("\nIteration test:")
for i, row in enumerate(ws):
    print(f"  Row {i}: {row}")

print("\n✓ Interactive test completed successfully!")
EOF

echo ""
echo "========================================="
echo "✅ All Reader MVP tests passed!"
echo "========================================="
echo ""
echo "The Reader implementation is working correctly."
echo "You can now:"
echo "  1. Commit the changes"
echo "  2. Push to GitHub"
echo "  3. Test the CI/CD pipeline"
echo "  4. Prepare for release"
