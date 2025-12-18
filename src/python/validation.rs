//! Python bindings for validation types

use crate::validation::{
    DataValidation, DateValidation, ListValidation, NumberValidation, TextValidation,
    ValidationError, ValidationErrorStyle, ValidationRule, ValidationWarning,
};
use pyo3::prelude::*;

/// Python wrapper for ListValidation
#[pyclass(name = "ListValidation")]
#[derive(Clone)]
pub struct PyListValidation {
    pub(crate) inner: ListValidation,
}

#[pymethods]
impl PyListValidation {
    /// Create a new list validation
    #[new]
    fn new(values: Vec<String>) -> Self {
        Self {
            inner: ListValidation::new(values),
        }
    }

    /// Set whether to show dropdown
    fn show_dropdown(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_dropdown(show);
        slf
    }
}

/// Python wrapper for NumberValidation
#[pyclass(name = "NumberValidation")]
#[derive(Clone)]
pub struct PyNumberValidation {
    pub(crate) inner: NumberValidation,
}

#[pymethods]
impl PyNumberValidation {
    /// Create a new number validation
    #[new]
    fn new() -> Self {
        Self {
            inner: NumberValidation::range(0.0, 0.0),
        }
    }

    /// Create a number validation with range
    #[staticmethod]
    fn range(min: f64, max: f64) -> Self {
        Self {
            inner: NumberValidation::range(min, max),
        }
    }

    /// Create a validation for minimum value only
    #[staticmethod]
    fn min(min: f64) -> Self {
        Self {
            inner: NumberValidation::min(min),
        }
    }

    /// Create a validation for maximum value only
    #[staticmethod]
    fn max(max: f64) -> Self {
        Self {
            inner: NumberValidation::max(max),
        }
    }
}

/// Python wrapper for DateValidation
#[pyclass(name = "DateValidation")]
#[derive(Clone)]
pub struct PyDateValidation {
    pub(crate) inner: DateValidation,
}

#[pymethods]
impl PyDateValidation {
    /// Create a new date validation
    #[new]
    fn new() -> Self {
        Self {
            inner: DateValidation::range(0.0, 0.0),
        }
    }

    /// Create a date validation with range
    #[staticmethod]
    fn range(min: f64, max: f64) -> Self {
        Self {
            inner: DateValidation::range(min, max),
        }
    }

    /// Create a validation for minimum date only
    #[staticmethod]
    fn min(min: f64) -> Self {
        Self {
            inner: DateValidation::min(min),
        }
    }

    /// Create a validation for maximum date only
    #[staticmethod]
    fn max(max: f64) -> Self {
        Self {
            inner: DateValidation::max(max),
        }
    }
}

/// Python wrapper for TextValidation
#[pyclass(name = "TextValidation")]
#[derive(Clone)]
pub struct PyTextValidation {
    pub(crate) inner: TextValidation,
}

#[pymethods]
impl PyTextValidation {
    /// Create a new text validation
    #[new]
    fn new() -> Self {
        Self {
            inner: TextValidation::range(0, 0),
        }
    }

    /// Create a text validation with length range
    #[staticmethod]
    fn range(min_length: usize, max_length: usize) -> Self {
        Self {
            inner: TextValidation::range(min_length, max_length),
        }
    }

    /// Create a validation for minimum length only
    #[staticmethod]
    fn min_length(min_length: usize) -> Self {
        Self {
            inner: TextValidation::min_length(min_length),
        }
    }

    /// Create a validation for maximum length only
    #[staticmethod]
    fn max_length(max_length: usize) -> Self {
        Self {
            inner: TextValidation::max_length(max_length),
        }
    }
}

/// Python wrapper for ValidationError
#[pyclass(name = "ValidationError")]
#[derive(Clone)]
pub struct PyValidationError {
    pub(crate) inner: ValidationError,
}

#[pymethods]
impl PyValidationError {
    /// Create a new validation error
    #[new]
    fn new(style: u8) -> Self {
        let style_enum = match style {
            0 => ValidationErrorStyle::Stop,
            1 => ValidationErrorStyle::Warning,
            2 => ValidationErrorStyle::Information,
            _ => ValidationErrorStyle::Stop,
        };
        Self {
            inner: ValidationError::new(style_enum),
        }
    }

    /// Set error title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set error message
    fn message(mut slf: PyRefMut<'_, Self>, message: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).message(message);
        slf
    }
}

/// Python wrapper for ValidationWarning
#[pyclass(name = "ValidationWarning")]
#[derive(Clone)]
pub struct PyValidationWarning {
    pub(crate) inner: ValidationWarning,
}

#[pymethods]
impl PyValidationWarning {
    /// Create a new validation warning
    #[new]
    fn new() -> Self {
        Self {
            inner: ValidationWarning::new(),
        }
    }

    /// Set warning title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set warning message
    fn message(mut slf: PyRefMut<'_, Self>, message: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).message(message);
        slf
    }
}

/// Python wrapper for DataValidation
#[pyclass(name = "DataValidation")]
pub struct PyDataValidation {
    pub(crate) inner: DataValidation,
}

#[pymethods]
impl PyDataValidation {
    /// Create a new data validation with a list
    #[staticmethod]
    fn list(list: &PyListValidation) -> Self {
        Self {
            inner: DataValidation::new(ValidationRule::List(list.inner.clone())),
        }
    }

    /// Create a new data validation with a number range
    #[staticmethod]
    fn number(number: &PyNumberValidation) -> Self {
        Self {
            inner: DataValidation::new(ValidationRule::Number(number.inner.clone())),
        }
    }

    /// Create a new data validation with a date range
    #[staticmethod]
    fn date(date: &PyDateValidation) -> Self {
        Self {
            inner: DataValidation::new(ValidationRule::Date(date.inner.clone())),
        }
    }

    /// Create a new data validation with text length constraints
    #[staticmethod]
    fn text(text: &PyTextValidation) -> Self {
        Self {
            inner: DataValidation::new(ValidationRule::Text(text.inner.clone())),
        }
    }

    /// Create a new data validation with a custom formula
    #[staticmethod]
    fn custom(formula: &str) -> Self {
        Self {
            inner: DataValidation::new(ValidationRule::Custom(formula.to_string())),
        }
    }

    /// Set error configuration
    fn error(mut slf: PyRefMut<'_, Self>, error: &PyValidationError) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).error(error.inner.clone());
        slf
    }

    /// Set input warning
    fn warning(mut slf: PyRefMut<'_, Self>, warning: &PyValidationWarning) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).warning(warning.inner.clone());
        slf
    }

    /// Set whether to ignore blank cells
    fn ignore_blank(mut slf: PyRefMut<'_, Self>, ignore: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).ignore_blank(ignore);
        slf
    }
}
