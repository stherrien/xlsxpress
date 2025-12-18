//! Error types for `XlsXpress`
//!
//! This module defines the error types used throughout the library.
//! Following clean code principles, errors provide context about what
//! went wrong and where.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for `XlsXpress` operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for `XlsXpress` operations
#[derive(Error, Debug)]
pub enum Error {
    /// Error reading or opening a file
    #[error("Failed to read file: {path}")]
    FileRead {
        /// Path to the file that failed to read
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// Error writing to a file
    #[error("Failed to write file: {path}")]
    FileWrite {
        /// Path to the file that failed to write
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// Invalid Excel file format
    #[error("Invalid Excel format: {reason}")]
    InvalidFormat {
        /// Explanation of why the format is invalid
        reason: String,
    },

    /// Sheet not found
    #[error("Sheet not found: {name}")]
    SheetNotFound {
        /// Name of the sheet that wasn't found
        name: String,
    },

    /// Cell reference error
    #[error("Invalid cell reference: {reference}")]
    InvalidCellReference {
        /// The invalid cell reference (e.g., "A1")
        reference: String,
    },

    /// Range error
    #[error("Invalid range: {range}")]
    InvalidRange {
        /// The invalid range (e.g., "A1:B10")
        range: String,
    },

    /// Error from calamine (reading)
    #[error("Calamine error: {0}")]
    Calamine(#[from] calamine::Error),

    /// Error from `rust_xlsxwriter` (writing)
    #[error("XlsxWriter error: {0}")]
    XlsxWriter(#[from] rust_xlsxwriter::XlsxError),

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}

impl Error {
    /// Create a new `InvalidFormat` error
    #[must_use]
    pub fn invalid_format(reason: impl Into<String>) -> Self {
        Self::InvalidFormat {
            reason: reason.into(),
        }
    }

    /// Create a new `SheetNotFound` error
    #[must_use]
    pub fn sheet_not_found(name: impl Into<String>) -> Self {
        Self::SheetNotFound { name: name.into() }
    }

    /// Create a new `InvalidCellReference` error
    #[must_use]
    pub fn invalid_cell_reference(reference: impl Into<String>) -> Self {
        Self::InvalidCellReference {
            reference: reference.into(),
        }
    }

    /// Create a new `InvalidRange` error
    #[must_use]
    pub fn invalid_range(range: impl Into<String>) -> Self {
        Self::InvalidRange {
            range: range.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::invalid_format("Not an Excel file");
        assert!(matches!(err, Error::InvalidFormat { .. }));
    }

    #[test]
    fn test_sheet_not_found_error() {
        let err = Error::sheet_not_found("Sheet1");
        assert!(matches!(err, Error::SheetNotFound { .. }));
        assert_eq!(err.to_string(), "Sheet not found: Sheet1");
    }

    #[test]
    fn test_invalid_cell_reference_error() {
        let err = Error::invalid_cell_reference("ZZZ999999");
        assert!(matches!(err, Error::InvalidCellReference { .. }));
    }
}
