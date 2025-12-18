//! Python bindings for chart types

use crate::charts::{
    AreaChart, BarChart, ChartPosition, ColumnChart, DataSeries, DoughnutChart, LineChart,
    PieChart, ScatterChart,
};
use pyo3::prelude::*;

/// Python wrapper for DataSeries
#[pyclass(name = "DataSeries")]
#[derive(Clone)]
pub struct PyDataSeries {
    pub(crate) inner: DataSeries,
}

#[pymethods]
impl PyDataSeries {
    /// Create a new data series
    #[new]
    fn new(values: &str) -> Self {
        Self {
            inner: DataSeries::new(values),
        }
    }

    /// Set series name
    fn name(mut slf: PyRefMut<'_, Self>, name: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).name(name);
        slf
    }

    /// Set categories range (X-axis)
    fn categories(mut slf: PyRefMut<'_, Self>, categories: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).categories(categories);
        slf
    }
}

/// Python wrapper for ChartPosition
#[pyclass(name = "ChartPosition")]
#[derive(Clone)]
pub struct PyChartPosition {
    pub(crate) inner: ChartPosition,
}

#[pymethods]
impl PyChartPosition {
    /// Create a new chart position
    #[new]
    fn new(row: u32, col: u16) -> Self {
        Self {
            inner: ChartPosition::new(row, col),
        }
    }

    /// Set chart width in pixels
    fn width(mut slf: PyRefMut<'_, Self>, width: u32) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).width(width);
        slf
    }

    /// Set chart height in pixels
    fn height(mut slf: PyRefMut<'_, Self>, height: u32) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).height(height);
        slf
    }
}

/// Python wrapper for LineChart
#[pyclass(name = "LineChart")]
pub struct PyLineChart {
    pub(crate) inner: LineChart,
}

#[pymethods]
impl PyLineChart {
    /// Create a new line chart
    #[new]
    fn new() -> Self {
        Self {
            inner: LineChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set X-axis title
    fn x_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).x_axis_title(title);
        slf
    }

    /// Set Y-axis title
    fn y_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).y_axis_title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }
}

/// Python wrapper for ColumnChart
#[pyclass(name = "ColumnChart")]
pub struct PyColumnChart {
    pub(crate) inner: ColumnChart,
}

#[pymethods]
impl PyColumnChart {
    /// Create a new column chart
    #[new]
    fn new() -> Self {
        Self {
            inner: ColumnChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set X-axis title
    fn x_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).x_axis_title(title);
        slf
    }

    /// Set Y-axis title
    fn y_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).y_axis_title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }

    /// Set whether columns should be stacked
    fn stacked(mut slf: PyRefMut<'_, Self>, stacked: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).stacked(stacked);
        slf
    }
}

/// Python wrapper for BarChart
#[pyclass(name = "BarChart")]
pub struct PyBarChart {
    pub(crate) inner: BarChart,
}

#[pymethods]
impl PyBarChart {
    /// Create a new bar chart
    #[new]
    fn new() -> Self {
        Self {
            inner: BarChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set X-axis title
    fn x_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).x_axis_title(title);
        slf
    }

    /// Set Y-axis title
    fn y_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).y_axis_title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }

    /// Set whether bars should be stacked
    fn stacked(mut slf: PyRefMut<'_, Self>, stacked: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).stacked(stacked);
        slf
    }
}

/// Python wrapper for PieChart
#[pyclass(name = "PieChart")]
pub struct PyPieChart {
    pub(crate) inner: PieChart,
}

#[pymethods]
impl PyPieChart {
    /// Create a new pie chart
    #[new]
    fn new() -> Self {
        Self {
            inner: PieChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }
}

/// Python wrapper for ScatterChart
#[pyclass(name = "ScatterChart")]
pub struct PyScatterChart {
    pub(crate) inner: ScatterChart,
}

#[pymethods]
impl PyScatterChart {
    /// Create a new scatter chart
    #[new]
    fn new() -> Self {
        Self {
            inner: ScatterChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set X-axis title
    fn x_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).x_axis_title(title);
        slf
    }

    /// Set Y-axis title
    fn y_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).y_axis_title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }
}

/// Python wrapper for AreaChart
#[pyclass(name = "AreaChart")]
pub struct PyAreaChart {
    pub(crate) inner: AreaChart,
}

#[pymethods]
impl PyAreaChart {
    /// Create a new area chart
    #[new]
    fn new() -> Self {
        Self {
            inner: AreaChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Set X-axis title
    fn x_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).x_axis_title(title);
        slf
    }

    /// Set Y-axis title
    fn y_axis_title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).y_axis_title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }

    /// Set whether areas should be stacked
    fn stacked(mut slf: PyRefMut<'_, Self>, stacked: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).stacked(stacked);
        slf
    }
}

/// Python wrapper for DoughnutChart
#[pyclass(name = "DoughnutChart")]
pub struct PyDoughnutChart {
    pub(crate) inner: DoughnutChart,
}

#[pymethods]
impl PyDoughnutChart {
    /// Create a new doughnut chart
    #[new]
    fn new() -> Self {
        Self {
            inner: DoughnutChart::new(),
        }
    }

    /// Set chart title
    fn title(mut slf: PyRefMut<'_, Self>, title: &str) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).title(title);
        slf
    }

    /// Add a data series to the chart
    fn add_series(mut slf: PyRefMut<'_, Self>, series: &PyDataSeries) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).add_series(series.inner.clone());
        slf
    }

    /// Set chart position on worksheet
    fn position(mut slf: PyRefMut<'_, Self>, position: &PyChartPosition) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).position(position.inner.clone());
        slf
    }

    /// Set whether to show legend
    fn show_legend(mut slf: PyRefMut<'_, Self>, show: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).show_legend(show);
        slf
    }
}
