//! Worksheet wrapper for `OpenPyXL` compatibility
//!
//! Provides a Worksheet type that mimics `OpenPyXL`'s Worksheet interface with
//! cell access via coordinates and 1-indexed row/column methods.

use crate::compat::cell::{Cell, CellValue};
use crate::compat::utils::{coordinate_from_string, coordinate_to_string};
use crate::error::{Error, Result};
use calamine::{Data, Range};

/// Worksheet wrapper compatible with `OpenPyXL`
///
/// Represents a single worksheet with cell data and metadata.
/// Uses 1-indexed rows/columns to match `OpenPyXL` behavior.
pub struct Worksheet {
    /// Worksheet name/title
    title: String,
    /// Cell data range from calamine
    range: Range<Data>,
}

impl Worksheet {
    /// Create a new worksheet from a calamine Range
    ///
    /// # Arguments
    ///
    /// * `title` - Name of the worksheet
    /// * `range` - Cell data range
    #[must_use]
    pub fn new(title: impl Into<String>, range: Range<Data>) -> Self {
        Self {
            title: title.into(),
            range,
        }
    }

    /// Get the worksheet title/name
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the maximum row number (1-indexed)
    ///
    /// Returns 0 if worksheet is empty.
    #[must_use]
    pub fn max_row(&self) -> usize {
        let (rows, _) = self.range.get_size();
        rows
    }

    /// Get the maximum column number (1-indexed)
    ///
    /// Returns 0 if worksheet is empty.
    #[must_use]
    pub fn max_column(&self) -> usize {
        let (_, cols) = self.range.get_size();
        cols
    }

    /// Get dimensions as (rows, columns) tuple
    #[must_use]
    pub fn dimensions(&self) -> (usize, usize) {
        self.range.get_size()
    }

    /// Get a cell by coordinate string (e.g., "A1", "B2")
    ///
    /// # Arguments
    ///
    /// * `coord` - Cell coordinate like "A1", "B2", "AA100"
    ///
    /// # Errors
    ///
    /// Returns error if coordinate is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let cell = ws.get_cell("A1")?;
    /// ```
    pub fn get_cell(&self, coord: &str) -> Result<Cell> {
        let (row, col) = coordinate_from_string(coord)?;
        self.cell(row, col)
    }

    /// Get a cell by row and column (1-indexed)
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-indexed)
    /// * `column` - Column number (1-indexed)
    ///
    /// # Errors
    ///
    /// Returns error if coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let cell = ws.cell(1, 1)?;  // A1
    /// let cell = ws.cell(2, 2)?;  // B2
    /// ```
    #[allow(clippy::cast_precision_loss)]
    pub fn cell(&self, row: usize, column: usize) -> Result<Cell> {
        if row == 0 || column == 0 {
            return Err(Error::invalid_cell_reference(coordinate_to_string(
                row, column,
            )));
        }

        // Convert to 0-indexed for calamine
        let row_idx = row - 1;
        let col_idx = column - 1;

        // Get cell value from range
        let value = self
            .range
            .get((row_idx, col_idx))
            .map_or(CellValue::Empty, |data| match data {
                Data::String(s) => CellValue::String(s.clone()),
                Data::Float(f) => CellValue::Number(*f),
                Data::Int(i) => CellValue::Number(*i as f64),
                Data::Bool(b) => CellValue::Boolean(*b),
                Data::Empty
                | Data::Error(_)
                | Data::DateTime(_)
                | Data::DateTimeIso(_)
                | Data::DurationIso(_) => CellValue::Empty,
            });

        Ok(Cell::new(row, column, value))
    }

    /// Iterate over rows in the worksheet
    ///
    /// # Arguments
    ///
    /// * `min_row` - Starting row (1-indexed, inclusive)
    /// * `max_row` - Ending row (1-indexed, inclusive)
    /// * `min_col` - Starting column (1-indexed, inclusive)
    /// * `max_col` - Ending column (1-indexed, inclusive)
    ///
    /// # Returns
    ///
    /// Iterator over rows, where each row is a Vec of Cells
    #[must_use]
    pub fn iter_rows(
        &self,
        min_row: usize,
        max_row: usize,
        min_col: usize,
        max_col: usize,
    ) -> RowIterator<'_> {
        RowIterator {
            worksheet: self,
            current_row: min_row,
            max_row,
            min_col,
            max_col,
        }
    }
}

/// Iterator over worksheet rows
pub struct RowIterator<'a> {
    worksheet: &'a Worksheet,
    current_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
}

impl Iterator for RowIterator<'_> {
    type Item = Vec<Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row > self.max_row {
            return None;
        }

        let mut row_cells = Vec::new();
        for col in self.min_col..=self.max_col {
            if let Ok(cell) = self.worksheet.cell(self.current_row, col) {
                row_cells.push(cell);
            }
        }

        self.current_row += 1;
        Some(row_cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Data;

    /// Helper: Create a test range with sample data
    fn create_test_range() -> Range<Data> {
        use calamine::Cell as CalCell;

        // Create cells with positions
        let cells = vec![
            CalCell::new((0, 0), Data::String("Hello".to_string())),
            CalCell::new((0, 1), Data::Float(42.0)),
            CalCell::new((0, 2), Data::Bool(true)),
            CalCell::new((1, 0), Data::String("World".to_string())),
            CalCell::new((1, 1), Data::Float(3.14)),
            CalCell::new((1, 2), Data::Bool(false)),
        ];

        Range::from_sparse(cells)
    }

    /// TDD RED: Test worksheet creation
    #[test]
    fn test_worksheet_new() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);
        assert_eq!(ws.title(), "Sheet1");
    }

    /// TDD RED: Test worksheet dimensions
    #[test]
    fn test_worksheet_dimensions() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        assert_eq!(ws.max_row(), 2);
        assert_eq!(ws.max_column(), 3);
        assert_eq!(ws.dimensions(), (2, 3));
    }

    /// TDD RED: Test cell access by row/column
    #[test]
    fn test_worksheet_cell() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        // Test A1 (1, 1)
        let cell = ws.cell(1, 1).unwrap();
        assert_eq!(cell.row(), 1);
        assert_eq!(cell.column(), 1);
        assert_eq!(cell.value(), &CellValue::String("Hello".to_string()));

        // Test B1 (1, 2)
        let cell = ws.cell(1, 2).unwrap();
        assert_eq!(cell.value(), &CellValue::Number(42.0));

        // Test C2 (2, 3)
        let cell = ws.cell(2, 3).unwrap();
        assert_eq!(cell.value(), &CellValue::Boolean(false));
    }

    /// TDD RED: Test cell access by coordinate string
    #[test]
    fn test_worksheet_get_cell() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        // Test A1
        let cell = ws.get_cell("A1").unwrap();
        assert_eq!(cell.coordinate(), "A1");
        assert_eq!(cell.value(), &CellValue::String("Hello".to_string()));

        // Test B2
        let cell = ws.get_cell("B2").unwrap();
        assert_eq!(cell.coordinate(), "B2");
        assert_eq!(cell.value(), &CellValue::Number(3.14));
    }

    /// TDD RED: Test invalid cell access
    #[test]
    fn test_worksheet_invalid_cell() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        // Zero-indexed should fail
        assert!(ws.cell(0, 1).is_err());
        assert!(ws.cell(1, 0).is_err());

        // Invalid coordinate
        assert!(ws.get_cell("A0").is_err());
        assert!(ws.get_cell("0A").is_err());
    }

    /// TDD RED: Test empty cell returns Empty value
    #[test]
    fn test_worksheet_empty_cell() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        // Cell beyond data range should be Empty
        let cell = ws.cell(10, 10).unwrap();
        assert_eq!(cell.value(), &CellValue::Empty);
    }

    /// TDD RED: Test row iterator
    #[test]
    fn test_worksheet_iter_rows() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        let rows: Vec<Vec<Cell>> = ws.iter_rows(1, 2, 1, 3).collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 3);
        assert_eq!(rows[1].len(), 3);

        // Check first row values
        assert_eq!(rows[0][0].value(), &CellValue::String("Hello".to_string()));
        assert_eq!(rows[0][1].value(), &CellValue::Number(42.0));
        assert_eq!(rows[0][2].value(), &CellValue::Boolean(true));

        // Check second row values
        assert_eq!(rows[1][0].value(), &CellValue::String("World".to_string()));
        assert_eq!(rows[1][1].value(), &CellValue::Number(3.14));
        assert_eq!(rows[1][2].value(), &CellValue::Boolean(false));
    }

    /// TDD RED: Test row iterator with subset of columns
    #[test]
    fn test_worksheet_iter_rows_subset() {
        let range = create_test_range();
        let ws = Worksheet::new("Sheet1", range);

        // Get only columns 1-2 (A-B)
        let rows: Vec<Vec<Cell>> = ws.iter_rows(1, 2, 1, 2).collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 2);
    }
}
