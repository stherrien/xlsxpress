//! Base chart types and configuration
//!
//! Provides common chart functionality and configuration options.

/// Chart types available in Excel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    /// Line chart
    Line,
    /// Column chart (vertical bars)
    Column,
    /// Bar chart (horizontal bars)
    Bar,
    /// Pie chart
    Pie,
    /// Scatter chart
    Scatter,
    /// Area chart
    Area,
    /// Doughnut chart
    Doughnut,
}

/// Chart positioning on worksheet
#[derive(Debug, Clone, PartialEq)]
pub struct ChartPosition {
    /// Top-left cell row
    pub row: u32,
    /// Top-left cell column
    pub col: u16,
    /// Width in pixels
    pub width: Option<u32>,
    /// Height in pixels
    pub height: Option<u32>,
}

impl ChartPosition {
    /// Create a new chart position
    ///
    /// # Arguments
    ///
    /// * `row` - Top-left cell row
    /// * `col` - Top-left cell column
    #[must_use]
    pub fn new(row: u32, col: u16) -> Self {
        Self {
            row,
            col,
            width: None,
            height: None,
        }
    }

    /// Set chart width in pixels
    #[must_use]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set chart height in pixels
    #[must_use]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
}

/// Base chart trait
pub trait Chart {
    /// Get the chart type
    fn chart_type(&self) -> ChartType;

    /// Get the chart title
    fn title(&self) -> Option<&str>;

    /// Get the chart position
    fn position(&self) -> Option<&ChartPosition>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test chart position creation
    #[test]
    fn test_chart_position_new() {
        let pos = ChartPosition::new(5, 10);
        assert_eq!(pos.row, 5);
        assert_eq!(pos.col, 10);
        assert_eq!(pos.width, None);
        assert_eq!(pos.height, None);
    }

    /// TDD RED: Test chart position with dimensions
    #[test]
    fn test_chart_position_with_dimensions() {
        let pos = ChartPosition::new(5, 10).width(640).height(480);
        assert_eq!(pos.row, 5);
        assert_eq!(pos.col, 10);
        assert_eq!(pos.width, Some(640));
        assert_eq!(pos.height, Some(480));
    }

    /// TDD RED: Test chart type enum
    #[test]
    fn test_chart_type_enum() {
        let chart_types = vec![
            ChartType::Line,
            ChartType::Column,
            ChartType::Bar,
            ChartType::Pie,
            ChartType::Scatter,
            ChartType::Area,
            ChartType::Doughnut,
        ];

        for chart_type in chart_types {
            // Just verify enum values exist and can be compared
            assert_eq!(chart_type, chart_type);
        }
    }
}
