//! Column chart implementation
//!
//! Provides `ColumnChart` type for creating vertical bar charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};
use super::line::DataSeries;

/// Column chart configuration
///
/// Creates column charts (vertical bars) with support for multiple data series,
/// titles, legends, and positioning.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{ColumnChart, DataSeries};
///
/// let chart = ColumnChart::new()
///     .title("Quarterly Revenue")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$5")
///         .name("Q1-Q4")
///         .categories("Sheet1!$A$2:$A$5"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnChart {
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
    /// Stacked columns
    stacked: bool,
}

impl ColumnChart {
    /// Create a new column chart
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

    /// Set whether columns should be stacked
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

    /// Check if columns are stacked
    #[must_use]
    pub fn is_stacked(&self) -> bool {
        self.stacked
    }
}

impl Chart for ColumnChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Column
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for ColumnChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test column chart creation
    #[test]
    fn test_column_chart_new() {
        let chart = ColumnChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
        assert!(!chart.is_stacked());
    }

    /// TDD RED: Test column chart with title
    #[test]
    fn test_column_chart_with_title() {
        let chart = ColumnChart::new().title("Revenue Report");
        assert_eq!(Chart::title(&chart), Some("Revenue Report"));
    }

    /// TDD RED: Test column chart with axis titles
    #[test]
    fn test_column_chart_with_axis_titles() {
        let chart = ColumnChart::new()
            .x_axis_title("Quarter")
            .y_axis_title("Revenue ($M)");

        assert_eq!(chart.get_x_axis_title(), Some("Quarter"));
        assert_eq!(chart.get_y_axis_title(), Some("Revenue ($M)"));
    }

    /// TDD RED: Test column chart with series
    #[test]
    fn test_column_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$5").name("Revenue");
        let chart = ColumnChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Revenue"));
    }

    /// TDD RED: Test column chart with multiple series
    #[test]
    fn test_column_chart_with_multiple_series() {
        let chart = ColumnChart::new()
            .add_series(DataSeries::new("Sheet1!$B$2:$B$5").name("2023"))
            .add_series(DataSeries::new("Sheet1!$C$2:$C$5").name("2024"));

        assert_eq!(chart.get_series().len(), 2);
        assert_eq!(chart.get_series()[0].get_name(), Some("2023"));
        assert_eq!(chart.get_series()[1].get_name(), Some("2024"));
    }

    /// TDD RED: Test column chart with position
    #[test]
    fn test_column_chart_with_position() {
        let pos = ChartPosition::new(2, 5).width(600).height(400);
        let chart = ColumnChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 2);
        assert_eq!(chart_pos.col, 5);
        assert_eq!(chart_pos.width, Some(600));
        assert_eq!(chart_pos.height, Some(400));
    }

    /// TDD RED: Test column chart legend control
    #[test]
    fn test_column_chart_legend() {
        let chart = ColumnChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = ColumnChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test column chart stacked mode
    #[test]
    fn test_column_chart_stacked() {
        let chart = ColumnChart::new().stacked(true);
        assert!(chart.is_stacked());

        let chart = ColumnChart::new().stacked(false);
        assert!(!chart.is_stacked());
    }

    /// TDD RED: Test column chart builder pattern
    #[test]
    fn test_column_chart_builder() {
        let chart = ColumnChart::new()
            .title("Sales Performance")
            .x_axis_title("Product")
            .y_axis_title("Units Sold")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$6")
                    .name("North")
                    .categories("Sheet1!$A$2:$A$6"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$6")
                    .name("South")
                    .categories("Sheet1!$A$2:$A$6"),
            )
            .stacked(true)
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Sales Performance"));
        assert_eq!(chart.get_x_axis_title(), Some("Product"));
        assert_eq!(chart.get_y_axis_title(), Some("Units Sold"));
        assert_eq!(chart.get_series().len(), 2);
        assert!(chart.is_stacked());
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_column_chart_trait() {
        let chart = ColumnChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Column);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_column_chart_default() {
        let chart = ColumnChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(!chart.is_stacked());
    }
}
