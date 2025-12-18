//! Data validation module
//!
//! Provides types for creating cell data validation rules in Excel worksheets.

pub mod types;

// Re-export for convenience
pub use types::{
    DataValidation, DateValidation, ListValidation, NumberValidation, TextValidation,
    ValidationError, ValidationErrorStyle, ValidationRule, ValidationWarning,
};
