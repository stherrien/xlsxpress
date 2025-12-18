//! Cell wrapper for `OpenPyXL` compatibility
//!
//! Provides a Cell type that mimics `OpenPyXL`'s Cell interface with
//! 1-indexed row/column and A1 notation coordinate.

use crate::compat::utils::coordinate_to_string;

/// Cell value types compatible with `OpenPyXL`
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Empty cell
    Empty,
}

impl From<String> for CellValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for CellValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<f64> for CellValue {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}

impl From<bool> for CellValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Empty => write!(f, ""),
        }
    }
}

/// Cell wrapper compatible with `OpenPyXL`
///
/// Represents a single cell with row, column, and value.
/// Uses 1-indexed row/column to match `OpenPyXL` behavior.
#[derive(Debug, Clone)]
pub struct Cell {
    /// Row number (1-indexed)
    row: usize,
    /// Column number (1-indexed)
    column: usize,
    /// Cell value
    value: CellValue,
}

impl Cell {
    /// Create a new cell
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-indexed)
    /// * `column` - Column number (1-indexed)
    /// * `value` - Cell value
    #[must_use]
    pub fn new(row: usize, column: usize, value: impl Into<CellValue>) -> Self {
        Self {
            row,
            column,
            value: value.into(),
        }
    }

    /// Get the cell's row number (1-indexed)
    #[must_use]
    pub fn row(&self) -> usize {
        self.row
    }

    /// Get the cell's column number (1-indexed)
    #[must_use]
    pub fn column(&self) -> usize {
        self.column
    }

    /// Get the cell's coordinate in A1 notation
    #[must_use]
    pub fn coordinate(&self) -> String {
        coordinate_to_string(self.row, self.column)
    }

    /// Get the cell's value
    #[must_use]
    pub fn value(&self) -> &CellValue {
        &self.value
    }

    /// Set the cell's value
    pub fn set_value(&mut self, value: impl Into<CellValue>) {
        self.value = value.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test cell creation with string value
    #[test]
    fn test_cell_new_string() {
        let cell = Cell::new(1, 1, "Hello");
        assert_eq!(cell.row(), 1);
        assert_eq!(cell.column(), 1);
        assert_eq!(cell.coordinate(), "A1");
        assert_eq!(cell.value(), &CellValue::String("Hello".to_string()));
    }

    /// TDD RED: Test cell creation with number value
    #[test]
    fn test_cell_new_number() {
        let cell = Cell::new(2, 2, 42.0);
        assert_eq!(cell.row(), 2);
        assert_eq!(cell.column(), 2);
        assert_eq!(cell.coordinate(), "B2");
        assert_eq!(cell.value(), &CellValue::Number(42.0));
    }

    /// TDD RED: Test cell creation with boolean value
    #[test]
    fn test_cell_new_boolean() {
        let cell = Cell::new(3, 3, true);
        assert_eq!(cell.row(), 3);
        assert_eq!(cell.column(), 3);
        assert_eq!(cell.coordinate(), "C3");
        assert_eq!(cell.value(), &CellValue::Boolean(true));
    }

    /// TDD RED: Test cell value modification
    #[test]
    fn test_cell_set_value() {
        let mut cell = Cell::new(1, 1, "Initial");
        assert_eq!(cell.value(), &CellValue::String("Initial".to_string()));

        cell.set_value("Updated");
        assert_eq!(cell.value(), &CellValue::String("Updated".to_string()));

        cell.set_value(123.45);
        assert_eq!(cell.value(), &CellValue::Number(123.45));
    }

    /// TDD RED: Test cell with different coordinates
    #[test]
    fn test_cell_coordinates() {
        let cell1 = Cell::new(1, 1, "A1");
        assert_eq!(cell1.coordinate(), "A1");

        let cell2 = Cell::new(100, 27, "AA100");
        assert_eq!(cell2.coordinate(), "AA100");

        let cell3 = Cell::new(1048576, 16384, "Max");
        assert_eq!(cell3.coordinate(), "XFD1048576");
    }

    /// TDD RED: Test CellValue Display trait
    #[test]
    fn test_cell_value_display() {
        assert_eq!(CellValue::String("test".to_string()).to_string(), "test");
        assert_eq!(CellValue::Number(42.5).to_string(), "42.5");
        assert_eq!(CellValue::Boolean(true).to_string(), "true");
        assert_eq!(CellValue::Empty.to_string(), "");
    }
}
