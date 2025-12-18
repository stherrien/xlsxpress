//! Font styling for Excel cells
//!
//! Provides Font type for configuring cell text appearance including
//! font family, size, bold, italic, and color.

use rust_xlsxwriter::{Color, Format};

/// Font configuration for cell styling
///
/// Configures text appearance in Excel cells including font family,
/// size, weight, style, and color.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::Font;
///
/// let font = Font::new()
///     .name("Arial")
///     .size(12)
///     .bold(true)
///     .color("#FF0000");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    /// Font name (e.g., "Arial", "Calibri")
    name: Option<String>,
    /// Font size in points
    size: Option<f64>,
    /// Bold text
    bold: bool,
    /// Italic text
    italic: bool,
    /// Text color
    color: Option<Color>,
}

impl Font {
    /// Create a new Font with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            size: None,
            bold: false,
            italic: false,
            color: None,
        }
    }

    /// Set font name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set font size in points
    #[must_use]
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }

    /// Set bold text
    #[must_use]
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set italic text
    #[must_use]
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Set text color from hex string
    ///
    /// # Arguments
    ///
    /// * `color` - Hex color string like "#FF0000" or "FF0000"
    #[must_use]
    pub fn color(mut self, color: impl Into<String>) -> Self {
        let color_str = color.into();
        let color_str = color_str.trim_start_matches('#');
        if let Ok(parsed) = u32::from_str_radix(color_str, 16) {
            self.color = Some(Color::RGB(parsed));
        }
        self
    }

    /// Set text color from RGB values
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

    /// Apply font settings to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply font settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    pub(crate) fn apply_to_format(&self, mut format: Format) -> Format {
        if let Some(ref name) = self.name {
            format = format.set_font_name(name);
        }
        if let Some(size) = self.size {
            format = format.set_font_size(size);
        }
        if self.bold {
            format = format.set_bold();
        }
        if self.italic {
            format = format.set_italic();
        }
        if let Some(color) = self.color {
            format = format.set_font_color(color);
        }
        format
    }

    /// Get font name
    #[must_use]
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get font size
    #[must_use]
    pub fn get_size(&self) -> Option<f64> {
        self.size
    }

    /// Check if bold
    #[must_use]
    pub fn is_bold(&self) -> bool {
        self.bold
    }

    /// Check if italic
    #[must_use]
    pub fn is_italic(&self) -> bool {
        self.italic
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test font creation with default values
    #[test]
    fn test_font_new() {
        let font = Font::new();
        assert_eq!(font.get_name(), None);
        assert_eq!(font.get_size(), None);
        assert!(!font.is_bold());
        assert!(!font.is_italic());
    }

    /// TDD RED: Test font with name
    #[test]
    fn test_font_name() {
        let font = Font::new().name("Arial");
        assert_eq!(font.get_name(), Some("Arial"));
    }

    /// TDD RED: Test font with size
    #[test]
    fn test_font_size() {
        let font = Font::new().size(12.0);
        assert_eq!(font.get_size(), Some(12.0));
    }

    /// TDD RED: Test font bold
    #[test]
    fn test_font_bold() {
        let font = Font::new().bold(true);
        assert!(font.is_bold());

        let font = Font::new().bold(false);
        assert!(!font.is_bold());
    }

    /// TDD RED: Test font italic
    #[test]
    fn test_font_italic() {
        let font = Font::new().italic(true);
        assert!(font.is_italic());

        let font = Font::new().italic(false);
        assert!(!font.is_italic());
    }

    /// TDD RED: Test font color from hex
    #[test]
    fn test_font_color_hex() {
        let font = Font::new().color("#FF0000");
        assert!(font.color.is_some());

        let font = Font::new().color("00FF00");
        assert!(font.color.is_some());
    }

    /// TDD RED: Test font color from RGB
    #[test]
    fn test_font_color_rgb() {
        let font = Font::new().rgb(255, 0, 0);
        assert!(font.color.is_some());
    }

    /// TDD RED: Test font builder pattern
    #[test]
    fn test_font_builder() {
        let font = Font::new()
            .name("Calibri")
            .size(14.0)
            .bold(true)
            .italic(true)
            .color("#0000FF");

        assert_eq!(font.get_name(), Some("Calibri"));
        assert_eq!(font.get_size(), Some(14.0));
        assert!(font.is_bold());
        assert!(font.is_italic());
        assert!(font.color.is_some());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_font_default() {
        let font = Font::default();
        assert_eq!(font.get_name(), None);
        assert!(!font.is_bold());
    }
}
