//! Alignment styling for Excel cells
//!
//! Provides Alignment type for configuring cell text alignment including
//! horizontal and vertical alignment, text wrapping, and rotation.

use rust_xlsxwriter::{Format, FormatAlign};

/// Horizontal alignment types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// General alignment (default)
    General,
    /// Left aligned
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
    /// Fill alignment
    Fill,
    /// Justify alignment
    Justify,
    /// Center across selection
    CenterAcross,
    /// Distributed alignment
    Distributed,
}

impl From<HorizontalAlignment> for FormatAlign {
    fn from(align: HorizontalAlignment) -> Self {
        match align {
            HorizontalAlignment::General => FormatAlign::General,
            HorizontalAlignment::Left => FormatAlign::Left,
            HorizontalAlignment::Center => FormatAlign::Center,
            HorizontalAlignment::Right => FormatAlign::Right,
            HorizontalAlignment::Fill => FormatAlign::Fill,
            HorizontalAlignment::Justify => FormatAlign::Justify,
            HorizontalAlignment::CenterAcross => FormatAlign::CenterAcross,
            HorizontalAlignment::Distributed => FormatAlign::Distributed,
        }
    }
}

/// Vertical alignment types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Top aligned
    Top,
    /// Center aligned (middle)
    Center,
    /// Bottom aligned
    Bottom,
    /// Justify alignment
    Justify,
    /// Distributed alignment
    Distributed,
}

impl From<VerticalAlignment> for FormatAlign {
    fn from(align: VerticalAlignment) -> Self {
        match align {
            VerticalAlignment::Top => FormatAlign::Top,
            VerticalAlignment::Center => FormatAlign::VerticalCenter,
            VerticalAlignment::Bottom => FormatAlign::Bottom,
            VerticalAlignment::Justify => FormatAlign::VerticalJustify,
            VerticalAlignment::Distributed => FormatAlign::VerticalDistributed,
        }
    }
}

/// Alignment configuration for cell styling
///
/// Configures text alignment, wrapping, rotation, and indentation in cells.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::{Alignment, HorizontalAlignment, VerticalAlignment};
///
/// // Center aligned text
/// let align = Alignment::new()
///     .horizontal(HorizontalAlignment::Center)
///     .vertical(VerticalAlignment::Center);
///
/// // Wrapped text
/// let align = Alignment::new()
///     .wrap_text(true);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    /// Horizontal alignment
    horizontal: Option<HorizontalAlignment>,
    /// Vertical alignment
    vertical: Option<VerticalAlignment>,
    /// Text wrapping enabled
    wrap_text: bool,
    /// Text rotation (0-360 degrees)
    rotation: Option<u16>,
    /// Indentation level
    indent: Option<u8>,
    /// Shrink to fit
    shrink_to_fit: bool,
}

impl Alignment {
    /// Create a new Alignment with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            horizontal: None,
            vertical: None,
            wrap_text: false,
            rotation: None,
            indent: None,
            shrink_to_fit: false,
        }
    }

    /// Set horizontal alignment
    #[must_use]
    pub fn horizontal(mut self, align: HorizontalAlignment) -> Self {
        self.horizontal = Some(align);
        self
    }

    /// Set vertical alignment
    #[must_use]
    pub fn vertical(mut self, align: VerticalAlignment) -> Self {
        self.vertical = Some(align);
        self
    }

    /// Set text wrapping
    #[must_use]
    pub fn wrap_text(mut self, wrap: bool) -> Self {
        self.wrap_text = wrap;
        self
    }

    /// Set text rotation in degrees (0-360)
    ///
    /// # Arguments
    ///
    /// * `degrees` - Rotation angle in degrees (0-360)
    #[must_use]
    pub fn rotation(mut self, degrees: u16) -> Self {
        // Clamp to 0-360 range
        self.rotation = Some(degrees % 361);
        self
    }

    /// Set indentation level (0-15)
    ///
    /// # Arguments
    ///
    /// * `level` - Indentation level (0-15)
    #[must_use]
    pub fn indent(mut self, level: u8) -> Self {
        // Clamp to 0-15 range
        self.indent = Some(level.min(15));
        self
    }

    /// Set shrink to fit
    #[must_use]
    pub fn shrink_to_fit(mut self, shrink: bool) -> Self {
        self.shrink_to_fit = shrink;
        self
    }

    /// Apply alignment settings to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply alignment settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    #[allow(clippy::cast_possible_wrap)]
    pub(crate) fn apply_to_format(&self, mut format: Format) -> Format {
        // Set horizontal alignment
        if let Some(align) = self.horizontal {
            format = format.set_align(align.into());
        }

        // Set vertical alignment
        if let Some(align) = self.vertical {
            format = format.set_align(align.into());
        }

        // Set text wrapping
        if self.wrap_text {
            format = format.set_text_wrap();
        }

        // Set rotation
        if let Some(rotation) = self.rotation {
            format = format.set_rotation(rotation as i16);
        }

        // Set indentation
        if let Some(indent) = self.indent {
            format = format.set_indent(indent);
        }

        // Set shrink to fit
        if self.shrink_to_fit {
            format = format.set_shrink();
        }

        format
    }

    /// Get horizontal alignment
    #[must_use]
    pub fn get_horizontal(&self) -> Option<HorizontalAlignment> {
        self.horizontal
    }

    /// Get vertical alignment
    #[must_use]
    pub fn get_vertical(&self) -> Option<VerticalAlignment> {
        self.vertical
    }

    /// Check if text wrapping is enabled
    #[must_use]
    pub fn is_wrapped(&self) -> bool {
        self.wrap_text
    }

    /// Get rotation angle
    #[must_use]
    pub fn get_rotation(&self) -> Option<u16> {
        self.rotation
    }

    /// Get indentation level
    #[must_use]
    pub fn get_indent(&self) -> Option<u8> {
        self.indent
    }

    /// Check if shrink to fit is enabled
    #[must_use]
    pub fn is_shrink_to_fit(&self) -> bool {
        self.shrink_to_fit
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test alignment creation with default values
    #[test]
    fn test_alignment_new() {
        let align = Alignment::new();
        assert_eq!(align.get_horizontal(), None);
        assert_eq!(align.get_vertical(), None);
        assert!(!align.is_wrapped());
        assert_eq!(align.get_rotation(), None);
        assert_eq!(align.get_indent(), None);
        assert!(!align.is_shrink_to_fit());
    }

    /// TDD RED: Test horizontal alignment
    #[test]
    fn test_horizontal_alignment() {
        let align = Alignment::new().horizontal(HorizontalAlignment::Center);
        assert_eq!(align.get_horizontal(), Some(HorizontalAlignment::Center));

        let align = Alignment::new().horizontal(HorizontalAlignment::Right);
        assert_eq!(align.get_horizontal(), Some(HorizontalAlignment::Right));
    }

    /// TDD RED: Test vertical alignment
    #[test]
    fn test_vertical_alignment() {
        let align = Alignment::new().vertical(VerticalAlignment::Center);
        assert_eq!(align.get_vertical(), Some(VerticalAlignment::Center));

        let align = Alignment::new().vertical(VerticalAlignment::Bottom);
        assert_eq!(align.get_vertical(), Some(VerticalAlignment::Bottom));
    }

    /// TDD RED: Test text wrapping
    #[test]
    fn test_wrap_text() {
        let align = Alignment::new().wrap_text(true);
        assert!(align.is_wrapped());

        let align = Alignment::new().wrap_text(false);
        assert!(!align.is_wrapped());
    }

    /// TDD RED: Test text rotation
    #[test]
    fn test_rotation() {
        let align = Alignment::new().rotation(45);
        assert_eq!(align.get_rotation(), Some(45));

        let align = Alignment::new().rotation(90);
        assert_eq!(align.get_rotation(), Some(90));
    }

    /// TDD RED: Test rotation clamping
    #[test]
    fn test_rotation_clamping() {
        let align = Alignment::new().rotation(400);
        assert_eq!(align.get_rotation(), Some(39)); // 400 % 361 = 39
    }

    /// TDD RED: Test indentation
    #[test]
    fn test_indent() {
        let align = Alignment::new().indent(3);
        assert_eq!(align.get_indent(), Some(3));

        let align = Alignment::new().indent(10);
        assert_eq!(align.get_indent(), Some(10));
    }

    /// TDD RED: Test indentation clamping
    #[test]
    fn test_indent_clamping() {
        let align = Alignment::new().indent(20);
        assert_eq!(align.get_indent(), Some(15)); // Clamped to max 15
    }

    /// TDD RED: Test shrink to fit
    #[test]
    fn test_shrink_to_fit() {
        let align = Alignment::new().shrink_to_fit(true);
        assert!(align.is_shrink_to_fit());

        let align = Alignment::new().shrink_to_fit(false);
        assert!(!align.is_shrink_to_fit());
    }

    /// TDD RED: Test builder pattern
    #[test]
    fn test_alignment_builder() {
        let align = Alignment::new()
            .horizontal(HorizontalAlignment::Center)
            .vertical(VerticalAlignment::Center)
            .wrap_text(true)
            .rotation(45)
            .indent(2)
            .shrink_to_fit(true);

        assert_eq!(align.get_horizontal(), Some(HorizontalAlignment::Center));
        assert_eq!(align.get_vertical(), Some(VerticalAlignment::Center));
        assert!(align.is_wrapped());
        assert_eq!(align.get_rotation(), Some(45));
        assert_eq!(align.get_indent(), Some(2));
        assert!(align.is_shrink_to_fit());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_alignment_default() {
        let align = Alignment::default();
        assert_eq!(align.get_horizontal(), None);
        assert!(!align.is_wrapped());
    }

    /// TDD RED: Test horizontal alignment enum conversion
    #[test]
    fn test_horizontal_alignment_conversion() {
        let aligns = vec![
            HorizontalAlignment::General,
            HorizontalAlignment::Left,
            HorizontalAlignment::Center,
            HorizontalAlignment::Right,
            HorizontalAlignment::Fill,
            HorizontalAlignment::Justify,
            HorizontalAlignment::CenterAcross,
            HorizontalAlignment::Distributed,
        ];

        for align in aligns {
            let _format_align: FormatAlign = align.into();
            // Just verify it compiles and converts
        }
    }

    /// TDD RED: Test vertical alignment enum conversion
    #[test]
    fn test_vertical_alignment_conversion() {
        let aligns = vec![
            VerticalAlignment::Top,
            VerticalAlignment::Center,
            VerticalAlignment::Bottom,
            VerticalAlignment::Justify,
            VerticalAlignment::Distributed,
        ];

        for align in aligns {
            let _format_align: FormatAlign = align.into();
            // Just verify it compiles and converts
        }
    }
}
