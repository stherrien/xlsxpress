//! Python bindings for style types

use crate::styles::{
    Alignment, Border, BorderStyle, Fill, FillPattern, Font, HorizontalAlignment, NumberFormat,
    Style, VerticalAlignment,
};
use pyo3::prelude::*;

/// Python wrapper for Font
#[pyclass(name = "Font")]
#[derive(Clone)]
pub struct PyFont {
    pub(crate) inner: Font,
}

#[pymethods]
impl PyFont {
    /// Create a new font
    #[new]
    fn new() -> Self {
        Self { inner: Font::new() }
    }

    /// Set font name
    fn name(mut slf: PyRefMut<'_, Self>, name: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).name(name);
        slf
    }

    /// Set font size in points
    fn size(mut slf: PyRefMut<'_, Self>, size: f64) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).size(size);
        slf
    }

    /// Set bold text
    fn bold(mut slf: PyRefMut<'_, Self>, bold: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).bold(bold);
        slf
    }

    /// Set italic text
    fn italic(mut slf: PyRefMut<'_, Self>, italic: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).italic(italic);
        slf
    }

    /// Set text color from hex string
    fn color(mut slf: PyRefMut<'_, Self>, color: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).color(color);
        slf
    }

    /// Set text color from RGB values
    fn rgb(mut slf: PyRefMut<'_, Self>, r: u8, g: u8, b: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).rgb(r, g, b);
        slf
    }
}

/// Python wrapper for Fill
#[pyclass(name = "Fill")]
#[derive(Clone)]
pub struct PyFill {
    pub(crate) inner: Fill,
}

#[pymethods]
impl PyFill {
    /// Create a new fill
    #[new]
    fn new() -> Self {
        Self { inner: Fill::new() }
    }

    /// Create a solid fill with a color
    #[staticmethod]
    fn solid(color: &str) -> Self {
        Self {
            inner: Fill::solid(color),
        }
    }

    /// Create a pattern fill
    #[staticmethod]
    fn pattern(pattern: u8) -> Self {
        let pattern_enum = match pattern {
            0 => FillPattern::Solid,
            1 => FillPattern::DarkGray,
            2 => FillPattern::MediumGray,
            3 => FillPattern::LightGray,
            4 => FillPattern::Gray125,
            5 => FillPattern::Gray0625,
            _ => FillPattern::Solid,
        };
        Self {
            inner: Fill::pattern(pattern_enum),
        }
    }

    /// Set the fill pattern
    fn set_pattern(mut slf: PyRefMut<'_, Self>, pattern: u8) -> PyRefMut<'_, Self> {
        let pattern_enum = match pattern {
            0 => FillPattern::Solid,
            1 => FillPattern::DarkGray,
            2 => FillPattern::MediumGray,
            3 => FillPattern::LightGray,
            4 => FillPattern::Gray125,
            5 => FillPattern::Gray0625,
            _ => FillPattern::Solid,
        };
        slf.inner = std::mem::take(&mut slf.inner).set_pattern(pattern_enum);
        slf
    }

    /// Set foreground color from RGB values
    fn rgb(mut slf: PyRefMut<'_, Self>, r: u8, g: u8, b: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).rgb(r, g, b);
        slf
    }

    /// Set background color for patterns
    fn background_color(mut slf: PyRefMut<'_, Self>, color: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).background_color(color);
        slf
    }
}

/// Python wrapper for Border
#[pyclass(name = "Border")]
#[derive(Clone)]
pub struct PyBorder {
    pub(crate) inner: Border,
}

#[pymethods]
impl PyBorder {
    /// Create a new border with no borders
    #[new]
    fn new() -> Self {
        Self {
            inner: Border::new(),
        }
    }

    /// Create a border with all edges set to the same style
    #[staticmethod]
    fn all(style: u8) -> Self {
        Self {
            inner: Border::all(border_style_from_u8(style)),
        }
    }

    /// Create a border with outline (top, bottom, left, right)
    #[staticmethod]
    fn outline(style: u8) -> Self {
        Self {
            inner: Border::outline(border_style_from_u8(style)),
        }
    }

    /// Set top border style
    fn top(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).top(border_style_from_u8(style));
        slf
    }

    /// Set bottom border style
    fn bottom(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).bottom(border_style_from_u8(style));
        slf
    }

    /// Set left border style
    fn left(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).left(border_style_from_u8(style));
        slf
    }

    /// Set right border style
    fn right(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).right(border_style_from_u8(style));
        slf
    }

    /// Set diagonal up border style
    fn diagonal_up(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).diagonal_up(border_style_from_u8(style));
        slf
    }

    /// Set diagonal down border style
    fn diagonal_down(mut slf: PyRefMut<'_, Self>, style: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).diagonal_down(border_style_from_u8(style));
        slf
    }

    /// Set border color from hex string
    fn color(mut slf: PyRefMut<'_, Self>, color: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).color(color);
        slf
    }

    /// Set border color from RGB values
    fn rgb(mut slf: PyRefMut<'_, Self>, r: u8, g: u8, b: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).rgb(r, g, b);
        slf
    }
}

/// Helper function to convert u8 to BorderStyle
fn border_style_from_u8(style: u8) -> BorderStyle {
    match style {
        0 => BorderStyle::None,
        1 => BorderStyle::Thin,
        2 => BorderStyle::Medium,
        3 => BorderStyle::Thick,
        4 => BorderStyle::Dashed,
        5 => BorderStyle::Dotted,
        6 => BorderStyle::Double,
        7 => BorderStyle::Hair,
        8 => BorderStyle::MediumDashed,
        9 => BorderStyle::DashDot,
        10 => BorderStyle::MediumDashDot,
        11 => BorderStyle::DashDotDot,
        12 => BorderStyle::MediumDashDotDot,
        13 => BorderStyle::SlantDashDot,
        _ => BorderStyle::Thin,
    }
}

/// Python wrapper for Alignment
#[pyclass(name = "Alignment")]
#[derive(Clone)]
pub struct PyAlignment {
    pub(crate) inner: Alignment,
}

#[pymethods]
impl PyAlignment {
    /// Create a new alignment
    #[new]
    fn new() -> Self {
        Self {
            inner: Alignment::new(),
        }
    }

    /// Set horizontal alignment
    fn horizontal(mut slf: PyRefMut<'_, Self>, align: u8) -> PyRefMut<'_, Self> {
        let align_enum = match align {
            0 => HorizontalAlignment::General,
            1 => HorizontalAlignment::Left,
            2 => HorizontalAlignment::Center,
            3 => HorizontalAlignment::Right,
            4 => HorizontalAlignment::Fill,
            5 => HorizontalAlignment::Justify,
            6 => HorizontalAlignment::CenterAcross,
            7 => HorizontalAlignment::Distributed,
            _ => HorizontalAlignment::General,
        };
        slf.inner = std::mem::take(&mut slf.inner).horizontal(align_enum);
        slf
    }

    /// Set vertical alignment
    fn vertical(mut slf: PyRefMut<'_, Self>, align: u8) -> PyRefMut<'_, Self> {
        let align_enum = match align {
            0 => VerticalAlignment::Top,
            1 => VerticalAlignment::Center,
            2 => VerticalAlignment::Bottom,
            3 => VerticalAlignment::Justify,
            4 => VerticalAlignment::Distributed,
            _ => VerticalAlignment::Top,
        };
        slf.inner = std::mem::take(&mut slf.inner).vertical(align_enum);
        slf
    }

    /// Set text wrapping
    fn wrap_text(mut slf: PyRefMut<'_, Self>, wrap: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).wrap_text(wrap);
        slf
    }

    /// Set text rotation in degrees (0-360)
    fn rotation(mut slf: PyRefMut<'_, Self>, degrees: u16) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).rotation(degrees);
        slf
    }

    /// Set indentation level (0-15)
    fn indent(mut slf: PyRefMut<'_, Self>, level: u8) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).indent(level);
        slf
    }

    /// Set shrink to fit
    fn shrink_to_fit(mut slf: PyRefMut<'_, Self>, shrink: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).shrink_to_fit(shrink);
        slf
    }
}

/// Python wrapper for NumberFormat
#[pyclass(name = "NumberFormat")]
#[derive(Clone)]
pub struct PyNumberFormat {
    pub(crate) inner: NumberFormat,
}

#[pymethods]
impl PyNumberFormat {
    /// Create a new number format
    #[new]
    fn new() -> Self {
        Self {
            inner: NumberFormat::new(),
        }
    }

    /// Create a general number format
    #[staticmethod]
    fn general() -> Self {
        Self {
            inner: NumberFormat::general(),
        }
    }

    /// Create a number format with specified decimal places
    #[staticmethod]
    fn number(decimals: u8) -> Self {
        Self {
            inner: NumberFormat::number(decimals),
        }
    }

    /// Create a currency format with specified decimal places
    #[staticmethod]
    fn currency(decimals: u8) -> Self {
        Self {
            inner: NumberFormat::currency(decimals),
        }
    }

    /// Create an accounting format with specified decimal places
    #[staticmethod]
    fn accounting(decimals: u8) -> Self {
        Self {
            inner: NumberFormat::accounting(decimals),
        }
    }

    /// Create a date format
    #[staticmethod]
    fn date() -> Self {
        Self {
            inner: NumberFormat::date(),
        }
    }

    /// Create a time format
    #[staticmethod]
    fn time() -> Self {
        Self {
            inner: NumberFormat::time(),
        }
    }

    /// Create a percentage format with specified decimal places
    #[staticmethod]
    fn percentage(decimals: u8) -> Self {
        Self {
            inner: NumberFormat::percentage(decimals),
        }
    }

    /// Create a fraction format
    #[staticmethod]
    fn fraction() -> Self {
        Self {
            inner: NumberFormat::fraction(),
        }
    }

    /// Create a scientific notation format with specified decimal places
    #[staticmethod]
    fn scientific(decimals: u8) -> Self {
        Self {
            inner: NumberFormat::scientific(decimals),
        }
    }

    /// Create a text format
    #[staticmethod]
    fn text() -> Self {
        Self {
            inner: NumberFormat::text(),
        }
    }

    /// Create a custom number format
    #[staticmethod]
    fn custom(format: &str) -> Self {
        Self {
            inner: NumberFormat::custom(format),
        }
    }
}

/// Python wrapper for Style
#[pyclass(name = "Style")]
pub struct PyStyle {
    pub(crate) inner: Style,
}

#[pymethods]
impl PyStyle {
    /// Create a new style
    #[new]
    fn new() -> Self {
        Self {
            inner: Style::new(),
        }
    }

    /// Set font styling
    fn font(mut slf: PyRefMut<'_, Self>, font: &PyFont) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).font(font.inner.clone());
        slf
    }

    /// Set fill styling
    fn fill(mut slf: PyRefMut<'_, Self>, fill: &PyFill) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).fill(fill.inner.clone());
        slf
    }

    /// Set border styling
    fn border(mut slf: PyRefMut<'_, Self>, border: &PyBorder) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).border(border.inner.clone());
        slf
    }

    /// Set alignment styling
    fn alignment(mut slf: PyRefMut<'_, Self>, alignment: &PyAlignment) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).alignment(alignment.inner.clone());
        slf
    }

    /// Set number format styling
    fn number_format(
        mut slf: PyRefMut<'_, Self>,
        number_format: &PyNumberFormat,
    ) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).number_format(number_format.inner.clone());
        slf
    }
}
