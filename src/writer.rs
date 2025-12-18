//! Excel file writing functionality
//!
//! This module provides fast Excel file writing using `rust_xlsxwriter`.
//! Follows TDD and clean code principles with functions kept under 20 lines
//! and cognitive complexity under 15.

use crate::charts::{
    AreaChart, BarChart, ColumnChart, DoughnutChart, LineChart, PieChart, ScatterChart,
};
use crate::error::Result;
use crate::styles::Style;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use rust_xlsxwriter::{Chart, ChartType, ExcelDateTime, Format, Workbook};
use std::path::Path;

/// Excel file writer
///
/// Provides high-performance writing of Excel files using `rust_xlsxwriter`.
/// Supports .xlsx and .xlsm formats.
///
/// # Examples
///
/// ```rust,no_run
/// use xlsxpress::Writer;
///
/// let mut writer = Writer::new();
/// writer.add_worksheet("Sheet1")?;
/// writer.save("output.xlsx")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Writer {
    /// Internal `rust_xlsxwriter` workbook
    workbook: Workbook,
}

impl Writer {
    /// Create a new Excel file writer
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Writer;
    ///
    /// let writer = Writer::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            workbook: Workbook::new(),
        }
    }

    /// Add a worksheet to the workbook
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the worksheet
    ///
    /// # Errors
    ///
    /// Returns error if worksheet cannot be created.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.add_worksheet("Sheet1")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn add_worksheet(&mut self, name: &str) -> Result<()> {
        // GREEN phase: Minimal implementation
        self.workbook.add_worksheet().set_name(name)?;
        Ok(())
    }

    /// Write a string value to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - String value to write
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_string(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: &str,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_string(row as u32, col as u16, value)?;
        Ok(())
    }

    /// Write a number value to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - Number value to write
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_number(&mut self, sheet: usize, row: usize, col: usize, value: f64) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_number(row as u32, col as u16, value)?;
        Ok(())
    }

    /// Write a boolean value to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - Boolean value to write
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_boolean(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: bool,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_boolean(row as u32, col as u16, value)?;
        Ok(())
    }

    /// Write a date value to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - Date value to write
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn write_date(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: NaiveDate,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        let year = value.year() as u16;
        let month = value.month() as u8;
        let day = value.day() as u8;
        let excel_date = ExcelDateTime::from_ymd(year, month, day)?;
        worksheet.write_datetime(row as u32, col as u16, excel_date)?;
        Ok(())
    }

    /// Write a datetime value to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - `DateTime` value to write
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn write_datetime(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: NaiveDateTime,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        let excel_date =
            ExcelDateTime::from_ymd(value.year() as u16, value.month() as u8, value.day() as u8)?;
        let excel_datetime = excel_date.and_hms(
            value.hour() as u16,
            value.minute() as u8,
            f64::from(value.second()),
        )?;
        worksheet.write_datetime(row as u32, col as u16, excel_datetime)?;
        Ok(())
    }

    /// Write a formula to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `formula` - Formula string (with or without leading =)
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_formula(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        formula: &str,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_formula(row as u32, col as u16, formula)?;
        Ok(())
    }

    /// Write a URL/hyperlink to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `url` - URL string
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_url(&mut self, sheet: usize, row: usize, col: usize, url: &str) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_url(row as u32, col as u16, url)?;
        Ok(())
    }

    /// Write a URL/hyperlink with custom text to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `url` - URL string
    /// * `text` - Display text for the hyperlink
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_url_with_text(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        url: &str,
        text: &str,
    ) -> Result<()> {
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_url_with_text(row as u32, col as u16, url, text)?;
        Ok(())
    }

    /// Write a string value with style to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - String value to write
    /// * `style` - Style to apply to the cell
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_string_with_style(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: &str,
        style: &Style,
    ) -> Result<()> {
        let format = Self::create_format_from_style(style);
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_string_with_format(row as u32, col as u16, value, &format)?;
        Ok(())
    }

    /// Write a number value with style to a cell
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `row` - Zero-based row index (max 1,048,576)
    /// * `col` - Zero-based column index (max 16,384)
    /// * `value` - Number value to write
    /// * `style` - Style to apply to the cell
    ///
    /// # Errors
    ///
    /// Returns error if cell cannot be written or if row/col exceed Excel limits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn write_number_with_style(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        value: f64,
        style: &Style,
    ) -> Result<()> {
        let format = Self::create_format_from_style(style);
        let worksheet = self.workbook.worksheet_from_index(sheet)?;
        worksheet.write_number_with_format(row as u32, col as u16, value, &format)?;
        Ok(())
    }

    /// Helper method to create a Format from a Style
    fn create_format_from_style(style: &Style) -> Format {
        let format = Format::new();
        style.apply_to_format(format)
    }

    /// Insert a line chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `LineChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_line_chart(&mut self, sheet: usize, chart: &LineChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Line);
        Self::configure_chart(&mut xl_chart, chart);
        self.insert_chart(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert a column chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `ColumnChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_column_chart(&mut self, sheet: usize, chart: &ColumnChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Column);
        Self::configure_column_chart(&mut xl_chart, chart);
        self.insert_chart_column(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert a bar chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `BarChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_bar_chart(&mut self, sheet: usize, chart: &BarChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Bar);
        Self::configure_bar_chart(&mut xl_chart, chart);
        self.insert_chart_bar(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert a pie chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `PieChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_pie_chart(&mut self, sheet: usize, chart: &PieChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Pie);
        Self::configure_pie_chart(&mut xl_chart, chart);
        self.insert_chart_pie(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert a scatter chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `ScatterChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_scatter_chart(&mut self, sheet: usize, chart: &ScatterChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Scatter);
        Self::configure_scatter_chart(&mut xl_chart, chart);
        self.insert_chart_scatter(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert an area chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `AreaChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_area_chart(&mut self, sheet: usize, chart: &AreaChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Area);
        Self::configure_area_chart(&mut xl_chart, chart);
        self.insert_chart_area(sheet, &xl_chart, chart)?;
        Ok(())
    }

    /// Insert a doughnut chart into a worksheet
    ///
    /// # Arguments
    ///
    /// * `sheet` - Zero-based sheet index
    /// * `chart` - `DoughnutChart` configuration
    ///
    /// # Errors
    ///
    /// Returns error if chart cannot be inserted.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_doughnut_chart(&mut self, sheet: usize, chart: &DoughnutChart) -> Result<()> {
        let mut xl_chart = Chart::new(ChartType::Doughnut);
        Self::configure_doughnut_chart(&mut xl_chart, chart);
        self.insert_chart_doughnut(sheet, &xl_chart, chart)?;
        Ok(())
    }

    // TODO: Add data validation integration when rust_xlsxwriter adds support

    /// Helper to configure line chart
    fn configure_chart(xl_chart: &mut Chart, chart: &LineChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if let Some(x_title) = chart.get_x_axis_title() {
            xl_chart.x_axis().set_name(x_title);
        }

        if let Some(y_title) = chart.get_y_axis_title() {
            xl_chart.y_axis().set_name(y_title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure column chart
    fn configure_column_chart(xl_chart: &mut Chart, chart: &ColumnChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if let Some(x_title) = chart.get_x_axis_title() {
            xl_chart.x_axis().set_name(x_title);
        }

        if let Some(y_title) = chart.get_y_axis_title() {
            xl_chart.y_axis().set_name(y_title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure bar chart
    fn configure_bar_chart(xl_chart: &mut Chart, chart: &BarChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if let Some(x_title) = chart.get_x_axis_title() {
            xl_chart.x_axis().set_name(x_title);
        }

        if let Some(y_title) = chart.get_y_axis_title() {
            xl_chart.y_axis().set_name(y_title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure pie chart
    fn configure_pie_chart(xl_chart: &mut Chart, chart: &PieChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure scatter chart
    fn configure_scatter_chart(xl_chart: &mut Chart, chart: &ScatterChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if let Some(x_title) = chart.get_x_axis_title() {
            xl_chart.x_axis().set_name(x_title);
        }

        if let Some(y_title) = chart.get_y_axis_title() {
            xl_chart.y_axis().set_name(y_title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure area chart
    fn configure_area_chart(xl_chart: &mut Chart, chart: &AreaChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if let Some(x_title) = chart.get_x_axis_title() {
            xl_chart.x_axis().set_name(x_title);
        }

        if let Some(y_title) = chart.get_y_axis_title() {
            xl_chart.y_axis().set_name(y_title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to configure doughnut chart
    fn configure_doughnut_chart(xl_chart: &mut Chart, chart: &DoughnutChart) {
        use crate::charts::Chart as ChartTrait;

        if let Some(title) = ChartTrait::title(chart) {
            xl_chart.title().set_name(title);
        }

        if !chart.is_legend_shown() {
            xl_chart.legend().set_hidden();
        }

        for series in chart.get_series() {
            let mut chart_series = xl_chart.add_series();
            if let Some(name) = series.get_name() {
                chart_series = chart_series.set_name(name);
            }
            if let Some(categories) = series.get_categories() {
                chart_series = chart_series.set_categories(categories);
            }
            chart_series.set_values(series.get_values());
        }
    }

    /// Helper to insert chart into worksheet
    fn insert_chart(&mut self, sheet: usize, chart: &Chart, line_chart: &LineChart) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(line_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert column chart into worksheet
    fn insert_chart_column(
        &mut self,
        sheet: usize,
        chart: &Chart,
        column_chart: &ColumnChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(column_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert bar chart into worksheet
    fn insert_chart_bar(
        &mut self,
        sheet: usize,
        chart: &Chart,
        bar_chart: &BarChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(bar_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert pie chart into worksheet
    fn insert_chart_pie(
        &mut self,
        sheet: usize,
        chart: &Chart,
        pie_chart: &PieChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(pie_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert scatter chart into worksheet
    fn insert_chart_scatter(
        &mut self,
        sheet: usize,
        chart: &Chart,
        scatter_chart: &ScatterChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(scatter_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert area chart into worksheet
    fn insert_chart_area(
        &mut self,
        sheet: usize,
        chart: &Chart,
        area_chart: &AreaChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(area_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Helper to insert doughnut chart into worksheet
    fn insert_chart_doughnut(
        &mut self,
        sheet: usize,
        chart: &Chart,
        doughnut_chart: &DoughnutChart,
    ) -> Result<()> {
        use crate::charts::Chart as ChartTrait;

        let worksheet = self.workbook.worksheet_from_index(sheet)?;

        if let Some(pos) = ChartTrait::position(doughnut_chart) {
            worksheet.insert_chart(pos.row, pos.col, chart)?;
        } else {
            worksheet.insert_chart(0, 0, chart)?;
        }

        Ok(())
    }

    /// Save the workbook to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the Excel file will be saved
    ///
    /// # Errors
    ///
    /// Returns `Error::FileWrite` if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xlsxpress::Writer;
    ///
    /// let mut writer = Writer::new();
    /// writer.save("output.xlsx")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn save<P: AsRef<Path>>(mut self, path: P) -> Result<()> {
        // GREEN phase: Minimal implementation
        self.workbook.save(path.as_ref())?;
        Ok(())
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// TDD RED: Test that we can create a new workbook
    #[test]
    fn test_create_workbook() {
        // Act: Create a new workbook
        let writer = Writer::new();

        // Assert: Should create successfully (compiles = success)
        assert!(std::mem::size_of_val(&writer) > 0);
    }

    /// TDD RED: Test adding a worksheet
    #[test]
    fn test_add_worksheet() {
        // Arrange: Create a new workbook
        let mut writer = Writer::new();

        // Act: Add a worksheet
        let result = writer.add_worksheet("Sheet1");

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to add worksheet: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a string cell
    #[test]
    fn test_write_string_cell() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write a string to cell A1
        let result = writer.write_string(0, 0, 0, "Hello");

        // Assert: Should succeed
        assert!(result.is_ok(), "Failed to write string: {:?}", result.err());
    }

    /// TDD RED: Test writing a number cell
    #[test]
    fn test_write_number_cell() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write a number to cell B1
        let result = writer.write_number(0, 0, 1, 42.0);

        // Assert: Should succeed
        assert!(result.is_ok(), "Failed to write number: {:?}", result.err());
    }

    /// TDD RED: Test saving workbook to file
    #[test]
    fn test_save_workbook() {
        // Arrange: Create workbook, add sheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Test").unwrap();

        // Act: Save to file
        let path = PathBuf::from("tests/fixtures/output_test.xlsx");
        let result = writer.save(&path);

        // Assert: Should save successfully
        assert!(result.is_ok(), "Failed to save: {:?}", result.err());

        // Verify file exists
        assert!(path.exists(), "Output file should exist");

        // Cleanup
        std::fs::remove_file(&path).ok();
    }

    /// TDD RED: Test that we can create multiple worksheets
    #[test]
    fn test_multiple_worksheets() {
        // Arrange: Create a new workbook
        let mut writer = Writer::new();

        // Act: Add multiple worksheets
        let result1 = writer.add_worksheet("Sheet1");
        let result2 = writer.add_worksheet("Sheet2");
        let result3 = writer.add_worksheet("Sheet3");

        // Assert: All should succeed
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }

    /// TDD RED: Test writing a boolean cell (true)
    #[test]
    fn test_write_boolean_true() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write boolean true to cell A1
        let result = writer.write_boolean(0, 0, 0, true);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write boolean: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a boolean cell (false)
    #[test]
    fn test_write_boolean_false() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write boolean false to cell B1
        let result = writer.write_boolean(0, 0, 1, false);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write boolean: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a date cell
    #[test]
    fn test_write_date() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write date 2024-01-15 to cell A1
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let result = writer.write_date(0, 0, 0, date);

        // Assert: Should succeed
        assert!(result.is_ok(), "Failed to write date: {:?}", result.err());
    }

    /// TDD RED: Test writing a datetime cell
    #[test]
    fn test_write_datetime() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write datetime to cell B1
        let datetime = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(14, 30, 45)
            .unwrap();
        let result = writer.write_datetime(0, 0, 1, datetime);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write datetime: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a formula cell
    #[test]
    fn test_write_formula() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write formula to cell C1
        let result = writer.write_formula(0, 0, 2, "=A1+B1");

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write formula: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a complex formula
    #[test]
    fn test_write_complex_formula() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write complex formula
        let result = writer.write_formula(0, 0, 2, "=SUM(A1:A10)");

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write complex formula: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a URL/hyperlink
    #[test]
    fn test_write_url() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write URL to cell A1
        let result = writer.write_url(0, 0, 0, "https://www.rust-lang.org");

        // Assert: Should succeed
        assert!(result.is_ok(), "Failed to write URL: {:?}", result.err());
    }

    /// TDD RED: Test writing a URL with custom text
    #[test]
    fn test_write_url_with_text() {
        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Act: Write URL with custom text to cell A1
        let result =
            writer.write_url_with_text(0, 0, 0, "https://www.rust-lang.org", "Rust Website");

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write URL with text: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a styled string cell
    #[test]
    fn test_write_styled_string() {
        use crate::styles::{Font, Style};

        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Create a style with bold font
        let style = Style::new().font(Font::new().bold(true).size(14.0));

        // Act: Write styled string to cell A1
        let result = writer.write_string_with_style(0, 0, 0, "Bold Text", &style);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write styled string: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing a styled number cell
    #[test]
    fn test_write_styled_number() {
        use crate::styles::{NumberFormat, Style};

        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Create a style with currency format
        let style = Style::new().number_format(NumberFormat::currency(2));

        // Act: Write styled number to cell B1
        let result = writer.write_number_with_style(0, 0, 1, 1234.56, &style);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write styled number: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test writing with complex style
    #[test]
    fn test_write_with_complex_style() {
        use crate::styles::{
            Alignment, Border, BorderStyle, Fill, Font, HorizontalAlignment, Style,
        };

        // Arrange: Create workbook and add worksheet
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();

        // Create a complex style
        let style = Style::new()
            .font(Font::new().bold(true).size(14.0).color("#FF0000"))
            .fill(Fill::solid("#FFFF00"))
            .border(Border::all(BorderStyle::Thin))
            .alignment(Alignment::new().horizontal(HorizontalAlignment::Center));

        // Act: Write styled string
        let result = writer.write_string_with_style(0, 0, 0, "Styled", &style);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to write complex styled cell: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a line chart
    #[test]
    fn test_insert_line_chart() {
        use crate::charts::{DataSeries, LineChart};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Month").unwrap();
        writer.write_string(0, 0, 1, "Sales").unwrap();
        writer.write_string(0, 1, 0, "Jan").unwrap();
        writer.write_number(0, 1, 1, 100.0).unwrap();

        // Create a line chart
        let chart = LineChart::new().title("Monthly Sales").add_series(
            DataSeries::new("Sheet1!$B$2:$B$2")
                .name("Sales")
                .categories("Sheet1!$A$2:$A$2"),
        );

        // Act: Insert chart
        let result = writer.insert_line_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert line chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a column chart
    #[test]
    fn test_insert_column_chart() {
        use crate::charts::{ColumnChart, DataSeries};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Quarter").unwrap();
        writer.write_string(0, 0, 1, "Revenue").unwrap();
        writer.write_string(0, 1, 0, "Q1").unwrap();
        writer.write_number(0, 1, 1, 1000.0).unwrap();

        // Create a column chart
        let chart = ColumnChart::new().title("Quarterly Revenue").add_series(
            DataSeries::new("Sheet1!$B$2:$B$2")
                .name("Revenue")
                .categories("Sheet1!$A$2:$A$2"),
        );

        // Act: Insert chart
        let result = writer.insert_column_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert column chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a bar chart
    #[test]
    fn test_insert_bar_chart() {
        use crate::charts::{BarChart, DataSeries};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Department").unwrap();
        writer.write_string(0, 0, 1, "Budget").unwrap();
        writer.write_string(0, 1, 0, "Sales").unwrap();
        writer.write_number(0, 1, 1, 50000.0).unwrap();

        // Create a bar chart
        let chart = BarChart::new().title("Department Budget").add_series(
            DataSeries::new("Sheet1!$B$2:$B$2")
                .name("Budget")
                .categories("Sheet1!$A$2:$A$2"),
        );

        // Act: Insert chart
        let result = writer.insert_bar_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert bar chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a pie chart
    #[test]
    fn test_insert_pie_chart() {
        use crate::charts::{DataSeries, PieChart};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Category").unwrap();
        writer.write_string(0, 0, 1, "Value").unwrap();
        writer.write_string(0, 1, 0, "Product A").unwrap();
        writer.write_number(0, 1, 1, 35.0).unwrap();
        writer.write_string(0, 2, 0, "Product B").unwrap();
        writer.write_number(0, 2, 1, 25.0).unwrap();

        // Create a pie chart
        let chart = PieChart::new().title("Market Share").add_series(
            DataSeries::new("Sheet1!$B$2:$B$3")
                .name("Products")
                .categories("Sheet1!$A$2:$A$3"),
        );

        // Act: Insert chart
        let result = writer.insert_pie_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert pie chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a scatter chart
    #[test]
    fn test_insert_scatter_chart() {
        use crate::charts::{DataSeries, ScatterChart};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "X Values").unwrap();
        writer.write_string(0, 0, 1, "Y Values").unwrap();
        writer.write_number(0, 1, 0, 1.0).unwrap();
        writer.write_number(0, 1, 1, 2.5).unwrap();
        writer.write_number(0, 2, 0, 2.0).unwrap();
        writer.write_number(0, 2, 1, 5.0).unwrap();

        // Create a scatter chart
        let chart = ScatterChart::new()
            .title("Correlation Plot")
            .x_axis_title("Independent")
            .y_axis_title("Dependent")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$3")
                    .name("Data Points")
                    .categories("Sheet1!$A$2:$A$3"),
            );

        // Act: Insert chart
        let result = writer.insert_scatter_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert scatter chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting an area chart
    #[test]
    fn test_insert_area_chart() {
        use crate::charts::{AreaChart, DataSeries};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Month").unwrap();
        writer.write_string(0, 0, 1, "Value").unwrap();
        writer.write_string(0, 1, 0, "Jan").unwrap();
        writer.write_number(0, 1, 1, 100.0).unwrap();
        writer.write_string(0, 2, 0, "Feb").unwrap();
        writer.write_number(0, 2, 1, 150.0).unwrap();

        // Create an area chart
        let chart = AreaChart::new()
            .title("Revenue Trend")
            .x_axis_title("Time")
            .y_axis_title("Amount")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$3")
                    .name("Revenue")
                    .categories("Sheet1!$A$2:$A$3"),
            );

        // Act: Insert chart
        let result = writer.insert_area_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert area chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting a doughnut chart
    #[test]
    fn test_insert_doughnut_chart() {
        use crate::charts::{DataSeries, DoughnutChart};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Category").unwrap();
        writer.write_string(0, 0, 1, "Value").unwrap();
        writer.write_string(0, 1, 0, "Item A").unwrap();
        writer.write_number(0, 1, 1, 40.0).unwrap();
        writer.write_string(0, 2, 0, "Item B").unwrap();
        writer.write_number(0, 2, 1, 30.0).unwrap();
        writer.write_string(0, 3, 0, "Item C").unwrap();
        writer.write_number(0, 3, 1, 30.0).unwrap();

        // Create a doughnut chart
        let chart = DoughnutChart::new()
            .title("Budget Distribution")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$4")
                    .name("Allocation")
                    .categories("Sheet1!$A$2:$A$4"),
            );

        // Act: Insert chart
        let result = writer.insert_doughnut_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert doughnut chart: {:?}",
            result.err()
        );
    }

    /// TDD RED: Test inserting chart with multiple series
    #[test]
    fn test_insert_chart_multiple_series() {
        use crate::charts::{DataSeries, LineChart};

        // Arrange: Create workbook, add worksheet, write data
        let mut writer = Writer::new();
        writer.add_worksheet("Sheet1").unwrap();
        writer.write_string(0, 0, 0, "Month").unwrap();
        writer.write_string(0, 0, 1, "Product A").unwrap();
        writer.write_string(0, 0, 2, "Product B").unwrap();
        writer.write_string(0, 1, 0, "Jan").unwrap();
        writer.write_number(0, 1, 1, 100.0).unwrap();
        writer.write_number(0, 1, 2, 150.0).unwrap();

        // Create a line chart with multiple series
        let chart = LineChart::new()
            .title("Product Comparison")
            .add_series(
                DataSeries::new("Sheet1!$B$2:$B$2")
                    .name("Product A")
                    .categories("Sheet1!$A$2:$A$2"),
            )
            .add_series(
                DataSeries::new("Sheet1!$C$2:$C$2")
                    .name("Product B")
                    .categories("Sheet1!$A$2:$A$2"),
            );

        // Act: Insert chart
        let result = writer.insert_line_chart(0, &chart);

        // Assert: Should succeed
        assert!(
            result.is_ok(),
            "Failed to insert chart with multiple series: {:?}",
            result.err()
        );
    }
}
