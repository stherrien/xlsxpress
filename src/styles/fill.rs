//! Fill styling for Excel cells
//!
//! Provides Fill type for configuring cell background colors and patterns.

use rust_xlsxwriter::{Color, Format, FormatPattern};

/// Fill pattern types for cell backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillPattern {
    /// Solid fill (most common)
    Solid,
    /// Dark gray pattern
    DarkGray,
    /// Medium gray pattern
    MediumGray,
    /// Light gray pattern
    LightGray,
    /// Gray 125 pattern
    Gray125,
    /// Gray 0625 pattern
    Gray0625,
}

impl From<FillPattern> for FormatPattern {
    fn from(pattern: FillPattern) -> Self {
        match pattern {
            FillPattern::Solid => FormatPattern::Solid,
            FillPattern::DarkGray => FormatPattern::DarkGray,
            FillPattern::MediumGray => FormatPattern::MediumGray,
            FillPattern::LightGray => FormatPattern::LightGray,
            FillPattern::Gray125 => FormatPattern::Gray125,
            FillPattern::Gray0625 => FormatPattern::Gray0625,
        }
    }
}

/// Fill configuration for cell styling
///
/// Configures cell background appearance including color and pattern.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::Fill;
///
/// // Solid fill with color
/// let fill = Fill::solid("#FFFF00");
///
/// // Pattern fill
/// let fill = Fill::pattern(FillPattern::LightGray);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    /// Fill pattern type
    pattern: FillPattern,
    /// Foreground color
    foreground_color: Option<Color>,
    /// Background color (for patterns)
    background_color: Option<Color>,
}

impl Fill {
    /// Create a new Fill with default settings (no fill)
    #[must_use]
    pub fn new() -> Self {
        Self {
            pattern: FillPattern::Solid,
            foreground_color: None,
            background_color: None,
        }
    }

    /// Create a solid fill with a color
    ///
    /// # Arguments
    ///
    /// * `color` - Hex color string like "#FFFF00" or "FFFF00"
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fill = Fill::solid("#FFFF00");  // Yellow
    /// ```
    #[must_use]
    pub fn solid(color: impl Into<String>) -> Self {
        let mut fill = Self::new();
        fill.pattern = FillPattern::Solid;
        fill.set_color(color);
        fill
    }

    /// Create a pattern fill
    ///
    /// # Arguments
    ///
    /// * `pattern` - Fill pattern type
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let fill = Fill::pattern(FillPattern::LightGray);
    /// ```
    #[must_use]
    pub fn pattern(pattern: FillPattern) -> Self {
        let mut fill = Self::new();
        fill.pattern = pattern;
        fill
    }

    /// Set the fill pattern
    #[must_use]
    pub fn set_pattern(mut self, pattern: FillPattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Set foreground color from hex string
    ///
    /// # Arguments
    ///
    /// * `color` - Hex color string like "#FF0000" or "FF0000"
    pub fn set_color(&mut self, color: impl Into<String>) {
        let color_str = color.into();
        let color_str = color_str.trim_start_matches('#');
        if let Ok(parsed) = u32::from_str_radix(color_str, 16) {
            self.foreground_color = Some(Color::RGB(parsed));
        }
    }

    /// Set foreground color from RGB values
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    #[must_use]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        let rgb_value = u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b);
        self.foreground_color = Some(Color::RGB(rgb_value));
        self
    }

    /// Set background color for patterns
    ///
    /// # Arguments
    ///
    /// * `color` - Hex color string like "#FFFFFF" or "FFFFFF"
    #[must_use]
    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        let color_str = color.into();
        let color_str = color_str.trim_start_matches('#');
        if let Ok(parsed) = u32::from_str_radix(color_str, 16) {
            self.background_color = Some(Color::RGB(parsed));
        }
        self
    }

    /// Apply fill settings to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply fill settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    pub(crate) fn apply_to_format(&self, mut format: Format) -> Format {
        // Set pattern
        format = format.set_pattern(self.pattern.into());

        // Set foreground color
        if let Some(color) = self.foreground_color {
            format = format.set_background_color(color);
        }

        // Set background color for patterns
        if let Some(color) = self.background_color {
            format = format.set_foreground_color(color);
        }

        format
    }

    /// Get the fill pattern
    #[must_use]
    pub fn get_pattern(&self) -> FillPattern {
        self.pattern
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test fill creation with default values
    #[test]
    fn test_fill_new() {
        let fill = Fill::new();
        assert_eq!(fill.get_pattern(), FillPattern::Solid);
        assert!(fill.foreground_color.is_none());
        assert!(fill.background_color.is_none());
    }

    /// TDD RED: Test solid fill with color
    #[test]
    fn test_fill_solid() {
        let fill = Fill::solid("#FFFF00");
        assert_eq!(fill.get_pattern(), FillPattern::Solid);
        assert!(fill.foreground_color.is_some());
    }

    /// TDD RED: Test solid fill with hex color (no #)
    #[test]
    fn test_fill_solid_no_hash() {
        let fill = Fill::solid("FF0000");
        assert!(fill.foreground_color.is_some());
    }

    /// TDD RED: Test pattern fill
    #[test]
    fn test_fill_pattern() {
        let fill = Fill::pattern(FillPattern::LightGray);
        assert_eq!(fill.get_pattern(), FillPattern::LightGray);
    }

    /// TDD RED: Test set pattern
    #[test]
    fn test_fill_set_pattern() {
        let fill = Fill::new().set_pattern(FillPattern::DarkGray);
        assert_eq!(fill.get_pattern(), FillPattern::DarkGray);
    }

    /// TDD RED: Test RGB color
    #[test]
    fn test_fill_rgb() {
        let fill = Fill::new().rgb(255, 0, 0);
        assert!(fill.foreground_color.is_some());
    }

    /// TDD RED: Test background color
    #[test]
    fn test_fill_background_color() {
        let fill = Fill::new().background_color("#FFFFFF");
        assert!(fill.background_color.is_some());
    }

    /// TDD RED: Test builder pattern
    #[test]
    fn test_fill_builder() {
        let fill = Fill::new()
            .set_pattern(FillPattern::Solid)
            .rgb(0, 255, 0)
            .background_color("#000000");

        assert_eq!(fill.get_pattern(), FillPattern::Solid);
        assert!(fill.foreground_color.is_some());
        assert!(fill.background_color.is_some());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_fill_default() {
        let fill = Fill::default();
        assert_eq!(fill.get_pattern(), FillPattern::Solid);
    }

    /// TDD RED: Test fill pattern enum conversion
    #[test]
    fn test_fill_pattern_conversion() {
        let patterns = vec![
            FillPattern::Solid,
            FillPattern::DarkGray,
            FillPattern::MediumGray,
            FillPattern::LightGray,
            FillPattern::Gray125,
            FillPattern::Gray0625,
        ];

        for pattern in patterns {
            let _format_pattern: FormatPattern = pattern.into();
            // Just verify it compiles and converts
        }
    }
}
