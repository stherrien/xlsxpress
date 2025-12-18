//! Excel file reading functionality
//!
//! This module provides fast Excel file reading using the calamine library.
//! Follows TDD and clean code principles with functions kept under 20 lines
//! and cognitive complexity under 15.

use crate::error::{Error, Result};
use calamine::{open_workbook_auto, Data, DataType, Range, Reader as CalamineReader, Sheets};
use std::path::Path;

/// Excel file reader
///
/// Provides high-performance reading of Excel files using calamine.
/// Supports .xlsx, .xlsm, .xls, .xlsb, and .ods formats.
///
/// # Examples
///
/// ```rust,no_run
/// use xlsxpress::Reader;
///
/// let reader = Reader::open("data.xlsx")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Reader {
    /// Internal calamine workbook
    /// Sheets enum supports all Excel formats
    workbook: Sheets<std::io::BufReader<std::fs::File>>,
}

impl Reader {
    /// Open an Excel file for reading
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Excel file
    ///
    /// # Errors
    ///
    /// Returns `Error::FileRead` if the file cannot be opened or read.
    /// Returns `Error::InvalidFormat` if the file is not a valid Excel file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Reader;
    ///
    /// let reader = Reader::open("data.xlsx")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        // GREEN phase: Minimal implementation to pass test
        let workbook = open_workbook_auto(path.as_ref())?;
        Ok(Self { workbook })
    }

    /// Get list of sheet names in the workbook
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Reader;
    ///
    /// let reader = Reader::open("data.xlsx")?;
    /// let names = reader.sheet_names();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn sheet_names(&self) -> Vec<String> {
        // BLUE phase: Refactored per clippy suggestion
        self.workbook.sheet_names().clone()
    }

    /// Get a worksheet range by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the worksheet
    ///
    /// # Errors
    ///
    /// Returns `Error::SheetNotFound` if the sheet doesn't exist.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Reader;
    ///
    /// let mut reader = Reader::open("data.xlsx")?;
    /// let range = reader.worksheet_range("Sheet1")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn worksheet_range(&mut self, name: &str) -> Result<Range<Data>> {
        // GREEN phase: Minimal implementation
        self.workbook
            .worksheet_range(name)
            .map_err(|_| Error::sheet_not_found(name))
    }

    /// Get cell value as string
    ///
    /// # Arguments
    ///
    /// * `range` - The worksheet range
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    ///
    /// Returns `None` if cell is empty or out of bounds.
    #[must_use]
    pub fn get_cell_value(&self, range: &Range<Data>, row: usize, col: usize) -> Option<String> {
        // GREEN phase: Minimal implementation
        range.get((row, col)).and_then(|cell| {
            if cell.is_empty() {
                None
            } else {
                Some(cell.to_string())
            }
        })
    }

    /// Get cell value as number
    ///
    /// # Arguments
    ///
    /// * `range` - The worksheet range
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    ///
    /// Returns `None` if cell is not a number or empty.
    #[must_use]
    pub fn get_cell_number(&self, range: &Range<Data>, row: usize, col: usize) -> Option<f64> {
        // BLUE phase: Refactored per clippy suggestion
        range.get((row, col)).and_then(DataType::get_float)
    }

    /// Get dimensions of a range (rows, columns)
    ///
    /// # Arguments
    ///
    /// * `range` - The worksheet range
    ///
    /// Returns tuple of (`row_count`, `column_count`).
    #[must_use]
    pub fn get_dimensions(&self, range: &Range<Data>) -> (usize, usize) {
        // GREEN phase: Minimal implementation
        let (rows, cols) = range.get_size();
        (rows, cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test that we can open an Excel file
    ///
    /// This test will fail initially because Reader::open() returns an error.
    /// Following TDD, we write the test FIRST, watch it fail, then implement.
    #[test]
    fn test_open_xlsx_file() {
        // Arrange: Use test fixture
        let path = "tests/fixtures/test.xlsx";

        // Act: Try to open the file
        let result = Reader::open(path);

        // Assert: File should open successfully
        assert!(
            result.is_ok(),
            "Failed to open test.xlsx: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test that opening a non-existent file returns an error
    #[test]
    fn test_open_nonexistent_file() {
        // Arrange: Path to non-existent file
        let path = "tests/fixtures/nonexistent.xlsx";

        // Act: Try to open the file
        let result = Reader::open(path);

        // Assert: Should return an error
        assert!(result.is_err(), "Should fail to open non-existent file");
    }

    /// TDD RED: Test that opening an invalid file returns an error
    #[test]
    fn test_open_invalid_file() {
        // Arrange: Create a non-Excel file
        let path = "tests/fixtures/invalid.txt";
        std::fs::write(path, b"Not an Excel file").unwrap();

        // Act: Try to open the file
        let result = Reader::open(path);

        // Assert: Should return an error
        assert!(result.is_err(), "Should fail to open invalid Excel file");

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    /// TDD RED: Test that we can list sheet names
    #[test]
    fn test_list_sheet_names() {
        // Arrange: Open test file
        let reader = Reader::open("tests/fixtures/test.xlsx").unwrap();

        // Act: Get sheet names
        let sheet_names = reader.sheet_names();

        // Assert: Should have at least one sheet named "Sheet1"
        assert!(!sheet_names.is_empty(), "Should have at least one sheet");
        assert_eq!(sheet_names[0], "Sheet1", "First sheet should be Sheet1");
    }

    /// TDD RED: Test that we can access a sheet by name
    #[test]
    fn test_get_sheet_by_name() {
        // Arrange: Open test file
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();

        // Act: Get sheet by name
        let result = reader.worksheet_range("Sheet1");

        // Assert: Should successfully get the sheet
        assert!(result.is_ok(), "Should get Sheet1: {:?}", result.err());
    }

    /// TDD RED: Test that accessing non-existent sheet returns error
    #[test]
    fn test_get_nonexistent_sheet() {
        // Arrange: Open test file
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();

        // Act: Try to get non-existent sheet
        let result = reader.worksheet_range("NonExistent");

        // Assert: Should return error
        assert!(result.is_err(), "Should fail to get non-existent sheet");
    }

    /// TDD RED: Test reading a string cell value
    #[test]
    fn test_read_string_cell() {
        // Arrange: Open test file and get range
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();

        // Act: Read cell A1 (should be "Hello")
        let value = reader.get_cell_value(&range, 0, 0);

        // Assert: Should read "Hello"
        assert_eq!(value, Some("Hello".to_string()));
    }

    /// TDD RED: Test reading a number cell value
    #[test]
    fn test_read_number_cell() {
        // Arrange: Open test file and get range
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();

        // Act: Read cell B1 (should be 42)
        let value = reader.get_cell_number(&range, 0, 1);

        // Assert: Should read 42.0
        assert_eq!(value, Some(42.0));
    }

    /// TDD RED: Test reading a float cell value
    #[test]
    fn test_read_float_cell() {
        // Arrange: Open test file and get range
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();

        // Act: Read cell B2 (should be 3.14)
        let value = reader.get_cell_number(&range, 1, 1);

        // Assert: Should read 3.14
        assert!(value.is_some());
        let val = value.unwrap();
        assert!((val - 3.14).abs() < 0.001, "Expected 3.14, got {}", val);
    }

    /// TDD RED: Test reading an empty cell
    #[test]
    fn test_read_empty_cell() {
        // Arrange: Open test file and get range
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();

        // Act: Read cell C1 (should be empty)
        let value = reader.get_cell_value(&range, 0, 2);

        // Assert: Should be None
        assert_eq!(value, None);
    }

    /// TDD RED: Test getting cell dimensions
    #[test]
    fn test_get_dimensions() {
        // Arrange: Open test file and get range
        let mut reader = Reader::open("tests/fixtures/test.xlsx").unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();

        // Act: Get dimensions
        let (rows, cols) = reader.get_dimensions(&range);

        // Assert: Should have at least 2 rows and 2 columns
        assert!(rows >= 2, "Should have at least 2 rows, got {}", rows);
        assert!(cols >= 2, "Should have at least 2 columns, got {}", cols);
    }
}
