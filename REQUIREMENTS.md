# XlsXpress Requirements Document

## 1. Project Overview

**XlsXpress** is a high-performance Excel processing library built with Rust and exposed to Python via PyO3 bindings. It aims to provide lightning-fast data engineering pipelines for legacy Excel reports by combining the best Rust Excel libraries with an ergonomic Python API.

### Vision
Replace OpenPyXL and XlsxWriter as the go-to Python Excel library by offering:
- **10x+ performance improvement** over pure Python implementations
- **Memory efficiency** that handles 100MB+ files without exhausting RAM
- **Full feature parity** with OpenPyXL for common use cases
- **Drop-in compatibility** for easier migration from existing codebases

### Why Rust?
- Parsing XML-based formats (.xlsx) is a task Rust excels at
- Zero-cost abstractions and memory safety without garbage collection
- Excellent existing ecosystem (calamine, rust_xlsxwriter)
- Proven performance: calamine processes 1.122M cells/second

---

## 2. Performance Targets

### Reading Performance
| Metric | Target | Baseline (OpenPyXL) |
|--------|--------|---------------------|
| Throughput | >1M cells/second | ~119K cells/second |
| Memory (50MB file) | <100MB RAM | ~2.5GB RAM |
| Load time (50MB file) | <5 seconds | ~45 seconds |

### Writing Performance
| Metric | Target | Baseline (XlsxWriter) |
|--------|--------|----------------------|
| Throughput | >500K cells/second | ~130K cells/second |
| Memory | Constant (<50MB) | Variable |
| Large file support | Unlimited rows | Limited by RAM |

### General Targets
- **Startup time**: <100ms to import library
- **FFI overhead**: <1% of total processing time for batch operations
- **Parallel processing**: Near-linear scaling up to 8 cores for independent operations

---

## 3. Supported File Formats

### Full Read & Write Support
| Format | Extension | Description |
|--------|-----------|-------------|
| Excel 2007+ | .xlsx | Primary target format |
| Excel Macro-Enabled | .xlsm | Preserve VBA macros (cannot create/edit VBA) |

### Read-Only Support
| Format | Extension | Description |
|--------|-----------|-------------|
| Excel Binary | .xlsb | Fast binary format |
| Excel 97-2003 | .xls | Legacy format |
| OpenDocument | .ods | LibreOffice/OpenOffice format |
| Excel Add-in | .xlam, .xla | Add-in formats |

---

## 4. Core Features

### 4.1 Reading Capabilities
- **Cell values**: Strings, numbers, booleans, dates, errors
- **Multiple sheets**: Access by name or index
- **Merged cells**: Proper handling of merged regions
- **Named ranges**: Read defined names
- **Metadata**: Document properties, sheet dimensions
- **Streaming mode**: Memory-efficient row-by-row iteration
- **Parallel reading**: Concurrent sheet processing

### 4.2 Writing Capabilities
- **Cell values**: All standard types with automatic type inference
- **Multiple sheets**: Create, rename, delete worksheets
- **Merged cells**: Create merged regions
- **Named ranges**: Define and reference names
- **Metadata**: Set document properties
- **Streaming mode**: Constant-memory writes for unlimited rows
- **Workbook templates**: Start from existing files

### 4.3 Modification Capabilities
- **In-place editing**: Modify existing workbooks
- **Sheet operations**: Copy, move, hide/unhide sheets
- **Range operations**: Insert/delete rows and columns
- **VBA preservation**: Maintain macros in .xlsm files (read-only)

---

## 5. Advanced Features

### 5.1 Charts & Graphs
**Priority: High**

Supported chart types:
- Area charts (2D/3D, stacked)
- Bar charts (clustered, stacked)
- Column charts (clustered, stacked)
- Line charts (with markers)
- Pie charts (exploded, doughnut)
- Scatter plots (XY)
- Combination charts

Chart features:
- Custom titles and legends
- Axis configuration (labels, scaling, logarithmic)
- Data series from cell ranges
- Chart positioning and sizing

### 5.2 Cell Styling
**Priority: High**

Font properties:
- Name, size, color
- Bold, italic, underline, strikethrough
- Subscript/superscript

Cell formatting:
- Background colors and patterns
- Border styles (all edges, diagonal)
- Alignment (horizontal, vertical, rotation)
- Text wrapping and shrink-to-fit
- Number formats (built-in and custom)

Conditional formatting:
- Color scales (2-color, 3-color)
- Data bars
- Icon sets
- Cell value rules
- Formula-based rules

### 5.3 Formulas
**Priority: High**

Formula support:
- Store and preserve formulas
- Formula translation (adjust references when copying)
- Array formulas
- Dynamic array formulas (Excel 365)
- Named formula references

**Note**: Formula evaluation/calculation is OUT OF SCOPE. Formulas are stored but not computed. Users needing calculated values should open files in Excel or use a separate calculation engine.

### 5.4 Data Validation
**Priority: High**

Validation types:
- Dropdown lists (from ranges or explicit values)
- Whole number ranges
- Decimal ranges
- Date ranges
- Text length constraints
- Custom formula validation

Validation options:
- Input messages (title + body)
- Error alerts (stop, warning, information)
- Allow blank cells option

---

## 6. API Design

### 6.1 Dual API Architecture

XlsXpress provides two API layers:

#### Modern API (Primary)
Optimized for Rust performance characteristics:
```python
import xlsxpress as xp

# Reading
with xp.open("data.xlsx") as wb:
    df = wb.sheet("Sales").to_dataframe()

# Writing
with xp.create("output.xlsx") as wb:
    wb.add_sheet("Results").write_dataframe(df)
```

#### Compatibility API
Drop-in replacement for OpenPyXL:
```python
from xlsxpress.compat import openpyxl

wb = openpyxl.load_workbook("data.xlsx")
ws = wb.active
ws["A1"] = "Hello"
wb.save("output.xlsx")
```

### 6.2 Design Principles

1. **Batch over iteration**: Encourage operations on ranges, not individual cells
2. **Context managers**: Ensure proper resource cleanup
3. **Type hints**: Full type annotation for IDE support
4. **Pandas integration**: Native DataFrame read/write
5. **NumPy support**: Zero-copy array operations where possible
6. **Lazy loading**: Don't load data until accessed
7. **Immutable styles**: Style objects are immutable (create new, don't modify)

### 6.3 Error Handling

- Custom exception hierarchy rooted at `XlsXpressError`
- Clear error messages with file/sheet/cell context
- Graceful handling of corrupted files
- Warnings for deprecated features or format limitations

---

## 7. Development Principles

XlsXpress adheres to industry-standard software engineering practices to ensure code quality, maintainability, and reliability.

### 7.1 Test-Driven Development (TDD)

**Red-Green-Refactor Cycle:**
All features must be developed following TDD:
1. **Red**: Write a failing test that defines desired functionality
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve code quality while keeping tests green

**Test Coverage Requirements:**
- Minimum 90% code coverage for production code
- 100% coverage for critical paths (data integrity, security)
- Both Rust unit tests and Python integration tests required
- Property-based testing for complex algorithms (using proptest in Rust)

**Test Categories:**
```rust
// Rust: Unit tests within modules
#[cfg(test)]
mod tests {
    #[test]
    fn test_read_cell_string() { ... }
}
```

```python
# Python: Integration and compatibility tests
def test_openpyxl_compat_cell_access():
    wb = openpyxl.load_workbook("test.xlsx")
    assert wb["Sheet1"]["A1"].value == "Expected"
```

### 7.2 Clean Code Principles

**Meaningful Names:**
- Use descriptive, intention-revealing names
- Avoid abbreviations unless universally understood (e.g., `df` for DataFrame)
- Functions/methods: verb phrases (`read_workbook`, `write_cell`)
- Classes/types: nouns (`Workbook`, `CellStyle`)
- Booleans: predicates (`is_empty`, `has_formula`)

**Function Design:**
- Single Responsibility: Each function does ONE thing
- Small functions: Target 10-20 lines maximum
- Few parameters: Maximum 3-4 parameters; use structs for more
- No side effects: Pure functions preferred; clearly document mutations
- Error handling: Use Result types in Rust, exceptions in Python

**Code Organization:**
```rust
// Good: Clear, single-purpose function
fn parse_cell_value(raw: &str) -> Result<CellValue, ParseError> {
    // Implementation
}

// Bad: Multiple responsibilities
fn parse_and_format_and_validate(data: &str, format: &str, rules: &Rules) -> Result<String, Error> {
    // Too much happening here
}
```

**Comments:**
- Code should be self-documenting through clear names
- Comments explain WHY, not WHAT
- Document complex algorithms, business logic, and non-obvious decisions
- Use doc comments for public APIs (Rust: `///`, Python: docstrings)

### 7.3 DRY Principle (Don't Repeat Yourself)

**No Code Duplication:**
- Extract repeated logic into functions/methods
- Use traits/protocols for shared behavior
- Create utility modules for common operations
- Prefer composition over inheritance

**Configuration Over Duplication:**
- Use data-driven approaches for similar operations
- Create generic/template functions where appropriate
- Leverage Rust's type system and Python's dynamic features

**Example:**
```rust
// Good: Generic function
fn read_range<T: FromCell>(sheet: &Sheet, range: Range) -> Result<Vec<T>> {
    // Single implementation for all types
}

// Bad: Repeated code
fn read_string_range(...) -> Vec<String> { /* code */ }
fn read_number_range(...) -> Vec<f64> { /* similar code */ }
```

### 7.4 SOLID Principles

**S - Single Responsibility Principle:**
- Each module, class, and function has ONE reason to change
- Separate concerns: reading, writing, formatting, validation
- Example: `Reader` handles reading, `Validator` handles validation

**O - Open/Closed Principle:**
- Open for extension, closed for modification
- Use traits for extensibility in Rust
- Use abstract base classes/protocols in Python compatibility layer

```rust
// Good: Extensible via trait
trait CellRenderer {
    fn render(&self, cell: &Cell) -> String;
}

// Custom renderers can be added without modifying core
```

**L - Liskov Substitution Principle:**
- Derived types must be substitutable for base types
- Maintain interface contracts
- Don't weaken preconditions or strengthen postconditions

**I - Interface Segregation Principle:**
- Many specific interfaces better than one general-purpose interface
- Example: `Readable`, `Writable`, `Stylable` traits vs one giant `Workbook` trait

**D - Dependency Inversion Principle:**
- Depend on abstractions, not concretions
- Use trait bounds in Rust
- Use dependency injection where appropriate

```rust
// Good: Depends on abstraction
fn process_data<R: Read>(reader: R) -> Result<Data> {
    // Works with any reader implementation
}
```

### 7.5 Code Quality Tools

**Rust:**
- `rustfmt`: Code formatting (enforced in CI)
- `clippy`: Linting with strict settings (deny warnings)
- `cargo-audit`: Security vulnerability scanning
- `cargo-deny`: License and dependency checking
- `cargo-tarpaulin`: Code coverage reporting

**Python:**
- `black`: Code formatting (line length: 88)
- `ruff`: Fast linting (replaces flake8, isort, pylint)
- `mypy`: Static type checking (strict mode)
- `pytest`: Testing framework with coverage plugin
- `bandit`: Security vulnerability scanning

**Pre-commit Hooks:**
All checks must pass before commit:
```yaml
# .pre-commit-config.yaml
- rustfmt --check
- cargo clippy -- -D warnings
- black --check
- ruff check
- mypy --strict
```

### 7.6 Cognitive Complexity Management

**SonarQube Standards:**
Following SonarQube's cognitive complexity calculation to maintain readable code.

**Cognitive Complexity Score:**
- Target: ≤15 per function (good)
- Warning: 16-25 (needs review)
- Critical: >25 (must refactor)

**Complexity Contributors (+1 each):**
- Control flow breaks: `if`, `else`, `elif`, `match`, `loop`, `while`, `for`
- Logical operators: `&&`, `||` (except simple guards)
- Recursion: Recursive function calls
- Nested structures: +1 for each nesting level
- Jump statements: `break`, `continue`, `return` (mid-function)

**Reduction Strategies:**
```rust
// Bad: High cognitive complexity (score: 12)
fn process_cell(cell: &Cell) -> Result<Value> {
    if cell.is_empty() {
        if cell.has_default() {
            return cell.default_value();
        } else {
            return Err(EmptyCell);
        }
    } else if cell.has_formula() {
        if cell.formula_cached() {
            return cell.cached_value();
        } else {
            return Err(NoCache);
        }
    } else {
        return cell.value();
    }
}

// Good: Lower complexity (score: 4) via early returns and extraction
fn process_cell(cell: &Cell) -> Result<Value> {
    if cell.is_empty() {
        return handle_empty_cell(cell);
    }
    if cell.has_formula() {
        return handle_formula_cell(cell);
    }
    cell.value()
}
```

### 7.7 Code Review Standards

All code changes require:
- [ ] Passing test suite (no red tests)
- [ ] New tests for new functionality (TDD)
- [ ] Code coverage ≥90%
- [ ] No clippy/ruff warnings
- [ ] Cognitive complexity ≤15 per function
- [ ] Updated documentation
- [ ] Reviewed by at least one maintainer

**Review Checklist:**
1. Does it follow TDD (tests written first)?
2. Is the code clean and readable?
3. Are there any DRY violations?
4. Does it adhere to SOLID principles?
5. Is cognitive complexity acceptable?
6. Are errors handled properly?
7. Is performance acceptable?
8. Are security implications considered?

---

## 8. Dependencies

### Rust Crates
| Crate | Purpose | Version |
|-------|---------|---------|
| calamine | Excel reading | ^0.24 |
| rust_xlsxwriter | Excel writing | ^0.64 |
| pyo3 | Python bindings | ^0.20 |
| serde | Serialization | ^1.0 |
| chrono | Date/time handling | ^0.4 |
| rayon | Parallel processing | ^1.8 |

### Python Requirements
| Package | Purpose | Version |
|---------|---------|---------|
| Python | Runtime | >=3.10 |
| numpy | Array operations | >=1.20 (optional) |
| pandas | DataFrame support | >=1.5 (optional) |

### Build Tools
| Tool | Purpose |
|------|---------|
| maturin | Build & publish wheels |
| cargo | Rust package manager |
| pytest | Python testing |
| criterion | Rust benchmarking |

---

## 9. Security Considerations

### XML Parsing
- **Billion Laughs Attack**: Protected via limited entity expansion
- **Quadratic Blowup**: Protected via size limits on parsed content
- **External Entity Injection (XXE)**: Disabled by default

### File Handling
- **Path traversal**: Validate all file paths
- **Zip bombs**: Limit decompression ratios
- **Memory limits**: Configurable caps on file sizes

### VBA/Macros
- **No execution**: Macros are never executed
- **Preservation only**: VBA code preserved but not parsed or validated
- **User warnings**: Alert when opening macro-enabled files

---

## 10. Non-Goals / Out of Scope

The following features are explicitly **NOT** planned:

### Formula Calculation Engine
- Formulas are stored, not evaluated
- Users should use Excel or dedicated calc engines for computed values

### VBA/Macro Creation
- Cannot create or modify VBA code
- Can only preserve existing macros in .xlsm files

### Password Protection
- Cannot create password-protected files
- Cannot open password-protected files

### Pivot Tables
- Cannot create pivot tables
- Reading pivot table data may have limitations

### Real-time Collaboration
- No support for co-authoring features
- No cloud/OneDrive integration

### Legacy .xls Writing
- Read support for .xls files
- Write support for .xlsx/.xlsm only

### Full OpenPyXL Parity
- Some obscure features may not be implemented
- Focus on common use cases (80/20 rule)

---

## 11. Target Users

### Primary Users
- **Data Engineers**: Processing large Excel exports from legacy systems
- **ETL Developers**: Excel as source/destination in data pipelines
- **Automation Developers**: Batch Excel report generation
- **Scientists/Analysts**: Loading Excel data into analysis tools

### Use Cases
1. Reading 100MB+ Excel files without memory issues
2. Generating thousands of formatted Excel reports
3. Migrating from OpenPyXL without major code changes
4. Building Excel processing into CI/CD pipelines
5. Pandas integration for data science workflows

---

## 12. Success Criteria

### Performance
- [ ] 5x faster than OpenPyXL for reading (measured on 50MB file)
- [ ] 3x faster than XlsxWriter for writing (measured on 1M row file)
- [ ] <100MB RAM for any file size in streaming mode

### Compatibility
- [ ] 90% of OpenPyXL test suite passes with compatibility API
- [ ] All Pandas read_excel/to_excel operations work
- [ ] Wheels available for Windows, macOS, Linux (x86_64, ARM64)

### Quality
- [ ] >90% test coverage
- [ ] Zero critical security vulnerabilities
- [ ] Comprehensive documentation with examples
- [ ] Published to PyPI with stable API

---

*Document Version: 1.0*
*Last Updated: December 2024*
