//! Scatter chart implementation
//!
//! Provides `ScatterChart` type for creating scatter (XY) charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};
use super::line::DataSeries;

/// Scatter chart configuration
///
/// Creates scatter (XY) charts with support for multiple data series,
/// titles, legends, and positioning. Scatter charts plot data points on X/Y axes.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{ScatterChart, DataSeries};
///
/// let chart = ScatterChart::new()
///     .title("Temperature vs Pressure")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$10")
///         .name("Experiment 1")
///         .categories("Sheet1!$A$2:$A$10"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ScatterChart {
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
}

impl ScatterChart {
    /// Create a new scatter chart
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            series: Vec::new(),
            position: None,
            show_legend: true,
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
}

impl Chart for ScatterChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Scatter
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for ScatterChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test scatter chart creation
    #[test]
    fn test_scatter_chart_new() {
        let chart = ScatterChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test scatter chart with title
    #[test]
    fn test_scatter_chart_with_title() {
        let chart = ScatterChart::new().title("Temperature vs Pressure");
        assert_eq!(Chart::title(&chart), Some("Temperature vs Pressure"));
    }

    /// TDD RED: Test scatter chart with axis titles
    #[test]
    fn test_scatter_chart_with_axis_titles() {
        let chart = ScatterChart::new()
            .x_axis_title("Temperature (°C)")
            .y_axis_title("Pressure (bar)");

        assert_eq!(chart.get_x_axis_title(), Some("Temperature (°C)"));
        assert_eq!(chart.get_y_axis_title(), Some("Pressure (bar)"));
    }

    /// TDD RED: Test scatter chart with series
    #[test]
    fn test_scatter_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10")
            .name("Experiment 1")
            .categories("Sheet1!$A$2:$A$10");
        let chart = ScatterChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Experiment 1"));
    }

    /// TDD RED: Test scatter chart with multiple series
    #[test]
    fn test_scatter_chart_with_multiple_series() {
        let chart = ScatterChart::new()
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$10")
                    .name("Series A")
                    .categories("Sheet1!$A$2:$A$10"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$10")
                    .name("Series B")
                    .categories("Sheet1!$A$2:$A$10"),
            );

        assert_eq!(chart.get_series().len(), 2);
        assert_eq!(chart.get_series()[0].get_name(), Some("Series A"));
        assert_eq!(chart.get_series()[1].get_name(), Some("Series B"));
    }

    /// TDD RED: Test scatter chart with position
    #[test]
    fn test_scatter_chart_with_position() {
        let pos = ChartPosition::new(4, 6).width(800).height(600);
        let chart = ScatterChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 4);
        assert_eq!(chart_pos.col, 6);
        assert_eq!(chart_pos.width, Some(800));
        assert_eq!(chart_pos.height, Some(600));
    }

    /// TDD RED: Test scatter chart legend control
    #[test]
    fn test_scatter_chart_legend() {
        let chart = ScatterChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = ScatterChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test scatter chart builder pattern
    #[test]
    fn test_scatter_chart_builder() {
        let chart = ScatterChart::new()
            .title("Correlation Analysis")
            .x_axis_title("Independent Variable")
            .y_axis_title("Dependent Variable")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$20")
                    .name("Dataset 1")
                    .categories("Sheet1!$A$2:$A$20"),
            )
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Correlation Analysis"));
        assert_eq!(chart.get_x_axis_title(), Some("Independent Variable"));
        assert_eq!(chart.get_y_axis_title(), Some("Dependent Variable"));
        assert_eq!(chart.get_series().len(), 1);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_scatter_chart_trait() {
        let chart = ScatterChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Scatter);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_scatter_chart_default() {
        let chart = ScatterChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }
}
