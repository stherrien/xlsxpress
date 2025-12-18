//! Composite style for Excel cells
//!
//! Provides Style type that combines font, fill, border, alignment, and
//! number format settings into a single styling unit.

use rust_xlsxwriter::Format;

use super::{Alignment, Border, Fill, Font, NumberFormat};

/// Composite cell style
///
/// Combines multiple styling components (font, fill, border, alignment,
/// number format) into a single style that can be applied to cells.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::styles::{Style, Font, Fill, Border, BorderStyle};
///
/// let style = Style::new()
///     .font(Font::new().bold(true).size(14.0))
///     .fill(Fill::solid("#FFFF00"))
///     .border(Border::all(BorderStyle::Thin));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// Font styling
    font: Option<Font>,
    /// Fill styling
    fill: Option<Fill>,
    /// Border styling
    border: Option<Border>,
    /// Alignment styling
    alignment: Option<Alignment>,
    /// Number format styling
    number_format: Option<NumberFormat>,
}

impl Style {
    /// Create a new Style with no styling components
    #[must_use]
    pub fn new() -> Self {
        Self {
            font: None,
            fill: None,
            border: None,
            alignment: None,
            number_format: None,
        }
    }

    /// Set font styling
    ///
    /// # Arguments
    ///
    /// * `font` - Font configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let style = Style::new()
    ///     .font(Font::new().bold(true).size(14.0));
    /// ```
    #[must_use]
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Set fill styling
    ///
    /// # Arguments
    ///
    /// * `fill` - Fill configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let style = Style::new()
    ///     .fill(Fill::solid("#FFFF00"));
    /// ```
    #[must_use]
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Set border styling
    ///
    /// # Arguments
    ///
    /// * `border` - Border configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let style = Style::new()
    ///     .border(Border::all(BorderStyle::Thin));
    /// ```
    #[must_use]
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// Set alignment styling
    ///
    /// # Arguments
    ///
    /// * `alignment` - Alignment configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let style = Style::new()
    ///     .alignment(Alignment::new()
    ///         .horizontal(HorizontalAlignment::Center));
    /// ```
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set number format styling
    ///
    /// # Arguments
    ///
    /// * `number_format` - Number format configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let style = Style::new()
    ///     .number_format(NumberFormat::currency(2));
    /// ```
    #[must_use]
    pub fn number_format(mut self, number_format: NumberFormat) -> Self {
        self.number_format = Some(number_format);
        self
    }

    /// Apply all style components to a `rust_xlsxwriter` Format
    ///
    /// # Arguments
    ///
    /// * `format` - Format to apply style settings to
    ///
    /// # Returns
    ///
    /// The modified Format (builder pattern)
    #[allow(dead_code)]
    pub(crate) fn apply_to_format(&self, mut format: Format) -> Format {
        // Apply font if set
        if let Some(ref font) = self.font {
            format = font.apply_to_format(format);
        }

        // Apply fill if set
        if let Some(ref fill) = self.fill {
            format = fill.apply_to_format(format);
        }

        // Apply border if set
        if let Some(ref border) = self.border {
            format = border.apply_to_format(format);
        }

        // Apply alignment if set
        if let Some(ref alignment) = self.alignment {
            format = alignment.apply_to_format(format);
        }

        // Apply number format if set
        if let Some(ref number_format) = self.number_format {
            format = number_format.apply_to_format(format);
        }

        format
    }

    /// Get font styling
    #[must_use]
    pub fn get_font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    /// Get fill styling
    #[must_use]
    pub fn get_fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    /// Get border styling
    #[must_use]
    pub fn get_border(&self) -> Option<&Border> {
        self.border.as_ref()
    }

    /// Get alignment styling
    #[must_use]
    pub fn get_alignment(&self) -> Option<&Alignment> {
        self.alignment.as_ref()
    }

    /// Get number format styling
    #[must_use]
    pub fn get_number_format(&self) -> Option<&NumberFormat> {
        self.number_format.as_ref()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styles::{BorderStyle, FillPattern, HorizontalAlignment};

    /// TDD RED: Test style creation with default values
    #[test]
    fn test_style_new() {
        let style = Style::new();
        assert!(style.get_font().is_none());
        assert!(style.get_fill().is_none());
        assert!(style.get_border().is_none());
        assert!(style.get_alignment().is_none());
        assert!(style.get_number_format().is_none());
    }

    /// TDD RED: Test style with font
    #[test]
    fn test_style_with_font() {
        let font = Font::new().bold(true).size(14.0);
        let style = Style::new().font(font.clone());

        assert!(style.get_font().is_some());
        let style_font = style.get_font().unwrap();
        assert!(style_font.is_bold());
        assert_eq!(style_font.get_size(), Some(14.0));
    }

    /// TDD RED: Test style with fill
    #[test]
    fn test_style_with_fill() {
        let fill = Fill::solid("#FFFF00");
        let style = Style::new().fill(fill.clone());

        assert!(style.get_fill().is_some());
        let style_fill = style.get_fill().unwrap();
        assert_eq!(style_fill.get_pattern(), FillPattern::Solid);
    }

    /// TDD RED: Test style with border
    #[test]
    fn test_style_with_border() {
        let border = Border::all(BorderStyle::Thin);
        let style = Style::new().border(border.clone());

        assert!(style.get_border().is_some());
        let style_border = style.get_border().unwrap();
        assert_eq!(style_border.get_top(), BorderStyle::Thin);
    }

    /// TDD RED: Test style with alignment
    #[test]
    fn test_style_with_alignment() {
        let alignment = Alignment::new().horizontal(HorizontalAlignment::Center);
        let style = Style::new().alignment(alignment.clone());

        assert!(style.get_alignment().is_some());
        let style_alignment = style.get_alignment().unwrap();
        assert_eq!(
            style_alignment.get_horizontal(),
            Some(HorizontalAlignment::Center)
        );
    }

    /// TDD RED: Test style with number format
    #[test]
    fn test_style_with_number_format() {
        let number_format = NumberFormat::currency(2);
        let style = Style::new().number_format(number_format.clone());

        assert!(style.get_number_format().is_some());
        let style_number_format = style.get_number_format().unwrap();
        assert_eq!(style_number_format.get_decimals(), Some(2));
    }

    /// TDD RED: Test style builder with all components
    #[test]
    fn test_style_builder_complete() {
        let style = Style::new()
            .font(Font::new().bold(true).size(14.0).color("#FF0000"))
            .fill(Fill::solid("#FFFF00"))
            .border(Border::all(BorderStyle::Thin))
            .alignment(Alignment::new().horizontal(HorizontalAlignment::Center))
            .number_format(NumberFormat::currency(2));

        assert!(style.get_font().is_some());
        assert!(style.get_fill().is_some());
        assert!(style.get_border().is_some());
        assert!(style.get_alignment().is_some());
        assert!(style.get_number_format().is_some());
    }

    /// TDD RED: Test style builder with partial components
    #[test]
    fn test_style_builder_partial() {
        let style = Style::new()
            .font(Font::new().bold(true))
            .fill(Fill::solid("#FFFF00"));

        assert!(style.get_font().is_some());
        assert!(style.get_fill().is_some());
        assert!(style.get_border().is_none());
        assert!(style.get_alignment().is_none());
        assert!(style.get_number_format().is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert!(style.get_font().is_none());
        assert!(style.get_fill().is_none());
    }

    /// TDD RED: Test style cloning
    #[test]
    fn test_style_clone() {
        let style1 = Style::new()
            .font(Font::new().bold(true))
            .fill(Fill::solid("#FFFF00"));

        let style2 = style1.clone();

        assert!(style2.get_font().is_some());
        assert!(style2.get_fill().is_some());
        assert_eq!(style1, style2);
    }
}
