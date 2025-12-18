//! Core Python bindings for Writer and Reader

use crate::writer::Writer;
use calamine::DataType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*; // For is_empty() method

/// Python wrapper for Writer
#[pyclass(name = "Writer", unsendable)]
pub struct PyWriter {
    inner: Option<Writer>, // Option to allow taking ownership for save()
    sheet_count: usize,    // Track number of sheets
}

#[pymethods]
impl PyWriter {
    /// Create a new Excel writer
    #[new]
    fn new() -> Self {
        Self {
            inner: Some(Writer::new()),
            sheet_count: 0,
        }
    }

    /// Add a worksheet
    fn add_worksheet(&mut self, name: &str) -> PyResult<usize> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .add_worksheet(name)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let sheet_index = self.sheet_count;
        self.sheet_count += 1;
        Ok(sheet_index)
    }

    /// Write a string to a cell
    fn write_string(&mut self, sheet: usize, row: usize, col: usize, value: &str) -> PyResult<()> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .write_string(sheet, row, col, value)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Write a number to a cell
    fn write_number(&mut self, sheet: usize, row: usize, col: usize, value: f64) -> PyResult<()> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .write_number(sheet, row, col, value)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Write a boolean to a cell
    fn write_boolean(&mut self, sheet: usize, row: usize, col: usize, value: bool) -> PyResult<()> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .write_boolean(sheet, row, col, value)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Write a formula to a cell
    fn write_formula(
        &mut self,
        sheet: usize,
        row: usize,
        col: usize,
        formula: &str,
    ) -> PyResult<()> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .write_formula(sheet, row, col, formula)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Write a URL to a cell
    fn write_url(&mut self, sheet: usize, row: usize, col: usize, url: &str) -> PyResult<()> {
        let writer = self
            .inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;

        writer
            .write_url(sheet, row, col, url)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // TODO: Re-enable styled write methods after fixing styles module
    // /// Write a string with style
    // fn write_string_with_style(
    //     &mut self,
    //     sheet: usize,
    //     row: usize,
    //     col: usize,
    //     value: &str,
    //     style: &PyStyle,
    // ) -> PyResult<()> {
    //     let writer = self.inner.as_mut()
    //         .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;
    //
    //     writer.write_string_with_style(sheet, row, col, value, &style.inner)
    //         .map_err(|e| PyValueError::new_err(e.to_string()))
    // }

    // /// Write a number with style
    // fn write_number_with_style(
    //     &mut self,
    //     sheet: usize,
    //     row: usize,
    //     col: usize,
    //     value: f64,
    //     style: &PyStyle,
    // ) -> PyResult<()> {
    //     let writer = self.inner.as_mut()
    //         .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;
    //
    //     writer.write_number_with_style(sheet, row, col, value, &style.inner)
    //         .map_err(|e| PyValueError::new_err(e.to_string()))
    // }

    // TODO: Re-enable chart insertion methods after fixing charts module
    // /// Insert a line chart
    // fn insert_line_chart(&mut self, sheet: usize, chart: &PyLineChart) -> PyResult<()> {
    //     let writer = self.inner.as_mut()
    //         .ok_or_else(|| PyValueError::new_err("Writer has been consumed by save()"))?;
    //
    //     writer.insert_line_chart(sheet, &chart.inner)
    //         .map_err(|e| PyValueError::new_err(e.to_string()))
    // }

    /// Save the workbook to a file
    fn save(&mut self, path: &str) -> PyResult<()> {
        let writer = self
            .inner
            .take()
            .ok_or_else(|| PyValueError::new_err("Writer has already been saved"))?;

        writer
            .save(path)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// Python wrapper for Reader
#[pyclass(name = "Reader")]
pub struct PyReader {
    inner: crate::reader::Reader,
}

#[pymethods]
impl PyReader {
    /// Open an Excel file for reading
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let reader =
            crate::reader::Reader::open(path).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: reader })
    }

    /// Get list of sheet names
    fn sheet_names(&self) -> Vec<String> {
        self.inner.sheet_names()
    }

    /// Get a worksheet by name and return its data as a `PyWorksheet`
    fn worksheet(&mut self, name: &str) -> PyResult<PyWorksheet> {
        let range = self
            .inner
            .worksheet_range(name)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyWorksheet { range })
    }
}

/// Python wrapper for a worksheet range
#[pyclass(name = "Worksheet")]
pub(crate) struct PyWorksheet {
    range: calamine::Range<calamine::Data>,
}

#[pymethods]
impl PyWorksheet {
    /// Get cell value as string
    fn get_value(&self, row: usize, col: usize) -> Option<String> {
        self.range.get((row, col)).and_then(|cell| {
            if cell.is_empty() {
                None
            } else {
                Some(cell.to_string())
            }
        })
    }

    /// Get cell value as number
    fn get_number(&self, row: usize, col: usize) -> Option<f64> {
        self.range.get((row, col)).and_then(DataType::get_float)
    }

    /// Get dimensions (rows, cols)
    fn dimensions(&self) -> (usize, usize) {
        self.range.get_size()
    }

    /// Convert worksheet to list of lists
    fn to_list(&self) -> Vec<Vec<Option<String>>> {
        let (rows, cols) = self.range.get_size();
        let mut result = Vec::with_capacity(rows);

        for row in 0..rows {
            let mut row_data = Vec::with_capacity(cols);
            for col in 0..cols {
                let value = self.range.get((row, col)).and_then(|cell| {
                    if cell.is_empty() {
                        None
                    } else {
                        Some(cell.to_string())
                    }
                });
                row_data.push(value);
            }
            result.push(row_data);
        }
        result
    }

    /// Iterate over rows
    fn __iter__(slf: PyRef<'_, Self>) -> PyWorksheetIterator {
        PyWorksheetIterator {
            worksheet: slf.into(),
            current_row: 0,
        }
    }
}

/// Iterator for worksheet rows
#[pyclass]
struct PyWorksheetIterator {
    worksheet: Py<PyWorksheet>,
    current_row: usize,
}

#[pymethods]
impl PyWorksheetIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> Option<Vec<Option<String>>> {
        let current_row = slf.current_row;

        // Scope the borrow to just this block
        let (_rows, _cols, row_data) = {
            let worksheet = slf.worksheet.borrow(py);
            let (rows, cols) = worksheet.range.get_size();

            if current_row >= rows {
                return None;
            }

            let mut row_data = Vec::with_capacity(cols);
            for col in 0..cols {
                let value = worksheet.range.get((current_row, col)).and_then(|cell| {
                    if cell.is_empty() {
                        None
                    } else {
                        Some(cell.to_string())
                    }
                });
                row_data.push(value);
            }

            (rows, cols, row_data)
        }; // worksheet borrow ends here

        slf.current_row += 1;
        Some(row_data)
    }
}
