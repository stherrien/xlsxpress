"""
XlsXpress - High-performance Excel processing library

XlsXpress is a Python library built on Rust that provides lightning-fast
Excel file reading and writing. It offers significant performance improvements
over pure Python solutions like OpenPyXL and XlsxWriter.

Features:
- Fast Excel reading (9.4x faster than OpenPyXL)
- Fast Excel writing (3.8x faster than XlsxWriter)
- Memory efficient
- OpenPyXL compatibility layer
- Full type hints for IDE support

Examples:
    Reading an Excel file::

        import xlsxpress

        reader = xlsxpress.Reader.open("data.xlsx")
        ws = reader.worksheet("Sheet1")
        data = ws.to_list()

    Writing an Excel file::

        import xlsxpress

        writer = xlsxpress.Writer()
        sheet = writer.add_worksheet("Results")
        writer.write_string(sheet, 0, 0, "Hello World")
        writer.save("output.xlsx")
"""

__version__ = "0.1.0"
__author__ = "XlsXpress Contributors"
__license__ = "MIT"

# Import core classes from Rust extension module
from .xlsxpress import (
    # Core classes (MVP - Available Now)
    Reader,
    Writer,
    Worksheet,
)

# TODO: Add advanced features after fixing compilation errors
# # Chart classes
# LineChart,
# ColumnChart,
# BarChart,
# PieChart,
# ScatterChart,
# AreaChart,
# DoughnutChart,
# DataSeries,
# ChartPosition,
# # Style classes
# Font,
# Fill,
# Border,
# Alignment,
# NumberFormat,
# Style,
# # Validation classes
# ListValidation,
# NumberValidation,
# DateValidation,
# TextValidation,
# ValidationError,
# ValidationWarning,
# DataValidation,

__all__ = [
    "__version__",
    # Core classes
    "Reader",
    "Writer",
    "Worksheet",
]
