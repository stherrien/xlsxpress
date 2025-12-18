//! Line chart implementation
//!
//! Provides `LineChart` type for creating line charts with data series,
//! titles, and customization options.

use super::chart::{Chart, ChartPosition, ChartType};

/// Data series for a line chart
#[derive(Debug, Clone, PartialEq)]
pub struct DataSeries {
    /// Series name
    name: Option<String>,
    /// Categories range (X-axis) in A1 notation
    categories: Option<String>,
    /// Values range (Y-axis) in A1 notation
    values: String,
}

impl DataSeries {
    /// Create a new data series
    ///
    /// # Arguments
    ///
    /// * `values` - Cell range for Y-axis values (e.g., "Sheet1!$A$1:$A$10")
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let series = DataSeries::new("Sheet1!$B$2:$B$10");
    /// ```
    #[must_use]
    pub fn new(values: impl Into<String>) -> Self {
        Self {
            name: None,
            categories: None,
            values: values.into(),
        }
    }

    /// Set series name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set categories range (X-axis)
    ///
    /// # Arguments
    ///
    /// * `categories` - Cell range for X-axis (e.g., "Sheet1!$A$2:$A$10")
    #[must_use]
    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    /// Get series name
    #[must_use]
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get categories range
    #[must_use]
    pub fn get_categories(&self) -> Option<&str> {
        self.categories.as_deref()
    }

    /// Get values range
    #[must_use]
    pub fn get_values(&self) -> &str {
        &self.values
    }
}

/// Line chart configuration
///
/// Creates line charts with support for multiple data series,
/// titles, legends, and positioning.
///
/// # Examples
///
/// ```rust,ignore
/// use xlsxpress::charts::{LineChart, DataSeries};
///
/// let chart = LineChart::new()
///     .title("Sales Data")
///     .add_series(DataSeries::new("Sheet1!$B$2:$B$10")
///         .name("Q1 Sales")
///         .categories("Sheet1!$A$2:$A$10"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LineChart {
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

impl LineChart {
    /// Create a new line chart
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

impl Chart for LineChart {
    fn chart_type(&self) -> ChartType {
        ChartType::Line
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn position(&self) -> Option<&ChartPosition> {
        self.position.as_ref()
    }
}

impl Default for LineChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test data series creation
    #[test]
    fn test_data_series_new() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10");
        assert_eq!(series.get_values(), "Sheet1!$B$2:$B$10");
        assert_eq!(series.get_name(), None);
        assert_eq!(series.get_categories(), None);
    }

    /// TDD RED: Test data series with name
    #[test]
    fn test_data_series_with_name() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10").name("Sales");
        assert_eq!(series.get_name(), Some("Sales"));
    }

    /// TDD RED: Test data series with categories
    #[test]
    fn test_data_series_with_categories() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10").categories("Sheet1!$A$2:$A$10");
        assert_eq!(series.get_categories(), Some("Sheet1!$A$2:$A$10"));
    }

    /// TDD RED: Test data series builder pattern
    #[test]
    fn test_data_series_builder() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10")
            .name("Q1 Sales")
            .categories("Sheet1!$A$2:$A$10");

        assert_eq!(series.get_name(), Some("Q1 Sales"));
        assert_eq!(series.get_categories(), Some("Sheet1!$A$2:$A$10"));
        assert_eq!(series.get_values(), "Sheet1!$B$2:$B$10");
    }

    /// TDD RED: Test line chart creation
    #[test]
    fn test_line_chart_new() {
        let chart = LineChart::new();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test line chart with title
    #[test]
    fn test_line_chart_with_title() {
        let chart = LineChart::new().title("Sales Report");
        assert_eq!(Chart::title(&chart), Some("Sales Report"));
    }

    /// TDD RED: Test line chart with axis titles
    #[test]
    fn test_line_chart_with_axis_titles() {
        let chart = LineChart::new()
            .x_axis_title("Month")
            .y_axis_title("Revenue");

        assert_eq!(chart.get_x_axis_title(), Some("Month"));
        assert_eq!(chart.get_y_axis_title(), Some("Revenue"));
    }

    /// TDD RED: Test line chart with series
    #[test]
    fn test_line_chart_with_series() {
        let series = DataSeries::new("Sheet1!$B$2:$B$10").name("Sales");
        let chart = LineChart::new().add_series(series);

        assert_eq!(chart.get_series().len(), 1);
        assert_eq!(chart.get_series()[0].get_name(), Some("Sales"));
    }

    /// TDD RED: Test line chart with multiple series
    #[test]
    fn test_line_chart_with_multiple_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("Sheet1!$B$2:$B$10").name("Q1"))
            .add_series(DataSeries::new("Sheet1!$C$2:$C$10").name("Q2"))
            .add_series(DataSeries::new("Sheet1!$D$2:$D$10").name("Q3"));

        assert_eq!(chart.get_series().len(), 3);
        assert_eq!(chart.get_series()[0].get_name(), Some("Q1"));
        assert_eq!(chart.get_series()[1].get_name(), Some("Q2"));
        assert_eq!(chart.get_series()[2].get_name(), Some("Q3"));
    }

    /// TDD RED: Test line chart with position
    #[test]
    fn test_line_chart_with_position() {
        let pos = ChartPosition::new(5, 10).width(640).height(480);
        let chart = LineChart::new().position(pos.clone());

        assert!(Chart::position(&chart).is_some());
        let chart_pos = Chart::position(&chart).unwrap();
        assert_eq!(chart_pos.row, 5);
        assert_eq!(chart_pos.col, 10);
        assert_eq!(chart_pos.width, Some(640));
        assert_eq!(chart_pos.height, Some(480));
    }

    /// TDD RED: Test line chart legend control
    #[test]
    fn test_line_chart_legend() {
        let chart = LineChart::new().show_legend(false);
        assert!(!chart.is_legend_shown());

        let chart = LineChart::new().show_legend(true);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test line chart builder pattern
    #[test]
    fn test_line_chart_builder() {
        let chart = LineChart::new()
            .title("Annual Sales")
            .x_axis_title("Quarter")
            .y_axis_title("Revenue ($)")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$5")
                    .name("2023")
                    .categories("Sheet1!$A$2:$A$5"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$5")
                    .name("2024")
                    .categories("Sheet1!$A$2:$A$5"),
            )
            .show_legend(true);

        assert_eq!(Chart::title(&chart), Some("Annual Sales"));
        assert_eq!(chart.get_x_axis_title(), Some("Quarter"));
        assert_eq!(chart.get_y_axis_title(), Some("Revenue ($)"));
        assert_eq!(chart.get_series().len(), 2);
        assert!(chart.is_legend_shown());
    }

    /// TDD RED: Test chart trait implementation
    #[test]
    fn test_line_chart_trait() {
        let chart = LineChart::new().title("Test Chart");

        assert_eq!(chart.chart_type(), ChartType::Line);
        assert_eq!(Chart::title(&chart), Some("Test Chart"));
        assert!(Chart::position(&chart).is_none());
    }

    /// TDD RED: Test default trait
    #[test]
    fn test_line_chart_default() {
        let chart = LineChart::default();
        assert!(Chart::title(&chart).is_none());
        assert_eq!(chart.get_series().len(), 0);
    }
}
