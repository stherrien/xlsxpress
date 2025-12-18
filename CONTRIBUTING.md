# Contributing to XlsXpress

Thank you for your interest in contributing to XlsXpress! This document outlines our development practices and coding standards.

## Table of Contents
- [Development Philosophy](#development-philosophy)
- [Setting Up Development Environment](#setting-up-development-environment)
- [Test-Driven Development (TDD)](#test-driven-development-tdd)
- [Clean Code Principles](#clean-code-principles)
- [DRY Principle](#dry-principle)
- [SOLID Principles](#solid-principles)
- [Cognitive Complexity](#cognitive-complexity)
- [Code Quality Tools](#code-quality-tools)
- [Contribution Workflow](#contribution-workflow)
- [Code Review Process](#code-review-process)

---

## Development Philosophy

XlsXpress is built on three core principles:

1. **Test-Driven Development (TDD)**: Tests are written before implementation
2. **Clean Code**: Code should be readable, maintainable, and self-documenting
3. **Performance**: Rust's zero-cost abstractions without sacrificing code quality

**All contributions must follow these principles.**

---

## Setting Up Development Environment

### Prerequisites
- Rust 1.70+ (stable toolchain)
- Python 3.10+
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/xlsxpress.git
cd xlsxpress
```

2. Install Rust dependencies:
```bash
rustup update stable
cargo install cargo-tarpaulin cargo-audit cargo-deny
```

3. Install Python dependencies:
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -e ".[dev]"
pip install pytest pytest-cov black ruff mypy bandit
```

4. Install pre-commit hooks:
```bash
pip install pre-commit
pre-commit install
```

### Verify Setup
```bash
# Rust tests
cargo test

# Python tests
pytest

# Code quality
cargo clippy -- -D warnings
cargo fmt --check
black --check python/
ruff check python/
mypy python/ --strict
```

---

## Test-Driven Development (TDD)

### The Red-Green-Refactor Cycle

**CRITICAL**: Every feature must follow this cycle:

1. **🔴 RED**: Write a failing test
   - Think about the API you want
   - Write a test that describes the desired behavior
   - Run the test and watch it fail
   - Ensure it fails for the right reason

2. **🟢 GREEN**: Make the test pass
   - Write the minimal code to pass the test
   - Don't worry about perfection yet
   - Get to green as quickly as possible

3. **🔵 REFACTOR**: Improve the code
   - Apply clean code principles
   - Remove duplication (DRY)
   - Check cognitive complexity (≤15)
   - Ensure SOLID principles
   - Keep all tests green

4. **🔁 REPEAT**: Move to the next feature

### Example TDD Workflow

```rust
// 1. RED: Write a failing test
#[test]
fn test_read_string_cell() {
    let reader = Reader::open("test.xlsx").unwrap();
    let sheet = reader.sheet("Sheet1").unwrap();

    // This will fail because Reader doesn't exist yet
    assert_eq!(sheet.cell("A1").as_string(), Some("Hello"));
}

// 2. GREEN: Implement minimal code
pub struct Reader { /* ... */ }
impl Reader {
    pub fn open(path: &str) -> Result<Self> {
        // Minimal implementation
    }
    pub fn sheet(&self, name: &str) -> Result<Sheet> {
        // Minimal implementation
    }
}

// 3. REFACTOR: Improve code quality
// - Extract common logic
// - Add error handling
// - Improve names
// - Check complexity

// 4. REPEAT: Next test
#[test]
fn test_read_number_cell() { /* ... */ }
```

### TDD Best Practices

**DO:**
- ✅ Write tests first, always
- ✅ Test one thing at a time
- ✅ Use descriptive test names: `test_read_string_cell_from_xlsx_file()`
- ✅ Test edge cases and error conditions
- ✅ Keep tests independent (no shared state)
- ✅ Use property-based testing for algorithms (proptest)

**DON'T:**
- ❌ Write production code without a failing test
- ❌ Write tests after implementation ("test-after" is not TDD)
- ❌ Skip the refactor step
- ❌ Write tests that depend on each other
- ❌ Test implementation details (test behavior)

---

## Clean Code Principles

### Meaningful Names

**Functions/Methods**: Use verb phrases
```rust
// Good
fn read_workbook(path: &Path) -> Result<Workbook>
fn parse_cell_value(raw: &str) -> CellValue
fn is_empty(&self) -> bool

// Bad
fn data(p: &Path) -> Result<Workbook>  // Unclear verb
fn process(s: &str) -> CellValue       // Too generic
fn empty(&self) -> bool                // Not a predicate
```

**Types/Structs**: Use nouns
```rust
// Good
struct Workbook { /* ... */ }
struct CellStyle { /* ... */ }
enum CellValue { /* ... */ }

// Bad
struct ReadData { /* ... */ }   // Verb in type name
struct StyleApplier { /* ... */ } // Action, not a thing
```

**Variables**: Descriptive and contextual
```rust
// Good
let sheet_count = workbook.sheets().len();
let cell_value = sheet.cell("A1").value();

// Bad
let n = workbook.sheets().len();  // Meaningless
let x = sheet.cell("A1").value(); // Too generic
```

### Function Design

**Single Responsibility**: One function, one job
```rust
// Good: Single responsibility
fn parse_cell_value(raw: &str) -> Result<CellValue> {
    // Only parses, doesn't validate or transform
}

fn validate_cell_value(value: &CellValue) -> Result<()> {
    // Only validates
}

// Bad: Multiple responsibilities
fn parse_and_validate_and_transform(raw: &str) -> Result<String> {
    // Too much happening here
}
```

**Small Functions**: Target 10-20 lines
```rust
// Good: Small, focused function
fn read_cell_string(cell: &Cell) -> Option<String> {
    match cell.value() {
        CellValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

// Bad: Too large (>50 lines)
fn process_cell(cell: &Cell) -> ProcessedCell {
    // 50+ lines of logic
    // Hard to test
    // Hard to understand
}
```

**Few Parameters**: Maximum 3-4 parameters
```rust
// Good: Few parameters
fn create_style(font: Font, fill: Fill) -> Style {
    Style { font, fill, ..Default::default() }
}

// Better: Use a builder or config struct
fn create_style(config: StyleConfig) -> Style {
    Style::from_config(config)
}

// Bad: Too many parameters
fn create_style(font: Font, size: u32, bold: bool, italic: bool,
                color: Color, bg: Color, border: Border) -> Style {
    // Too many parameters to track
}
```

### Comments

**Code Should Be Self-Documenting**
```rust
// Bad: Comment states the obvious
// Increment counter
counter += 1;

// Good: Code is clear without comment
processed_cells_count += 1;
```

**Comments Explain WHY, Not WHAT**
```rust
// Good: Explains reasoning
// Using binary search because sheet dimensions can exceed 1M rows
let cell_index = binary_search_rows(sheet, target_row);

// Bad: Explains what code does (redundant)
// This function searches for a cell
fn search_cell() { /* ... */ }
```

**Document Complex Algorithms**
```rust
/// Calculates cognitive complexity using SonarQube rules.
///
/// Complexity score increments for:
/// - Control flow breaks (+1)
/// - Nested structures (+1 per level)
/// - Logical operators (+1, except simple guards)
///
/// Target: ≤15 (good), 16-25 (review), >25 (refactor required)
fn calculate_complexity(ast: &Ast) -> u32 {
    // Implementation
}
```

---

## DRY Principle

**Don't Repeat Yourself**: Every piece of knowledge should have a single representation.

### Detecting Duplication

```rust
// Bad: Repeated code
fn read_string_range(sheet: &Sheet, range: Range) -> Vec<String> {
    let mut result = Vec::new();
    for cell in sheet.cells_in_range(range) {
        if let CellValue::String(s) = cell.value() {
            result.push(s.clone());
        }
    }
    result
}

fn read_number_range(sheet: &Sheet, range: Range) -> Vec<f64> {
    let mut result = Vec::new();
    for cell in sheet.cells_in_range(range) {
        if let CellValue::Number(n) = cell.value() {
            result.push(*n);
        }
    }
    result
}

// Good: Generic function (DRY)
fn read_range<T: FromCell>(sheet: &Sheet, range: Range) -> Vec<T> {
    sheet.cells_in_range(range)
        .filter_map(|cell| T::from_cell(cell))
        .collect()
}

trait FromCell {
    fn from_cell(cell: &Cell) -> Option<Self>
    where Self: Sized;
}
```

### Configuration Over Duplication

```rust
// Bad: Separate functions for each chart type
fn create_bar_chart(...) { /* boilerplate */ }
fn create_line_chart(...) { /* similar boilerplate */ }
fn create_pie_chart(...) { /* similar boilerplate */ }

// Good: Data-driven approach
enum ChartType { Bar, Line, Pie }

fn create_chart(chart_type: ChartType, config: ChartConfig) -> Chart {
    Chart::new(chart_type, config)
}
```

---

## SOLID Principles

### S - Single Responsibility Principle

Each module/class/function has ONE reason to change.

```rust
// Bad: Multiple responsibilities
struct Workbook {
    fn read_file(&mut self) { /* ... */ }
    fn parse_xml(&mut self) { /* ... */ }
    fn validate_schema(&mut self) { /* ... */ }
    fn render_ui(&self) { /* ... */ }
}

// Good: Separated responsibilities
struct Reader {
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;
}

struct Parser {
    fn parse_xml(&self, data: &[u8]) -> Result<Workbook>;
}

struct Validator {
    fn validate_schema(&self, workbook: &Workbook) -> Result<()>;
}

// UI rendering in separate crate
```

### O - Open/Closed Principle

Open for extension, closed for modification.

```rust
// Good: Extensible via trait
trait CellFormatter {
    fn format(&self, value: &CellValue) -> String;
}

struct DefaultFormatter;
impl CellFormatter for DefaultFormatter { /* ... */ }

struct CustomFormatter;
impl CellFormatter for CustomFormatter { /* ... */ }

// New formatters can be added without modifying existing code
```

### L - Liskov Substitution Principle

Subtypes must be substitutable for base types.

```rust
trait Workbook {
    fn sheet(&self, name: &str) -> Option<&Sheet>;
}

// Good: XlsxWorkbook honors the contract
struct XlsxWorkbook;
impl Workbook for XlsxWorkbook {
    fn sheet(&self, name: &str) -> Option<&Sheet> {
        // Returns None if sheet doesn't exist (as expected)
    }
}

// Bad: Violates LSP by panicking
struct BadWorkbook;
impl Workbook for BadWorkbook {
    fn sheet(&self, name: &str) -> Option<&Sheet> {
        // Panics instead of returning None - breaks contract!
        panic!("Sheet not found");
    }
}
```

### I - Interface Segregation Principle

Many specific interfaces > one general interface.

```rust
// Good: Segregated traits
trait Readable {
    fn read(&self) -> Result<Vec<u8>>;
}

trait Writable {
    fn write(&mut self, data: &[u8]) -> Result<()>;
}

trait Stylable {
    fn apply_style(&mut self, style: &Style);
}

// Bad: Fat interface
trait Workbook {
    fn read(&self) -> Result<Vec<u8>>;
    fn write(&mut self, data: &[u8]) -> Result<()>;
    fn apply_style(&mut self, style: &Style);
    fn create_chart(&mut self, chart: Chart);
    fn add_validation(&mut self, rule: ValidationRule);
    // ... 20 more methods
}
```

### D - Dependency Inversion Principle

Depend on abstractions, not concretions.

```rust
// Good: Depends on trait (abstraction)
fn process_workbook<R: Read>(reader: R) -> Result<Workbook> {
    // Works with any reader implementation
}

// Bad: Depends on concrete type
fn process_workbook(reader: FileReader) -> Result<Workbook> {
    // Only works with FileReader
}
```

---

## Cognitive Complexity

Following SonarQube's cognitive complexity rules to maintain readable code.

### Complexity Score
- **Target**: ≤15 (good)
- **Warning**: 16-25 (needs review)
- **Critical**: >25 (must refactor)

### What Increases Complexity (+1 each)
- Control flow: `if`, `else`, `elif`, `match`, `loop`, `while`, `for`
- Logical operators: `&&`, `||` (except simple guards)
- Recursion: Recursive calls
- Nesting: +1 for each nesting level
- Jump statements: `break`, `continue`, `return` (mid-function)

### Refactoring Strategies

**Strategy 1: Extract Functions**
```rust
// Bad: High complexity (score: 12)
fn process(data: &Data) -> Result<Output> {
    if data.is_valid() {
        if data.has_header() {
            if let Some(header) = data.parse_header() {
                // ... more nesting
            }
        }
    }
    // ...
}

// Good: Lower complexity (score: 4)
fn process(data: &Data) -> Result<Output> {
    validate_data(data)?;
    let header = extract_header(data)?;
    parse_body(data, &header)
}
```

**Strategy 2: Early Returns**
```rust
// Bad: Deep nesting (score: 8)
fn check_cell(cell: &Cell) -> Result<Value> {
    if !cell.is_empty() {
        if cell.has_value() {
            if cell.value_valid() {
                return Ok(cell.value());
            } else {
                return Err(InvalidValue);
            }
        } else {
            return Err(NoValue);
        }
    } else {
        return Err(EmptyCell);
    }
}

// Good: Early returns (score: 3)
fn check_cell(cell: &Cell) -> Result<Value> {
    if cell.is_empty() {
        return Err(EmptyCell);
    }
    if !cell.has_value() {
        return Err(NoValue);
    }
    if !cell.value_valid() {
        return Err(InvalidValue);
    }
    Ok(cell.value())
}
```

**Strategy 3: Use Match Expressions**
```rust
// Bad: Multiple if-else (score: 6)
fn parse_value(cell: &Cell) -> Result<Value> {
    if cell.is_string() {
        parse_string(cell)
    } else if cell.is_number() {
        parse_number(cell)
    } else if cell.is_bool() {
        parse_bool(cell)
    } else {
        Err(UnknownType)
    }
}

// Good: Match expression (score: 1)
fn parse_value(cell: &Cell) -> Result<Value> {
    match cell.cell_type() {
        CellType::String => parse_string(cell),
        CellType::Number => parse_number(cell),
        CellType::Bool => parse_bool(cell),
        _ => Err(UnknownType),
    }
}
```

---

## Code Quality Tools

### Rust Tools

**rustfmt**: Code formatting
```bash
cargo fmt
cargo fmt --check  # CI mode
```

**clippy**: Linting
```bash
cargo clippy -- -D warnings  # Deny all warnings
```

**cargo-tarpaulin**: Code coverage
```bash
cargo tarpaulin --out Html --output-dir coverage
# Target: ≥90% coverage
```

**cargo-audit**: Security vulnerabilities
```bash
cargo audit
```

### Python Tools

**black**: Code formatting
```bash
black python/
black --check python/  # CI mode
```

**ruff**: Fast linting
```bash
ruff check python/
ruff check --fix python/  # Auto-fix
```

**mypy**: Static type checking
```bash
mypy python/ --strict
```

**pytest**: Testing
```bash
pytest --cov=xlsxpress --cov-report=html
```

**bandit**: Security scanning
```bash
bandit -r python/
```

---

## Contribution Workflow

### 1. Create a Branch

Follow Git Flow naming:
```bash
# Feature branch
git checkout -b <JIRA-TICKET>-short-description

# Example
git checkout -b REG-1234-add-chart-support
```

**Never commit directly to `main` or `dev` branches.**

### 2. Write Tests First (TDD)

```bash
# Create test file
touch tests/rust/test_feature.rs

# Write failing test
# Run tests to verify it fails
cargo test

# Implement feature
# Run tests until they pass
cargo test

# Refactor
cargo clippy
```

### 3. Commit Changes

Write clear commit messages:
```bash
git commit -m "REG-1234: Add bar chart support

- Implement BarChart struct with configuration
- Add tests for chart creation and rendering
- Update documentation with examples
- Ensure cognitive complexity ≤15"
```

### 4. Run Quality Checks

Before pushing:
```bash
# Rust
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo tarpaulin

# Python
pytest
black --check python/
ruff check python/
mypy python/ --strict
```

### 5. Push and Create PR

```bash
git push -u origin REG-1234-add-chart-support
```

Create PR with:
- Clear description
- Link to Jira ticket
- Screenshots (if UI changes)
- Performance impact (if applicable)

---

## Code Review Process

### Review Checklist

Reviewers must verify:

- [ ] **TDD**: Tests written before implementation?
- [ ] **Test Coverage**: ≥90% coverage, all tests pass?
- [ ] **Clean Code**: Functions ≤20 lines, meaningful names?
- [ ] **DRY**: No code duplication?
- [ ] **SOLID**: Follows all five principles?
- [ ] **Cognitive Complexity**: All functions ≤15?
- [ ] **Error Handling**: Proper Result/Error types?
- [ ] **Documentation**: Public APIs documented?
- [ ] **Performance**: No regressions?
- [ ] **Security**: No vulnerabilities introduced?

### Review Comments

Be constructive:
```
# Good review comment
"This function has cognitive complexity of 18. Consider extracting
the validation logic into a separate function to reduce complexity."

# Bad review comment
"This code is too complex."
```

### Approval

- Minimum 1 approving review required
- All checks must pass (CI/CD)
- No unresolved comments

---

## Questions?

- Check [REQUIREMENTS.md](REQUIREMENTS.md) for design principles
- Check [TODO.md](TODO.md) for development tasks
- Open a discussion on GitHub
- Contact maintainers

---

**Thank you for contributing to XlsXpress!** 🎉

*Last Updated: December 2024*
