//! Number format styling for Excel cells
//!
//! Provides `NumberFormat` type for configuring how cell values are displayed
//! including currency, percentages, dates, and custom formats.

use rust_xlsxwriter::Format;

/// Predefined number format types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberFormatType {
    /// General format (default)
    General,
    /// Number format with decimals
    Number,
    /// Currency format
    Currency,
    /// Accounting format
    Accounting,
    /// Date format
    Date,
    /// Time format
    Time,
    /// Percentage format
    Percentage,
    /// Fraction format
    Fraction,
    /// Scientific notation
    Scientific,
    /// Text format
    Text,
    /// Custom format string
    Custom(String),
}

/// Number format configuration for cell styling
///
/// Configures how numeric values are displayed in cells including
/// currency symbols, decimal places, date/time formats, and more.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::NumberFormat;
///
/// // Currency format
/// let fmt = NumberFormat::currency(2);
///
/// // Percentage format
/// let fmt = NumberFormat::percentage(1);
///
/// // Custom format
/// let fmt = NumberFormat::custom("0.00%");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NumberFormat {
    /// Format type
    format_type: NumberFormatType,
    /// Number of decimal places (for numeric formats)
    decimals: Option<u8>,
}

impl NumberFormat {
    /// Create a new `NumberFormat` with general format
    #[must_use]
    pub fn new() -> Self {
        Self {
            format_type: NumberFormatType::General,
            decimals: None,
        }
    }

    /// Create a general number format
    #[must_use]
    pub fn general() -> Self {
        Self::new()
    }

    /// Create a number format with specified decimal places
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places (0-30)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::number(2);  // "0.00"
    /// ```
    #[must_use]
    pub fn number(decimals: u8) -> Self {
        Self {
            format_type: NumberFormatType::Number,
            decimals: Some(decimals.min(30)),
        }
    }

    /// Create a currency format with specified decimal places
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places (0-30)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::currency(2);  // "$#,##0.00"
    /// ```
    #[must_use]
    pub fn currency(decimals: u8) -> Self {
        Self {
            format_type: NumberFormatType::Currency,
            decimals: Some(decimals.min(30)),
        }
    }

    /// Create an accounting format with specified decimal places
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places (0-30)
    #[must_use]
    pub fn accounting(decimals: u8) -> Self {
        Self {
            format_type: NumberFormatType::Accounting,
            decimals: Some(decimals.min(30)),
        }
    }

    /// Create a date format
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::date();  // "yyyy-mm-dd"
    /// ```
    #[must_use]
    pub fn date() -> Self {
        Self {
            format_type: NumberFormatType::Date,
            decimals: None,
        }
    }

    /// Create a time format
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::time();  // "hh:mm:ss"
    /// ```
    #[must_use]
    pub fn time() -> Self {
        Self {
            format_type: NumberFormatType::Time,
            decimals: None,
        }
    }

    /// Create a percentage format with specified decimal places
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places (0-30)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::percentage(2);  // "0.00%"
    /// ```
    #[must_use]
    pub fn percentage(decimals: u8) -> Self {
        Self {
            format_type: NumberFormatType::Percentage,
            decimals: Some(decimals.min(30)),
        }
    }

    /// Create a fraction format
    #[must_use]
    pub fn fraction() -> Self {
        Self {
            format_type: NumberFormatType::Fraction,
            decimals: None,
        }
    }

    /// Create a scientific notation format with specified decimal places
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places (0-30)
    #[must_use]
    pub fn scientific(decimals: u8) -> Self {
        Self {
            format_type: NumberFormatType::Scientific,
            decimals: Some(decimals.min(30)),
        }
    }

    /// Create a text format
    #[must_use]
    pub fn text() -> Self {
        Self {
            format_type: NumberFormatType::Text,
            decimals: None,
        }
    }

    /// Create a custom number format
    ///
    /// # Arguments
    ///
    /// * `format` - Custom format string (Excel format syntax)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fmt = NumberFormat::custom("0.00%");
    /// let fmt = NumberFormat::custom("$#,##0.00_);[Red]($#,##0.00)");
    /// ```
    #[must_use]
    pub fn custom(format: impl Into<String>) -> Self {
        Self {
            format_type: NumberFormatType::Custom(format.into()),
            decimals: None,
        }
    }

    /// Get the format string for this number format
    fn get_format_string(&self) -> String {
        match &self.format_type {
            NumberFormatType::General => String::from("General"),
            NumberFormatType::Number => {
                let decimals = self.decimals.unwrap_or(2);
                if decimals == 0 {
                    String::from("0")
                } else {
                    format!("0.{}", "0".repeat(decimals as usize))
                }
            }
            NumberFormatType::Currency => {
                let decimals = self.decimals.unwrap_or(2);
                if decimals == 0 {
                    String::from("$#,##0")
                } else {
                    format!("$#,##0.{}", "0".repeat(decimals as usize))
                }
            }
            NumberFormatType::Accounting => {
                let decimals = self.decimals.unwrap_or(2);
                if decimals == 0 {
                    String::from("_($* #,##0_);_($* (#,##0);_($* \"-\"_);_(@_)")
                } else {
                    format!(
                        "_($* #,##0.{}_);_($* (#,##0.{});_($* \"-\"??_);_(@_)",
                        "0".repeat(decimals as usize),
                        "0".repeat(decimals as usize)
                    )
                }
            }
            NumberFormatType::Date => String::from("yyyy-mm-dd"),
            NumberFormatType::Time => String::from("hh:mm:ss"),
            NumberFormatType::Percentage => {
                let decimals = self.decimals.unwrap_or(2);
                if decimals == 0 {
                    String::from("0%")
                } else {
                    format!("0.{}%", "0".repeat(decimals as usize))
                }
            }
            NumberFormatType::Fraction => String::from("# ?/?"),
            NumberFormatType::Scientific => {
                let decimals = self.decimals.unwrap_or(2);
                if decimals == 0 {
                    String::from("0E+00")
                } else {
                    format!("0.{}E+00", "0".repeat(decimals as usize))
                }
            }
            NumberFormatType::Text => String::from("@"),
            NumberFormatType::Custom(fmt) => fmt.clone(),
        }
    }

    /// Apply number format settings to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply number format settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    pub(crate) fn apply_to_format(&self, format: Format) -> Format {
        let format_string = self.get_format_string();
        format.set_num_format(&format_string)
    }

    /// Get the format type
    #[must_use]
    pub fn get_format_type(&self) -> &NumberFormatType {
        &self.format_type
    }

    /// Get the number of decimal places
    #[must_use]
    pub fn get_decimals(&self) -> Option<u8> {
        self.decimals
    }
}

impl Default for NumberFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test number format creation with default values
    #[test]
    fn test_number_format_new() {
        let fmt = NumberFormat::new();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::General);
        assert_eq!(fmt.get_decimals(), None);
    }

    /// TDD RED: Test general format
    #[test]
    fn test_general_format() {
        let fmt = NumberFormat::general();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::General);
        assert_eq!(fmt.get_format_string(), "General");
    }

    /// TDD RED: Test number format
    #[test]
    fn test_number_format() {
        let fmt = NumberFormat::number(2);
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Number);
        assert_eq!(fmt.get_decimals(), Some(2));
        assert_eq!(fmt.get_format_string(), "0.00");

        let fmt = NumberFormat::number(0);
        assert_eq!(fmt.get_format_string(), "0");
    }

    /// TDD RED: Test currency format
    #[test]
    fn test_currency_format() {
        let fmt = NumberFormat::currency(2);
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Currency);
        assert_eq!(fmt.get_decimals(), Some(2));
        assert_eq!(fmt.get_format_string(), "$#,##0.00");

        let fmt = NumberFormat::currency(0);
        assert_eq!(fmt.get_format_string(), "$#,##0");
    }

    /// TDD RED: Test accounting format
    #[test]
    fn test_accounting_format() {
        let fmt = NumberFormat::accounting(2);
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Accounting);
        assert_eq!(fmt.get_decimals(), Some(2));
        assert!(fmt.get_format_string().contains('$'));
    }

    /// TDD RED: Test date format
    #[test]
    fn test_date_format() {
        let fmt = NumberFormat::date();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Date);
        assert_eq!(fmt.get_format_string(), "yyyy-mm-dd");
    }

    /// TDD RED: Test time format
    #[test]
    fn test_time_format() {
        let fmt = NumberFormat::time();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Time);
        assert_eq!(fmt.get_format_string(), "hh:mm:ss");
    }

    /// TDD RED: Test percentage format
    #[test]
    fn test_percentage_format() {
        let fmt = NumberFormat::percentage(2);
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Percentage);
        assert_eq!(fmt.get_decimals(), Some(2));
        assert_eq!(fmt.get_format_string(), "0.00%");

        let fmt = NumberFormat::percentage(0);
        assert_eq!(fmt.get_format_string(), "0%");
    }

    /// TDD RED: Test fraction format
    #[test]
    fn test_fraction_format() {
        let fmt = NumberFormat::fraction();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Fraction);
        assert_eq!(fmt.get_format_string(), "# ?/?");
    }

    /// TDD RED: Test scientific format
    #[test]
    fn test_scientific_format() {
        let fmt = NumberFormat::scientific(2);
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Scientific);
        assert_eq!(fmt.get_decimals(), Some(2));
        assert_eq!(fmt.get_format_string(), "0.00E+00");

        let fmt = NumberFormat::scientific(0);
        assert_eq!(fmt.get_format_string(), "0E+00");
    }

    /// TDD RED: Test text format
    #[test]
    fn test_text_format() {
        let fmt = NumberFormat::text();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::Text);
        assert_eq!(fmt.get_format_string(), "@");
    }

    /// TDD RED: Test custom format
    #[test]
    fn test_custom_format() {
        let fmt = NumberFormat::custom("0.00%");
        assert_eq!(
            *fmt.get_format_type(),
            NumberFormatType::Custom(String::from("0.00%"))
        );
        assert_eq!(fmt.get_format_string(), "0.00%");

        let fmt = NumberFormat::custom("$#,##0.00_);[Red]($#,##0.00)");
        assert_eq!(fmt.get_format_string(), "$#,##0.00_);[Red]($#,##0.00)");
    }

    /// TDD RED: Test decimal clamping
    #[test]
    fn test_decimal_clamping() {
        let fmt = NumberFormat::number(50);
        assert_eq!(fmt.get_decimals(), Some(30)); // Clamped to max 30
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_number_format_default() {
        let fmt = NumberFormat::default();
        assert_eq!(*fmt.get_format_type(), NumberFormatType::General);
    }
}
