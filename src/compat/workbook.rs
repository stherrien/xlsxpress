//! Workbook wrapper for `OpenPyXL` compatibility
//!
//! Provides a Workbook type that mimics `OpenPyXL`'s Workbook interface for
//! opening and reading Excel files.

use crate::compat::worksheet::Worksheet;
use crate::error::{Error, Result};
use crate::Reader;
use std::path::Path;

/// Workbook wrapper compatible with `OpenPyXL`
///
/// Represents an Excel workbook with multiple worksheets.
/// Wraps the `XlsXpress` Reader for compatibility.
pub struct Workbook {
    /// Internal reader
    reader: Reader,
}

impl Workbook {
    /// Create a new workbook from a Reader
    ///
    /// # Arguments
    ///
    /// * `reader` - `XlsXpress` Reader instance
    #[must_use]
    pub fn new(reader: Reader) -> Self {
        Self { reader }
    }

    /// Get list of worksheet names
    ///
    /// Returns a vector of worksheet names in the workbook.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let wb = load_workbook("data.xlsx")?;
    /// let names = wb.sheetnames();
    /// ```
    #[must_use]
    pub fn sheetnames(&self) -> Vec<String> {
        self.reader.sheet_names()
    }

    /// Get a worksheet by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the worksheet
    ///
    /// # Errors
    ///
    /// Returns error if worksheet is not found.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let wb = load_workbook("data.xlsx")?;
    /// let ws = wb.get_sheet_by_name("Sheet1")?;
    /// ```
    pub fn get_sheet_by_name(&mut self, name: &str) -> Result<Worksheet> {
        let range = self.reader.worksheet_range(name)?;
        Ok(Worksheet::new(name, range))
    }

    /// Get the active (first) worksheet
    ///
    /// # Errors
    ///
    /// Returns error if workbook has no worksheets.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let wb = load_workbook("data.xlsx")?;
    /// let ws = wb.active()?;
    /// ```
    pub fn active(&mut self) -> Result<Worksheet> {
        let sheet_names = self.sheetnames();
        if sheet_names.is_empty() {
            return Err(Error::Other("Workbook has no worksheets".to_string()));
        }
        self.get_sheet_by_name(&sheet_names[0])
    }

    /// Get a worksheet by index (0-based)
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based worksheet index
    ///
    /// # Errors
    ///
    /// Returns error if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let wb = load_workbook("data.xlsx")?;
    /// let ws = wb.get_sheet_by_index(0)?;  // First sheet
    /// ```
    pub fn get_sheet_by_index(&mut self, index: usize) -> Result<Worksheet> {
        let sheet_names = self.sheetnames();
        let name = sheet_names
            .get(index)
            .ok_or_else(|| Error::Other(format!("Sheet index {index} out of bounds")))?;
        self.get_sheet_by_name(name)
    }
}

/// Load an Excel workbook from a file path
///
/// This function mimics `OpenPyXL`'s `load_workbook()` function.
///
/// # Arguments
///
/// * `path` - Path to the Excel file
///
/// # Errors
///
/// Returns error if file cannot be opened or is not a valid Excel file.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::compat::load_workbook;
///
/// let wb = load_workbook("data.xlsx")?;
/// let ws = wb.active()?;
/// let cell = ws.cell(1, 1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_workbook<P: AsRef<Path>>(path: P) -> Result<Workbook> {
    let reader = Reader::open(path)?;
    Ok(Workbook::new(reader))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test load_workbook opens existing file
    #[test]
    fn test_load_workbook() {
        let result = load_workbook("tests/fixtures/test.xlsx");
        assert!(
            result.is_ok(),
            "Failed to load workbook: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test load_workbook fails for non-existent file
    #[test]
    fn test_load_workbook_nonexistent() {
        let result = load_workbook("tests/fixtures/nonexistent.xlsx");
        assert!(result.is_err());
    }

    /// TDD RED: Test sheetnames returns worksheet list
    #[test]
    fn test_workbook_sheetnames() {
        let wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let names = wb.sheetnames();

        assert!(!names.is_empty(), "Should have at least one sheet");
        assert_eq!(names[0], "Sheet1");
    }

    /// TDD RED: Test get_sheet_by_name returns worksheet
    #[test]
    fn test_workbook_get_sheet_by_name() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let ws = wb.get_sheet_by_name("Sheet1");

        assert!(ws.is_ok(), "Failed to get sheet: {:?}", ws.err());
        let ws = ws.unwrap();
        assert_eq!(ws.title(), "Sheet1");
    }

    /// TDD RED: Test get_sheet_by_name fails for non-existent sheet
    #[test]
    fn test_workbook_get_sheet_nonexistent() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let result = wb.get_sheet_by_name("NonExistent");

        assert!(result.is_err());
    }

    /// TDD RED: Test active returns first worksheet
    #[test]
    fn test_workbook_active() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let ws = wb.active();

        assert!(ws.is_ok(), "Failed to get active sheet: {:?}", ws.err());
        let ws = ws.unwrap();
        assert_eq!(ws.title(), "Sheet1");
    }

    /// TDD RED: Test get_sheet_by_index
    #[test]
    fn test_workbook_get_sheet_by_index() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let ws = wb.get_sheet_by_index(0);

        assert!(ws.is_ok(), "Failed to get sheet by index: {:?}", ws.err());
        let ws = ws.unwrap();
        assert_eq!(ws.title(), "Sheet1");
    }

    /// TDD RED: Test get_sheet_by_index fails for out of bounds
    #[test]
    fn test_workbook_get_sheet_by_index_out_of_bounds() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let result = wb.get_sheet_by_index(999);

        assert!(result.is_err());
    }

    /// TDD RED: Test reading cell data through compat layer
    #[test]
    fn test_workbook_read_cell_data() {
        let mut wb = load_workbook("tests/fixtures/test.xlsx").unwrap();
        let ws = wb.active().unwrap();

        // Read cell A1 (should be "Hello")
        let cell = ws.cell(1, 1).unwrap();
        assert_eq!(cell.coordinate(), "A1");

        // Read cell using get_cell
        let cell = ws.get_cell("B1").unwrap();
        assert_eq!(cell.coordinate(), "B1");
    }
}
