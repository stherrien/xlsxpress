//! Pie chart implementation
//!
//! Provides `PieChart` type for creating pie charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};
use super::line::DataSeries;

/// Pie chart configuration
///
/// Creates pie charts with support for data series, titles, legends,
/// and positioning. Pie charts display data as proportional slices.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{PieChart, DataSeries};
///
/// let chart = PieChart::new()
///     .title("Market Share")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$5")
///         .categories("Sheet1!$A$2:$A$5"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PieChart {
    /// Chart title
    title: Option<String>,
    /// Data series
    series: Vec<DataSeries>,
    /// Chart position on worksheet
    position: Option<ChartPosition>,
    /// Show legend
    show_legend: bool,
}

impl PieChart {
    /// Create a new pie chart
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
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

impl Chart for PieChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Pie
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for PieChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test pie chart creation
    #[test]
    fn test_pie_chart_new() {
        let chart = PieChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test pie chart with title
    #[test]
    fn test_pie_chart_with_title() {
        let chart = PieChart::new().title("Market Share");
        assert_eq!(Chart::title(&chart), Some("Market Share"));
    }

    /// TDD RED: Test pie chart with series
    #[test]
    fn test_pie_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$5")
            .name("Products")
            .categories("Sheet1!$A$2:$A$5");
        let chart = PieChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Products"));
        assert_eq!(
            chart.get_series()[0].get_categories(),
            Some("Sheet1!$A$2:$A$5")
        );
    }

    /// TDD RED: Test pie chart with multiple series
    #[test]
    fn test_pie_chart_with_multiple_series() {
        let chart = PieChart::new()
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$5")
                    .name("Q1")
                    .categories("Sheet1!$A$2:$A$5"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$5")
                    .name("Q2")
                    .categories("Sheet1!$A$2:$A$5"),
            );

        assert_eq!(chart.get_series().len(), 2);
        assert_eq!(chart.get_series()[0].get_name(), Some("Q1"));
        assert_eq!(chart.get_series()[1].get_name(), Some("Q2"));
    }

    /// TDD RED: Test pie chart with position
    #[test]
    fn test_pie_chart_with_position() {
        let pos = ChartPosition::new(1, 4).width(500).height(500);
        let chart = PieChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 1);
        assert_eq!(chart_pos.col, 4);
        assert_eq!(chart_pos.width, Some(500));
        assert_eq!(chart_pos.height, Some(500));
    }

    /// TDD RED: Test pie chart legend control
    #[test]
    fn test_pie_chart_legend() {
        let chart = PieChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = PieChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test pie chart builder pattern
    #[test]
    fn test_pie_chart_builder() {
        let chart = PieChart::new()
            .title("Sales Distribution")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$6")
                    .name("Revenue")
                    .categories("Sheet1!$A$2:$A$6"),
            )
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Sales Distribution"));
        assert_eq!(chart.get_series().len(), 1);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_pie_chart_trait() {
        let chart = PieChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Pie);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_pie_chart_default() {
        let chart = PieChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test pie chart with complete configuration
    #[test]
    fn test_pie_chart_complete() {
        let pos = ChartPosition::new(2, 8).width(600).height(400);
        let chart = PieChart::new()
            .title("Product Distribution")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$8")
                    .name("2024 Sales")
                    .categories("Sheet1!$A$2:$A$8"),
            )
            .position(pos)
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Product Distribution"));
        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("2024 Sales"));
        assert!(chart.is_legend_shown());
        assert!(Chart::position(&chart).is_some());
    }
}
