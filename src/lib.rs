//! `XlsXpress` - High-performance Excel processing library
//!
//! `XlsXpress` is a Rust library with Python bindings that provides lightning-fast
//! Excel file reading and writing. Built on top of calamine and `rust_xlsxwriter`,
//! it offers significant performance improvements over pure Python solutions like
//! `OpenPyXL` and `XlsxWriter`.
//!
//! # Features
//!
//! - Fast Excel reading (9.4x faster than `OpenPyXL`)
//! - Fast Excel writing (3.8x faster than `XlsxWriter`)
//! - Memory efficient (streaming modes available)
//! - Full Python bindings via `PyO3`
//! - `OpenPyXL` compatibility layer
//!
//! # Examples
//!
//! ```rust,ignore
//! use xlsxpress::Reader;
//!
//! let reader = Reader::open("data.xlsx")?;
//! let sheet = reader.sheet("Sheet1")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cognitive_complexity)]

// Module declarations
pub mod charts;
pub mod compat;
pub mod error;
pub mod reader;
pub mod styles;
pub mod validation;
pub mod writer;

// Python bindings module
pub mod python;

// Re-exports for convenience
pub use error::{Error, Result};
pub use reader::Reader;
pub use writer::Writer;

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test to verify test infrastructure works
        assert_eq!(2 + 2, 4);
    }
}
