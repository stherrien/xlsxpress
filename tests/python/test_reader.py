"""
Smoke tests for XlsXpress Reader Python bindings
"""

import pytest
import xlsxpress


def test_reader_open():
    """Test that Reader can open an Excel file"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    assert reader is not None


def test_reader_sheet_names():
    """Test getting sheet names"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    names = reader.sheet_names()

    assert isinstance(names, list)
    assert len(names) > 0
    assert "Sheet1" in names


def test_reader_worksheet():
    """Test getting a worksheet"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    assert worksheet is not None


def test_worksheet_dimensions():
    """Test getting worksheet dimensions"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    rows, cols = worksheet.dimensions()
    assert rows >= 2
    assert cols >= 2


def test_worksheet_get_value():
    """Test reading cell values"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    # Test reading string cell (A1 should be "Hello")
    value = worksheet.get_value(0, 0)
    assert value == "Hello"


def test_worksheet_get_number():
    """Test reading numeric values"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    # Test reading number cell (B1 should be 42)
    value = worksheet.get_number(0, 1)
    assert value == 42.0


def test_worksheet_to_list():
    """Test converting worksheet to list"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    data = worksheet.to_list()

    assert isinstance(data, list)
    assert len(data) > 0
    assert isinstance(data[0], list)

    # Check first cell
    assert data[0][0] == "Hello"
    assert data[0][1] == "42"


def test_worksheet_iteration():
    """Test iterating over worksheet rows"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")
    worksheet = reader.worksheet("Sheet1")

    rows = list(worksheet)

    assert isinstance(rows, list)
    assert len(rows) > 0
    assert isinstance(rows[0], list)

    # Check first row
    assert rows[0][0] == "Hello"


def test_reader_nonexistent_file():
    """Test that opening nonexistent file raises error"""
    with pytest.raises(ValueError):
        xlsxpress.Reader.open("nonexistent.xlsx")


def test_reader_nonexistent_sheet():
    """Test that accessing nonexistent sheet raises error"""
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")

    with pytest.raises(ValueError):
        reader.worksheet("NonExistentSheet")


def test_complete_workflow():
    """Test complete read workflow"""
    # Open file
    reader = xlsxpress.Reader.open("tests/fixtures/test.xlsx")

    # List sheets
    sheets = reader.sheet_names()
    assert "Sheet1" in sheets

    # Get worksheet
    ws = reader.worksheet("Sheet1")

    # Get dimensions
    rows, cols = ws.dimensions()
    assert rows > 0 and cols > 0

    # Read specific cells
    assert ws.get_value(0, 0) == "Hello"
    assert ws.get_number(0, 1) == 42.0

    # Convert to list
    data = ws.to_list()
    assert len(data) == rows
    assert len(data[0]) == cols

    # Iterate over rows
    row_count = 0
    for row in ws:
        row_count += 1
        assert isinstance(row, list)

    assert row_count == rows

    print("✓ All Reader workflow tests passed!")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
