//! Excel chart module
//!
//! Provides types for creating charts in Excel worksheets including line,
//! column, bar, pie, scatter, area, and doughnut charts.

pub mod area;
pub mod bar;
pub mod chart;
pub mod column;
pub mod doughnut;
pub mod line;
pub mod pie;
pub mod scatter;

// Re-export for convenience
pub use area::AreaChart;
pub use bar::BarChart;
pub use chart::{Chart, ChartPosition, ChartType};
pub use column::ColumnChart;
pub use doughnut::DoughnutChart;
pub use line::{DataSeries, LineChart};
pub use pie::PieChart;
pub use scatter::ScatterChart;
