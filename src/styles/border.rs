//! Border styling for Excel cells
//!
//! Provides Border type for configuring cell borders including styles,
//! colors, and individual edge configuration.

use rust_xlsxwriter::{Color, Format, FormatBorder};

/// Border style types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Thin border (most common)
    Thin,
    /// Medium border
    Medium,
    /// Thick border
    Thick,
    /// Dashed border
    Dashed,
    /// Dotted border
    Dotted,
    /// Double border
    Double,
    /// Hair-thin border
    Hair,
    /// Medium dashed border
    MediumDashed,
    /// Dash-dot border
    DashDot,
    /// Medium dash-dot border
    MediumDashDot,
    /// Dash-dot-dot border
    DashDotDot,
    /// Medium dash-dot-dot border
    MediumDashDotDot,
    /// Slant dash-dot border
    SlantDashDot,
}

impl From<BorderStyle> for FormatBorder {
    fn from(style: BorderStyle) -> Self {
        match style {
            BorderStyle::None => FormatBorder::None,
            BorderStyle::Thin => FormatBorder::Thin,
            BorderStyle::Medium => FormatBorder::Medium,
            BorderStyle::Thick => FormatBorder::Thick,
            BorderStyle::Dashed => FormatBorder::Dashed,
            BorderStyle::Dotted => FormatBorder::Dotted,
            BorderStyle::Double => FormatBorder::Double,
            BorderStyle::Hair => FormatBorder::Hair,
            BorderStyle::MediumDashed => FormatBorder::MediumDashed,
            BorderStyle::DashDot => FormatBorder::DashDot,
            BorderStyle::MediumDashDot => FormatBorder::MediumDashDot,
            BorderStyle::DashDotDot => FormatBorder::DashDotDot,
            BorderStyle::MediumDashDotDot => FormatBorder::MediumDashDotDot,
            BorderStyle::SlantDashDot => FormatBorder::SlantDashDot,
        }
    }
}

/// Border configuration for cell styling
///
/// Configures cell borders for individual edges or all edges at once.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::{Border, BorderStyle};
///
/// // All borders with thin style
/// let border = Border::all(BorderStyle::Thin);
///
/// // Individual borders
/// let border = Border::new()
///     .top(BorderStyle::Thick)
///     .bottom(BorderStyle::Thick);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    /// Top border style
    top: BorderStyle,
    /// Bottom border style
    bottom: BorderStyle,
    /// Left border style
    left: BorderStyle,
    /// Right border style
    right: BorderStyle,
    /// Diagonal up border style
    diagonal_up: BorderStyle,
    /// Diagonal down border style
    diagonal_down: BorderStyle,
    /// Border color
    color: Option<Color>,
}

impl Border {
    /// Create a new Border with no borders
    #[must_use]
    pub fn new() -> Self {
        Self {
            top: BorderStyle::None,
            bottom: BorderStyle::None,
            left: BorderStyle::None,
            right: BorderStyle::None,
            diagonal_up: BorderStyle::None,
            diagonal_down: BorderStyle::None,
            color: None,
        }
    }

    /// Create a border with all edges set to the same style
    ///
    /// # Arguments
    ///
    /// * `style` - Border style to apply to all edges
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let border = Border::all(BorderStyle::Thin);
    /// ```
    #[must_use]
    pub fn all(style: BorderStyle) -> Self {
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            diagonal_up: BorderStyle::None,
            diagonal_down: BorderStyle::None,
            color: None,
        }
    }

    /// Create a border with outline (top, bottom, left, right)
    ///
    /// # Arguments
    ///
    /// * `style` - Border style for outline
    #[must_use]
    pub fn outline(style: BorderStyle) -> Self {
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            diagonal_up: BorderStyle::None,
            diagonal_down: BorderStyle::None,
            color: None,
        }
    }

    /// Set top border style
    #[must_use]
    pub fn top(mut self, style: BorderStyle) -> Self {
        self.top = style;
        self
    }

    /// Set bottom border style
    #[must_use]
    pub fn bottom(mut self, style: BorderStyle) -> Self {
        self.bottom = style;
        self
    }

    /// Set left border style
    #[must_use]
    pub fn left(mut self, style: BorderStyle) -> Self {
        self.left = style;
        self
    }

    /// Set right border style
    #[must_use]
    pub fn right(mut self, style: BorderStyle) -> Self {
        self.right = style;
        self
    }

    /// Set diagonal up border style
    #[must_use]
    pub fn diagonal_up(mut self, style: BorderStyle) -> Self {
        self.diagonal_up = style;
        self
    }

    /// Set diagonal down border style
    #[must_use]
    pub fn diagonal_down(mut self, style: BorderStyle) -> Self {
        self.diagonal_down = style;
        self
    }

    /// Set border color from hex string
    ///
    /// # Arguments
    ///
    /// * `color` - Hex color string like "#000000" or "000000"
    #[must_use]
    pub fn color(mut self, color: impl Into<String>) -> Self {
        let color_str = color.into();
        let color_str = color_str.trim_start_matches('#');
        if let Ok(parsed) = u32::from_str_radix(color_str, 16) {
            self.color = Some(Color::RGB(parsed));
        }
        self
    }

    /// Set border color from RGB values
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    #[must_use]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        let rgb_value = u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b);
        self.color = Some(Color::RGB(rgb_value));
        self
    }

    /// Apply border settings to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply border settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    pub(crate) fn apply_to_format(&self, mut format: Format) -> Format {
        // Set border styles
        if self.top != BorderStyle::None {
            format = format.set_border_top(self.top.into());
        }
        if self.bottom != BorderStyle::None {
            format = format.set_border_bottom(self.bottom.into());
        }
        if self.left != BorderStyle::None {
            format = format.set_border_left(self.left.into());
        }
        if self.right != BorderStyle::None {
            format = format.set_border_right(self.right.into());
        }
        // Apply diagonal borders - rust_xlsxwriter has a single diagonal method
        // If either diagonal is set, use that style
        if self.diagonal_up != BorderStyle::None {
            format = format.set_border_diagonal(self.diagonal_up.into());
        } else if self.diagonal_down != BorderStyle::None {
            format = format.set_border_diagonal(self.diagonal_down.into());
        }

        // Set border color if specified
        if let Some(color) = self.color {
            format = format.set_border_color(color);
        }

        format
    }

    /// Get top border style
    #[must_use]
    pub fn get_top(&self) -> BorderStyle {
        self.top
    }

    /// Get bottom border style
    #[must_use]
    pub fn get_bottom(&self) -> BorderStyle {
        self.bottom
    }

    /// Get left border style
    #[must_use]
    pub fn get_left(&self) -> BorderStyle {
        self.left
    }

    /// Get right border style
    #[must_use]
    pub fn get_right(&self) -> BorderStyle {
        self.right
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test border creation with no borders
    #[test]
    fn test_border_new() {
        let border = Border::new();
        assert_eq!(border.get_top(), BorderStyle::None);
        assert_eq!(border.get_bottom(), BorderStyle::None);
        assert_eq!(border.get_left(), BorderStyle::None);
        assert_eq!(border.get_right(), BorderStyle::None);
    }

    /// TDD RED: Test all borders with same style
    #[test]
    fn test_border_all() {
        let border = Border::all(BorderStyle::Thin);
        assert_eq!(border.get_top(), BorderStyle::Thin);
        assert_eq!(border.get_bottom(), BorderStyle::Thin);
        assert_eq!(border.get_left(), BorderStyle::Thin);
        assert_eq!(border.get_right(), BorderStyle::Thin);
    }

    /// TDD RED: Test outline borders
    #[test]
    fn test_border_outline() {
        let border = Border::outline(BorderStyle::Medium);
        assert_eq!(border.get_top(), BorderStyle::Medium);
        assert_eq!(border.get_bottom(), BorderStyle::Medium);
        assert_eq!(border.get_left(), BorderStyle::Medium);
        assert_eq!(border.get_right(), BorderStyle::Medium);
    }

    /// TDD RED: Test individual border edges
    #[test]
    fn test_border_individual() {
        let border = Border::new()
            .top(BorderStyle::Thick)
            .bottom(BorderStyle::Thin);

        assert_eq!(border.get_top(), BorderStyle::Thick);
        assert_eq!(border.get_bottom(), BorderStyle::Thin);
        assert_eq!(border.get_left(), BorderStyle::None);
        assert_eq!(border.get_right(), BorderStyle::None);
    }

    /// TDD RED: Test border with color
    #[test]
    fn test_border_color() {
        let border = Border::all(BorderStyle::Thin).color("#FF0000");
        assert!(border.color.is_some());
    }

    /// TDD RED: Test border with RGB color
    #[test]
    fn test_border_rgb() {
        let border = Border::all(BorderStyle::Thin).rgb(0, 0, 255);
        assert!(border.color.is_some());
    }

    /// TDD RED: Test border builder pattern
    #[test]
    fn test_border_builder() {
        let border = Border::new()
            .top(BorderStyle::Thick)
            .bottom(BorderStyle::Thick)
            .left(BorderStyle::Thin)
            .right(BorderStyle::Thin)
            .color("#000000");

        assert_eq!(border.get_top(), BorderStyle::Thick);
        assert_eq!(border.get_bottom(), BorderStyle::Thick);
        assert_eq!(border.get_left(), BorderStyle::Thin);
        assert_eq!(border.get_right(), BorderStyle::Thin);
        assert!(border.color.is_some());
    }

    /// TDD RED: Test diagonal borders
    #[test]
    fn test_border_diagonal() {
        let border = Border::new()
            .diagonal_up(BorderStyle::Thin)
            .diagonal_down(BorderStyle::Thin);

        assert_eq!(border.diagonal_up, BorderStyle::Thin);
        assert_eq!(border.diagonal_down, BorderStyle::Thin);
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_border_default() {
        let border = Border::default();
        assert_eq!(border.get_top(), BorderStyle::None);
    }

    /// TDD RED: Test border style enum conversion
    #[test]
    fn test_border_style_conversion() {
        let styles = vec![
            BorderStyle::None,
            BorderStyle::Thin,
            BorderStyle::Medium,
            BorderStyle::Thick,
            BorderStyle::Dashed,
            BorderStyle::Dotted,
            BorderStyle::Double,
        ];

        for style in styles {
            let _format_border: FormatBorder = style.into();
            // Just verify it compiles and converts
        }
    }
}
