//! `OpenPyXL` compatibility layer
//!
//! This module provides an API compatible with `OpenPyXL` for easy migration.
//! It wraps the native `XlsXpress` Reader and Writer APIs with `OpenPyXL`-style
//! interfaces including A1 notation, 1-indexed rows/columns, and similar method names.

pub mod cell;
pub mod utils;
pub mod workbook;
pub mod worksheet;

// Re-export for convenience
pub use cell::{Cell, CellValue};
pub use utils::{
    column_index_from_string, coordinate_from_string, coordinate_to_string, get_column_letter,
};
pub use workbook::{load_workbook, Workbook};
pub use worksheet::{RowIterator, Worksheet};
