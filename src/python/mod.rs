//! Python bindings module
//!
//! Provides Python bindings for the XlsXpress library using PyO3.

pub mod bindings;
// TODO: Fix compilation errors in these modules before re-enabling
// pub mod charts;
// pub mod styles;
// pub mod validation;

use pyo3::prelude::*;

// Re-export public types
// pub use charts::*;
// pub use styles::*;
// pub use validation::*;

/// XlsXpress Python module initialization
#[pymodule]
fn xlsxpress(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Core classes
    m.add_class::<bindings::PyWriter>()?;
    m.add_class::<bindings::PyReader>()?;
    m.add_class::<bindings::PyWorksheet>()?;

    // TODO: Re-enable after fixing compilation errors
    // // Chart classes
    // m.add_class::<charts::PyLineChart>()?;
    // m.add_class::<charts::PyColumnChart>()?;
    // m.add_class::<charts::PyBarChart>()?;
    // m.add_class::<charts::PyPieChart>()?;
    // m.add_class::<charts::PyScatterChart>()?;
    // m.add_class::<charts::PyAreaChart>()?;
    // m.add_class::<charts::PyDoughnutChart>()?;
    // m.add_class::<charts::PyDataSeries>()?;
    // m.add_class::<charts::PyChartPosition>()?;
    //
    // // Style classes
    // m.add_class::<styles::PyFont>()?;
    // m.add_class::<styles::PyFill>()?;
    // m.add_class::<styles::PyBorder>()?;
    // m.add_class::<styles::PyAlignment>()?;
    // m.add_class::<styles::PyNumberFormat>()?;
    // m.add_class::<styles::PyStyle>()?;
    //
    // // Validation classes
    // m.add_class::<validation::PyListValidation>()?;
    // m.add_class::<validation::PyNumberValidation>()?;
    // m.add_class::<validation::PyDateValidation>()?;
    // m.add_class::<validation::PyTextValidation>()?;
    // m.add_class::<validation::PyValidationError>()?;
    // m.add_class::<validation::PyValidationWarning>()?;
    // m.add_class::<validation::PyDataValidation>()?;

    // Module version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
