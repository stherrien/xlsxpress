//! Bar chart implementation
//!
//! Provides `BarChart` type for creating horizontal bar charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};
use super::line::DataSeries;

/// Bar chart configuration
///
/// Creates bar charts (horizontal bars) with support for multiple data series,
/// titles, legends, and positioning.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{BarChart, DataSeries};
///
/// let chart = BarChart::new()
///     .title("Department Comparison")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$5")
///         .name("Budget")
///         .categories("Sheet1!$A$2:$A$5"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BarChart {
    /// Chart title
    title: Option<String>,
    /// X-axis title
    x_axis_title: Option<String>,
    /// Y-axis title
    y_axis_title: Option<String>,
    /// Data series
    series: Vec<DataSeries>,
    /// Chart position on worksheet
    position: Option<ChartPosition>,
    /// Show legend
    show_legend: bool,
    /// Stacked bars
    stacked: bool,
}

impl BarChart {
    /// Create a new bar chart
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            series: Vec::new(),
            position: None,
            show_legend: true,
            stacked: false,
        }
    }

    /// Set chart title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set X-axis title
    #[must_use]
    pub fn x_axis_title(mut self, title: impl Into<String>) -> Self {
        self.x_axis_title = Some(title.into());
        self
    }

    /// Set Y-axis title
    #[must_use]
    pub fn y_axis_title(mut self, title: impl Into<String>) -> Self {
        self.y_axis_title = Some(title.into());
        self
    }

    /// Add a data series to the chart
    #[must_use]
    pub fn add_series(mut self, series: DataSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Set chart position on worksheet
    #[must_use]
    pub fn position(mut self, position: ChartPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Set whether to show legend
    #[must_use]
    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Set whether bars should be stacked
    #[must_use]
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }

    /// Get X-axis title
    #[must_use]
    pub fn get_x_axis_title(&self) -> Option<&str> {
        self.x_axis_title.as_deref()
    }

    /// Get Y-axis title
    #[must_use]
    pub fn get_y_axis_title(&self) -> Option<&str> {
        self.y_axis_title.as_deref()
    }

    /// Get data series
    #[must_use]
    pub fn get_series(&self) -> &[DataSeries] {
        &self.series
    }

    /// Check if legend is shown
    #[must_use]
    pub fn is_legend_shown(&self) -> bool {
        self.show_legend
    }

    /// Check if bars are stacked
    #[must_use]
    pub fn is_stacked(&self) -> bool {
        self.stacked
    }
}

impl Chart for BarChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Bar
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for BarChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test bar chart creation
    #[test]
    fn test_bar_chart_new() {
        let chart = BarChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
        assert!(!chart.is_stacked());
    }

    /// TDD RED: Test bar chart with title
    #[test]
    fn test_bar_chart_with_title() {
        let chart = BarChart::new().title("Budget Analysis");
        assert_eq!(Chart::title(&chart), Some("Budget Analysis"));
    }

    /// TDD RED: Test bar chart with axis titles
    #[test]
    fn test_bar_chart_with_axis_titles() {
        let chart = BarChart::new()
            .x_axis_title("Amount ($)")
            .y_axis_title("Department");

        assert_eq!(chart.get_x_axis_title(), Some("Amount ($)"));
        assert_eq!(chart.get_y_axis_title(), Some("Department"));
    }

    /// TDD RED: Test bar chart with series
    #[test]
    fn test_bar_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$5").name("Spending");
        let chart = BarChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Spending"));
    }

    /// TDD RED: Test bar chart with multiple series
    #[test]
    fn test_bar_chart_with_multiple_series() {
        let chart = BarChart::new()
            .add_series(DataSeries::new("Sheet1!$B$2:$B$5").name("Planned"))
            .add_series(DataSeries::new("Sheet1!$C$2:$C$5").name("Actual"));

        assert_eq!(chart.get_series().len(), 2);
        assert_eq!(chart.get_series()[0].get_name(), Some("Planned"));
        assert_eq!(chart.get_series()[1].get_name(), Some("Actual"));
    }

    /// TDD RED: Test bar chart with position
    #[test]
    fn test_bar_chart_with_position() {
        let pos = ChartPosition::new(3, 8).width(700).height(500);
        let chart = BarChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 3);
        assert_eq!(chart_pos.col, 8);
        assert_eq!(chart_pos.width, Some(700));
        assert_eq!(chart_pos.height, Some(500));
    }

    /// TDD RED: Test bar chart legend control
    #[test]
    fn test_bar_chart_legend() {
        let chart = BarChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = BarChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test bar chart stacked mode
    #[test]
    fn test_bar_chart_stacked() {
        let chart = BarChart::new().stacked(true);
        assert!(chart.is_stacked());

        let chart = BarChart::new().stacked(false);
        assert!(!chart.is_stacked());
    }

    /// TDD RED: Test bar chart builder pattern
    #[test]
    fn test_bar_chart_builder() {
        let chart = BarChart::new()
            .title("Regional Performance")
            .x_axis_title("Sales Volume")
            .y_axis_title("Region")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$4")
                    .name("Q1")
                    .categories("Sheet1!$A$2:$A$4"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$4")
                    .name("Q2")
                    .categories("Sheet1!$A$2:$A$4"),
            )
            .stacked(true)
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Regional Performance"));
        assert_eq!(chart.get_x_axis_title(), Some("Sales Volume"));
        assert_eq!(chart.get_y_axis_title(), Some("Region"));
        assert_eq!(chart.get_series().len(), 2);
        assert!(chart.is_stacked());
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_bar_chart_trait() {
        let chart = BarChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Bar);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_bar_chart_default() {
        let chart = BarChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(!chart.is_stacked());
    }
}
