//! Data validation types and rules
//!
//! Provides comprehensive data validation support for Excel cells including
//! lists, numbers, dates, text length, and custom formulas.

/// Validation error style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationErrorStyle {
    /// Stop - prevents invalid data entry
    #[default]
    Stop,
    /// Warning - shows warning but allows entry
    Warning,
    /// Information - shows info message but allows entry
    Information,
}

/// List validation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ListValidation {
    /// List of allowed values
    values: Vec<String>,
    /// Show dropdown in cell
    show_dropdown: bool,
}

impl ListValidation {
    /// Create a new list validation
    #[must_use]
    pub fn new(values: Vec<String>) -> Self {
        Self {
            values,
            show_dropdown: true,
        }
    }

    /// Set whether to show dropdown
    #[must_use]
    pub fn show_dropdown(mut self, show: bool) -> Self {
        self.show_dropdown = show;
        self
    }

    /// Get the list values
    #[must_use]
    pub fn get_values(&self) -> &[String] {
        &self.values
    }

    /// Check if dropdown is shown
    #[must_use]
    pub fn is_dropdown_shown(&self) -> bool {
        self.show_dropdown
    }
}

/// Number validation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct NumberValidation {
    /// Minimum value (inclusive)
    min: Option<f64>,
    /// Maximum value (inclusive)
    max: Option<f64>,
}

impl NumberValidation {
    /// Create a new number validation with range
    #[must_use]
    pub fn range(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Create a validation for minimum value only
    #[must_use]
    pub fn min(min: f64) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    /// Create a validation for maximum value only
    #[must_use]
    pub fn max(max: f64) -> Self {
        Self {
            min: None,
            max: Some(max),
        }
    }

    /// Get the minimum value
    #[must_use]
    pub fn get_min(&self) -> Option<f64> {
        self.min
    }

    /// Get the maximum value
    #[must_use]
    pub fn get_max(&self) -> Option<f64> {
        self.max
    }
}

/// Date validation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DateValidation {
    /// Minimum date (Excel serial number)
    min: Option<f64>,
    /// Maximum date (Excel serial number)
    max: Option<f64>,
}

impl DateValidation {
    /// Create a new date validation with range
    #[must_use]
    pub fn range(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Create a validation for minimum date only
    #[must_use]
    pub fn min(min: f64) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    /// Create a validation for maximum date only
    #[must_use]
    pub fn max(max: f64) -> Self {
        Self {
            min: None,
            max: Some(max),
        }
    }

    /// Get the minimum date
    #[must_use]
    pub fn get_min(&self) -> Option<f64> {
        self.min
    }

    /// Get the maximum date
    #[must_use]
    pub fn get_max(&self) -> Option<f64> {
        self.max
    }
}

/// Text length validation configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextValidation {
    /// Minimum length
    min_length: Option<usize>,
    /// Maximum length
    max_length: Option<usize>,
}

impl TextValidation {
    /// Create a new text validation with length range
    #[must_use]
    pub fn range(min_length: usize, max_length: usize) -> Self {
        Self {
            min_length: Some(min_length),
            max_length: Some(max_length),
        }
    }

    /// Create a validation for minimum length only
    #[must_use]
    pub fn min_length(min_length: usize) -> Self {
        Self {
            min_length: Some(min_length),
            max_length: None,
        }
    }

    /// Create a validation for maximum length only
    #[must_use]
    pub fn max_length(max_length: usize) -> Self {
        Self {
            min_length: None,
            max_length: Some(max_length),
        }
    }

    /// Get the minimum length
    #[must_use]
    pub fn get_min_length(&self) -> Option<usize> {
        self.min_length
    }

    /// Get the maximum length
    #[must_use]
    pub fn get_max_length(&self) -> Option<usize> {
        self.max_length
    }
}

/// Validation rule types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRule {
    /// List validation (dropdown)
    List(ListValidation),
    /// Number range validation
    Number(NumberValidation),
    /// Date range validation
    Date(DateValidation),
    /// Text length validation
    Text(TextValidation),
    /// Custom formula validation
    Custom(String),
}

/// Validation error configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Error style
    style: ValidationErrorStyle,
    /// Error title
    title: Option<String>,
    /// Error message
    message: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    #[must_use]
    pub fn new(style: ValidationErrorStyle) -> Self {
        Self {
            style,
            title: None,
            message: None,
        }
    }

    /// Set error title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set error message
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Get error style
    #[must_use]
    pub fn get_style(&self) -> ValidationErrorStyle {
        self.style
    }

    /// Get error title
    #[must_use]
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Get error message
    #[must_use]
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl Default for ValidationError {
    fn default() -> Self {
        Self::new(ValidationErrorStyle::Stop)
    }
}

/// Validation input warning configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationWarning {
    /// Warning title
    title: Option<String>,
    /// Warning message
    message: Option<String>,
}

impl ValidationWarning {
    /// Create a new validation warning
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            message: None,
        }
    }

    /// Set warning title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set warning message
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Get warning title
    #[must_use]
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Get warning message
    #[must_use]
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl Default for ValidationWarning {
    fn default() -> Self {
        Self::new()
    }
}

/// Data validation configuration
///
/// Provides comprehensive data validation support for Excel cells.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::validation::{DataValidation, ListValidation};
///
/// // Create a dropdown list validation
/// let validation = DataValidation::new(
///     ValidationRule::List(ListValidation::new(vec![
///         "Option 1".to_string(),
///         "Option 2".to_string(),
///     ]))
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DataValidation {
    /// Validation rule
    rule: ValidationRule,
    /// Error configuration
    error: ValidationError,
    /// Input warning configuration
    warning: Option<ValidationWarning>,
    /// Ignore blank cells
    ignore_blank: bool,
}

impl DataValidation {
    /// Create a new data validation
    #[must_use]
    pub fn new(rule: ValidationRule) -> Self {
        Self {
            rule,
            error: ValidationError::default(),
            warning: None,
            ignore_blank: true,
        }
    }

    /// Set error configuration
    #[must_use]
    pub fn error(mut self, error: ValidationError) -> Self {
        self.error = error;
        self
    }

    /// Set input warning
    #[must_use]
    pub fn warning(mut self, warning: ValidationWarning) -> Self {
        self.warning = Some(warning);
        self
    }

    /// Set whether to ignore blank cells
    #[must_use]
    pub fn ignore_blank(mut self, ignore: bool) -> Self {
        self.ignore_blank = ignore;
        self
    }

    /// Get the validation rule
    #[must_use]
    pub fn get_rule(&self) -> &ValidationRule {
        &self.rule
    }

    /// Get the error configuration
    #[must_use]
    pub fn get_error(&self) -> &ValidationError {
        &self.error
    }

    /// Get the warning configuration
    #[must_use]
    pub fn get_warning(&self) -> Option<&ValidationWarning> {
        self.warning.as_ref()
    }

    /// Check if blank cells are ignored
    #[must_use]
    pub fn is_blank_ignored(&self) -> bool {
        self.ignore_blank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test list validation creation
    #[test]
    fn test_list_validation_new() {
        let validation = ListValidation::new(vec!["A".to_string(), "B".to_string()]);
        assert_eq!(validation.get_values().len(), 2);
        assert!(validation.is_dropdown_shown());
    }

    /// TDD RED: Test list validation dropdown control
    #[test]
    fn test_list_validation_dropdown() {
        let validation = ListValidation::new(vec!["A".to_string()]).show_dropdown(false);
        assert!(!validation.is_dropdown_shown());
    }

    /// TDD RED: Test number validation range
    #[test]
    fn test_number_validation_range() {
        let validation = NumberValidation::range(0.0, 100.0);
        assert_eq!(validation.get_min(), Some(0.0));
        assert_eq!(validation.get_max(), Some(100.0));
    }

    /// TDD RED: Test number validation min only
    #[test]
    fn test_number_validation_min() {
        let validation = NumberValidation::min(10.0);
        assert_eq!(validation.get_min(), Some(10.0));
        assert_eq!(validation.get_max(), None);
    }

    /// TDD RED: Test number validation max only
    #[test]
    fn test_number_validation_max() {
        let validation = NumberValidation::max(50.0);
        assert_eq!(validation.get_min(), None);
        assert_eq!(validation.get_max(), Some(50.0));
    }

    /// TDD RED: Test date validation range
    #[test]
    fn test_date_validation_range() {
        let validation = DateValidation::range(44197.0, 44927.0);
        assert_eq!(validation.get_min(), Some(44197.0));
        assert_eq!(validation.get_max(), Some(44927.0));
    }

    /// TDD RED: Test text validation range
    #[test]
    fn test_text_validation_range() {
        let validation = TextValidation::range(5, 10);
        assert_eq!(validation.get_min_length(), Some(5));
        assert_eq!(validation.get_max_length(), Some(10));
    }

    /// TDD RED: Test text validation min length
    #[test]
    fn test_text_validation_min() {
        let validation = TextValidation::min_length(3);
        assert_eq!(validation.get_min_length(), Some(3));
        assert_eq!(validation.get_max_length(), None);
    }

    /// TDD RED: Test validation error
    #[test]
    fn test_validation_error() {
        let error = ValidationError::new(ValidationErrorStyle::Warning)
            .title("Invalid Entry")
            .message("Please enter a valid value");

        assert_eq!(error.get_style(), ValidationErrorStyle::Warning);
        assert_eq!(error.get_title(), Some("Invalid Entry"));
        assert_eq!(error.get_message(), Some("Please enter a valid value"));
    }

    /// TDD RED: Test validation warning
    #[test]
    fn test_validation_warning() {
        let warning = ValidationWarning::new()
            .title("Input Help")
            .message("Select a value from the list");

        assert_eq!(warning.get_title(), Some("Input Help"));
        assert_eq!(warning.get_message(), Some("Select a value from the list"));
    }

    /// TDD RED: Test data validation with list
    #[test]
    fn test_data_validation_list() {
        let list = ListValidation::new(vec!["Yes".to_string(), "No".to_string()]);
        let validation = DataValidation::new(ValidationRule::List(list));

        assert!(matches!(validation.get_rule(), ValidationRule::List(_)));
        assert!(validation.is_blank_ignored());
    }

    /// TDD RED: Test data validation with number
    #[test]
    fn test_data_validation_number() {
        let number = NumberValidation::range(1.0, 10.0);
        let validation = DataValidation::new(ValidationRule::Number(number));

        assert!(matches!(validation.get_rule(), ValidationRule::Number(_)));
    }

    /// TDD RED: Test data validation builder pattern
    #[test]
    fn test_data_validation_builder() {
        let list = ListValidation::new(vec!["Option 1".to_string()]);
        let error = ValidationError::new(ValidationErrorStyle::Stop).title("Error");
        let warning = ValidationWarning::new().title("Info");

        let validation = DataValidation::new(ValidationRule::List(list))
            .error(error)
            .warning(warning.clone())
            .ignore_blank(false);

        assert!(!validation.is_blank_ignored());
        assert_eq!(validation.get_error().get_title(), Some("Error"));
        assert_eq!(validation.get_warning().unwrap().get_title(), Some("Info"));
    }

    /// TDD RED: Test validation error default
    #[test]
    fn test_validation_error_default() {
        let error = ValidationError::default();
        assert_eq!(error.get_style(), ValidationErrorStyle::Stop);
        assert!(error.get_title().is_none());
        assert!(error.get_message().is_none());
    }
}
