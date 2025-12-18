//! Cell styling module
//!
//! Provides types for configuring cell appearance including fonts, fills,
//! borders, alignment, and number formats.

pub mod alignment;
pub mod border;
pub mod fill;
pub mod font;
pub mod number_format;
pub mod style;

// Re-export for convenience
pub use alignment::{Alignment, HorizontalAlignment, VerticalAlignment};
pub use border::{Border, BorderStyle};
pub use fill::{Fill, FillPattern};
pub use font::Font;
pub use number_format::{NumberFormat, NumberFormatType};
pub use style::Style;
