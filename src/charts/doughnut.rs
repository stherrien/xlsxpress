//! Doughnut chart implementation
//!
//! Provides `DoughnutChart` type for creating doughnut charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};
use super::line::DataSeries;

/// Doughnut chart configuration
///
/// Creates doughnut charts (pie charts with a hole in the center) with support
/// for data series, titles, legends, and positioning.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{DoughnutChart, DataSeries};
///
/// let chart = DoughnutChart::new()
///     .title("Budget Allocation")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$6")
///         .categories("Sheet1!$A$2:$A$6"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DoughnutChart {
    /// Chart title
    title: Option<String>,
    /// Data series
    series: Vec<DataSeries>,
    /// Chart position on worksheet
    position: Option<ChartPosition>,
    /// Show legend
    show_legend: bool,
}

impl DoughnutChart {
    /// Create a new doughnut chart
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

impl Chart for DoughnutChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Doughnut
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for DoughnutChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test doughnut chart creation
    #[test]
    fn test_doughnut_chart_new() {
        let chart = DoughnutChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test doughnut chart with title
    #[test]
    fn test_doughnut_chart_with_title() {
        let chart = DoughnutChart::new().title("Budget Allocation");
        assert_eq!(Chart::title(&chart), Some("Budget Allocation"));
    }

    /// TDD RED: Test doughnut chart with series
    #[test]
    fn test_doughnut_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$6")
            .name("Departments")
            .categories("Sheet1!$A$2:$A$6");
        let chart = DoughnutChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Departments"));
        assert_eq!(
            chart.get_series()[0].get_categories(),
            Some("Sheet1!$A$2:$A$6")
        );
    }

    /// TDD RED: Test doughnut chart with multiple series
    #[test]
    fn test_doughnut_chart_with_multiple_series() {
        let chart = DoughnutChart::new()
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$6")
                    .name("2023")
                    .categories("Sheet1!$A$2:$A$6"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$6")
                    .name("2024")
                    .categories("Sheet1!$A$2:$A$6"),
            );

        assert_eq!(chart.get_series().len(), 2);
        assert_eq!(chart.get_series()[0].get_name(), Some("2023"));
        assert_eq!(chart.get_series()[1].get_name(), Some("2024"));
    }

    /// TDD RED: Test doughnut chart with position
    #[test]
    fn test_doughnut_chart_with_position() {
        let pos = ChartPosition::new(2, 3).width(550).height(450);
        let chart = DoughnutChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 2);
        assert_eq!(chart_pos.col, 3);
        assert_eq!(chart_pos.width, Some(550));
        assert_eq!(chart_pos.height, Some(450));
    }

    /// TDD RED: Test doughnut chart legend control
    #[test]
    fn test_doughnut_chart_legend() {
        let chart = DoughnutChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = DoughnutChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test doughnut chart builder pattern
    #[test]
    fn test_doughnut_chart_builder() {
        let chart = DoughnutChart::new()
            .title("Expense Distribution")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$7")
                    .name("Expenses")
                    .categories("Sheet1!$A$2:$A$7"),
            )
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Expense Distribution"));
        assert_eq!(chart.get_series().len(), 1);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_doughnut_chart_trait() {
        let chart = DoughnutChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Doughnut);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_doughnut_chart_default() {
        let chart = DoughnutChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test doughnut chart with complete configuration
    #[test]
    fn test_doughnut_chart_complete() {
        let pos = ChartPosition::new(3, 5).width(650).height(500);
        let chart = DoughnutChart::new()
            .title("Portfolio Allocation")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$10")
                    .name("Investments")
                    .categories("Sheet1!$A$2:$A$10"),
            )
            .position(pos)
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Portfolio Allocation"));
        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Investments"));
        assert!(chart.is_legend_shown());
        assert!(Chart::position(&chart).is_some());
    }
}
