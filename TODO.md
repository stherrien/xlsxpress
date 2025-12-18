# XlsXpress Development TODO

## Development Approach

**CRITICAL: This project follows Test-Driven Development (TDD) and Clean Code principles.**

### TDD Workflow (Red-Green-Refactor)
For EVERY feature, follow this cycle strictly:
1. **RED**: Write a failing test first
2. **GREEN**: Write minimal code to pass the test
3. **REFACTOR**: Improve code quality (DRY, SOLID, cognitive complexity ≤15)
4. **REPEAT**: Continue for next feature

### Code Quality Standards
- **Clean Code**: Functions ≤20 lines, meaningful names, single responsibility
- **DRY**: No code duplication, extract common logic
- **SOLID**: Follow all five principles (see REQUIREMENTS.md §7.4)
- **Cognitive Complexity**: Keep ≤15 per function (see REQUIREMENTS.md §7.6)
- **Coverage**: Minimum 90% test coverage, 100% for critical paths

### Before Writing ANY Code
- [ ] Write the test first (TDD Red phase)
- [ ] Ensure test fails for the right reason
- [ ] Write minimal implementation (TDD Green phase)
- [ ] Refactor while keeping tests green
- [ ] Run clippy/ruff to check code quality
- [ ] Verify cognitive complexity ≤15

---

## Phase 1: Project Setup

### Repository Structure
- [ ] Initialize Rust workspace with `cargo init --lib`
- [ ] Create `Cargo.toml` with workspace configuration
- [ ] Set up `pyproject.toml` for maturin builds
- [ ] Create directory structure:
  ```
  xlsxpress/
  ├── src/              # Rust source code
  │   ├── lib.rs        # Library root
  │   ├── reader/       # Reading functionality
  │   ├── writer/       # Writing functionality
  │   └── python/       # PyO3 bindings
  ├── python/           # Python package
  │   └── xlsxpress/
  │       ├── __init__.py
  │       └── compat/   # OpenPyXL compatibility
  ├── tests/            # Test suites
  │   ├── rust/         # Rust unit tests
  │   └── python/       # Python integration tests
  ├── benches/          # Performance benchmarks
  └── docs/             # Documentation
  ```

### Development Environment
- [ ] Add `.gitignore` for Rust/Python artifacts
- [ ] Create `rust-toolchain.toml` (stable channel)
- [ ] Set up pre-commit hooks (rustfmt, clippy, black, ruff, mypy)
- [ ] Configure CI/CD pipeline (GitHub Actions):
  - [ ] Rust tests + coverage (cargo-tarpaulin)
  - [ ] Python tests + coverage (pytest-cov)
  - [ ] Code quality checks (clippy deny warnings, ruff, mypy strict)
  - [ ] Cognitive complexity check
  - [ ] Security scans (cargo-audit, bandit)
- [ ] Add `CONTRIBUTING.md` with TDD and coding standards
- [ ] Install code quality tools:
  - [ ] `cargo install cargo-tarpaulin cargo-audit cargo-deny`
  - [ ] `pip install pytest pytest-cov black ruff mypy bandit`

### Dependencies
- [ ] Add Rust dependencies to `Cargo.toml`:
  - `pyo3 = { version = "0.20", features = ["extension-module"] }`
  - `calamine = "0.24"`
  - `rust_xlsxwriter = "0.64"`
  - `serde = { version = "1.0", features = ["derive"] }`
  - `chrono = "0.4"`
  - `rayon = "1.8"`
  - `thiserror = "1.0"`
- [ ] Configure maturin in `pyproject.toml`
- [ ] Set up development dependencies (pytest, numpy, pandas)

---

## Phase 2: Core Reading (calamine Integration)

**TDD Reminder**: Write tests BEFORE implementation for each feature below.

### Basic Reading
- [ ] **TEST**: Write test for opening .xlsx file
- [ ] **CODE**: Create `Reader` struct wrapping calamine
- [ ] **TEST**: Write tests for file opening (path and bytes)
- [ ] **CODE**: Implement file opening (path and bytes)
- [ ] **TEST**: Write tests for format detection
- [ ] **CODE**: Support format detection (.xlsx, .xlsm, .xls, .xlsb, .ods)
- [ ] **TEST**: Write tests for sheet operations
- [ ] **CODE**: Implement sheet listing and access by name/index
- [ ] **TDD**: Read cell values with proper type conversion:
  - [ ] **TEST**: String cell test → **CODE**: String parsing
  - [ ] **TEST**: Number cell tests → **CODE**: Number parsing
  - [ ] **TEST**: Boolean cell tests → **CODE**: Boolean parsing
  - [ ] **TEST**: Date cell tests → **CODE**: Date/timezone handling
  - [ ] **TEST**: Formula cell tests → **CODE**: Formula as string
  - [ ] **TEST**: Error cell tests → **CODE**: Error enum
  - [ ] **TEST**: Empty cell tests → **CODE**: Empty/blank handling

### Sheet Operations
- [ ] Get sheet dimensions (used range)
- [ ] Read specific ranges (A1:B10 notation)
- [ ] Handle merged cells
- [ ] Read sheet metadata (hidden, protection status)

### Streaming Mode
- [ ] Implement row iterator for memory efficiency
- [ ] Support `skip_empty_area` for sparse data
- [ ] Add progress callbacks for large files

### Python Bindings (Reading)
- [ ] Create `Workbook` Python class
- [ ] Create `Worksheet` Python class
- [ ] Implement `__iter__` for row iteration
- [ ] Add `to_list()` method (nested lists)
- [ ] Add `to_dict()` method (list of dicts)
- [ ] Add `to_dataframe()` method (pandas integration)
- [ ] Implement NumPy array conversion

---

## Phase 3: Core Writing (rust_xlsxwriter Integration)

### Basic Writing
- [ ] Create `Writer` struct wrapping rust_xlsxwriter
- [ ] Implement workbook creation
- [ ] Add/remove/rename worksheets
- [ ] Write cell values with type inference:
  - [ ] Strings
  - [ ] Numbers
  - [ ] Booleans
  - [ ] Dates/times
  - [ ] Formulas
  - [ ] URLs/hyperlinks
- [ ] Save to file path or bytes

### Range Operations
- [ ] Write 2D arrays to ranges
- [ ] Merge cells
- [ ] Set column widths
- [ ] Set row heights
- [ ] Freeze panes
- [ ] Auto-filter setup

### Streaming Mode
- [ ] Implement constant-memory row appending
- [ ] Support unlimited row counts
- [ ] Add flush controls for memory management

### Python Bindings (Writing)
- [ ] Create `WorkbookWriter` Python class
- [ ] Create `WorksheetWriter` Python class
- [ ] Implement context manager protocol (`__enter__`, `__exit__`)
- [ ] Add `write_dataframe()` method
- [ ] Add `write_array()` method (NumPy)
- [ ] Support method chaining for ergonomic API

---

## Phase 4: OpenPyXL Compatibility Layer

### Core Classes
- [ ] Implement `openpyxl.Workbook` wrapper
- [ ] Implement `openpyxl.load_workbook()` function
- [ ] Implement `Worksheet` with cell access patterns:
  - [ ] `ws['A1']` notation
  - [ ] `ws.cell(row=1, column=1)` method
  - [ ] `ws.iter_rows()` iterator
  - [ ] `ws.iter_cols()` iterator
- [ ] Implement `Cell` class with value/style properties

### Compatibility Shims
- [ ] Map OpenPyXL method names to XlsXpress equivalents
- [ ] Support both `save()` and context managers
- [ ] Handle OpenPyXL-style coordinate helpers
- [ ] Implement `utils` module (column letters, ranges)

### Deprecation Warnings
- [ ] Add warnings for unsupported OpenPyXL features
- [ ] Document migration path for each warning
- [ ] Provide feature detection (`xlsxpress.compat.supports()`)

---

## Phase 5: Advanced Features

### 5.1 Cell Styling
- [ ] Implement `Font` class (name, size, bold, italic, color)
- [ ] Implement `Fill` class (solid, pattern, gradient)
- [ ] Implement `Border` class (all edges, diagonal)
- [ ] Implement `Alignment` class (horizontal, vertical, wrap)
- [ ] Implement `NumberFormat` for custom formats
- [ ] Create `Style` composite class
- [ ] Support named styles
- [ ] Implement style caching for performance

### 5.2 Conditional Formatting
- [ ] Color scale rules (2-color, 3-color)
- [ ] Data bar rules
- [ ] Icon set rules
- [ ] Cell value comparison rules
- [ ] Formula-based rules
- [ ] Top/bottom N rules
- [ ] Duplicate/unique rules

### 5.3 Charts
- [ ] Create base `Chart` class
- [ ] Implement chart types:
  - [ ] AreaChart
  - [ ] BarChart
  - [ ] ColumnChart
  - [ ] LineChart
  - [ ] PieChart
  - [ ] ScatterChart
  - [ ] DoughnutChart
- [ ] Chart customization:
  - [ ] Titles and legends
  - [ ] Axis configuration
  - [ ] Data labels
  - [ ] Chart positioning

### 5.4 Data Validation
- [ ] List validation (dropdowns)
- [ ] Whole number validation
- [ ] Decimal validation
- [ ] Date validation
- [ ] Text length validation
- [ ] Custom formula validation
- [ ] Input messages and error alerts

### 5.5 Formulas
- [ ] Store formulas without evaluation
- [ ] Formula translation (cell reference adjustment)
- [ ] Array formula support
- [ ] Named range references in formulas
- [ ] Dynamic array formula support

### 5.6 Additional Features
- [ ] Images/pictures insertion
- [ ] Comments/notes
- [ ] Page setup and print settings
- [ ] Headers and footers
- [ ] Sheet protection
- [ ] Named ranges (create/read)
- [ ] Excel tables with structured references

---

## Phase 6: Testing & Benchmarks

**NOTE**: This phase runs continuously throughout development via TDD, not just at the end.

### Code Quality Checks (Continuous)
- [ ] Run `cargo clippy -- -D warnings` before every commit
- [ ] Run `cargo fmt --check` to verify formatting
- [ ] Check cognitive complexity with SonarQube rules (≤15)
- [ ] Verify test coverage ≥90% with `cargo tarpaulin`
- [ ] Run `black --check` and `ruff check` for Python code
- [ ] Run `mypy --strict` for Python type checking
- [ ] Security scans: `cargo audit` and `bandit`

### Unit Tests (Rust)
- [ ] Reader tests for each format
- [ ] Writer tests for all cell types
- [ ] Style application tests
- [ ] Chart generation tests
- [ ] Error handling tests
- [ ] Edge case tests (empty files, max dimensions)

### Integration Tests (Python)
- [ ] End-to-end read/write cycles
- [ ] Pandas integration tests
- [ ] NumPy integration tests
- [ ] OpenPyXL compatibility test suite
- [ ] Large file handling tests
- [ ] Concurrent access tests

### Performance Benchmarks
- [ ] Create benchmark suite with criterion (Rust)
- [ ] Create Python benchmark scripts
- [ ] Benchmark vs OpenPyXL:
  - [ ] Small file (1MB) read/write
  - [ ] Medium file (10MB) read/write
  - [ ] Large file (100MB) read/write
  - [ ] Streaming mode memory usage
- [ ] Benchmark vs XlsxWriter (write performance)
- [ ] Document performance results

### Compatibility Testing
- [ ] Test files from different Excel versions
- [ ] Test files from LibreOffice/Calc
- [ ] Test files from Google Sheets export
- [ ] Validate output in Excel, LibreOffice, Google Sheets

---

## Phase 7: Documentation & Release

### API Documentation
- [ ] Docstrings for all public Python classes/methods
- [ ] Type stubs (.pyi files) for IDE support
- [ ] Generate API reference with Sphinx
- [ ] Create user guide with tutorials:
  - [ ] Getting started
  - [ ] Reading Excel files
  - [ ] Writing Excel files
  - [ ] Working with styles
  - [ ] Creating charts
  - [ ] Migrating from OpenPyXL
  - [ ] Performance optimization tips

### Examples
- [ ] Basic read/write examples
- [ ] Pandas DataFrame integration
- [ ] Styled report generation
- [ ] Chart creation examples
- [ ] Large file processing
- [ ] Migration examples from OpenPyXL

### Release Preparation
- [ ] Create `CHANGELOG.md`
- [ ] Write release notes
- [ ] Set up semantic versioning
- [ ] Configure PyPI publishing
- [ ] Build wheels for all platforms:
  - [ ] Linux x86_64 (manylinux)
  - [ ] Linux ARM64
  - [ ] macOS x86_64
  - [ ] macOS ARM64 (Apple Silicon)
  - [ ] Windows x86_64
- [ ] Test installation from PyPI
- [ ] Create GitHub release with binaries

### Community
- [ ] Write `CONTRIBUTING.md`
- [ ] Set up issue templates
- [ ] Create discussion forum or Discord
- [ ] Write security policy (`SECURITY.md`)
- [ ] Add code of conduct

---

## Milestones

### v0.1.0 - Alpha (Core Reading)
- Basic reading for .xlsx files
- Python bindings with Workbook/Worksheet classes
- Pandas DataFrame integration
- Basic documentation

### v0.2.0 - Alpha (Core Writing)
- Basic writing for .xlsx files
- Cell value types and simple formatting
- Streaming write mode

### v0.3.0 - Beta (Compatibility)
- OpenPyXL compatibility layer
- Common migration scenarios working
- Comprehensive test suite

### v0.4.0 - Beta (Advanced Features)
- Full cell styling
- Conditional formatting
- Data validation
- Basic charts

### v0.5.0 - Beta (Charts & Polish)
- Complete chart support
- Performance optimization
- Documentation complete

### v1.0.0 - Stable Release
- Production-ready
- Full platform support
- API stability guarantee
- Community feedback incorporated

---

## Notes

### Performance Optimization Priorities
1. Minimize Python/Rust FFI crossings
2. Use batch operations over cell-by-cell access
3. Implement lazy loading wherever possible
4. Consider SIMD for numeric operations
5. Profile and optimize hot paths

### Known Challenges
- Formula evaluation not planned (too complex)
- VBA handling limited to preservation
- Some OpenPyXL features may be impossible to replicate
- Cross-platform testing requires CI infrastructure

### Resources
- [calamine docs](https://docs.rs/calamine)
- [rust_xlsxwriter docs](https://rustxlsxwriter.github.io/)
- [PyO3 user guide](https://pyo3.rs/)
- [maturin docs](https://www.maturin.rs/)
- [OpenPyXL docs](https://openpyxl.readthedocs.io/)

---

*Last Updated: December 2024*
