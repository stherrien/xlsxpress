//! Coordinate utilities for `OpenPyXL` compatibility
//!
//! Provides functions to convert between Excel A1 notation and row/column indices.
//! Follows `OpenPyXL`'s coordinate system where rows and columns are 1-indexed.

use crate::error::{Error, Result};

/// Convert column letter(s) to column number (1-indexed)
///
/// # Arguments
///
/// * `col` - Column letter(s) like "A", "B", "AA", "XFD"
///
/// # Returns
///
/// Column number where A=1, B=2, ..., Z=26, AA=27, etc.
///
/// # Errors
///
/// Returns error if column letter is invalid.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(column_index_from_string("A")?, 1);
/// assert_eq!(column_index_from_string("Z")?, 26);
/// assert_eq!(column_index_from_string("AA")?, 27);
/// ```
pub fn column_index_from_string(col: &str) -> Result<usize> {
    if col.is_empty() {
        return Err(Error::invalid_cell_reference(col));
    }

    let mut result = 0;
    for c in col.chars() {
        if !c.is_ascii_uppercase() {
            return Err(Error::invalid_cell_reference(col));
        }
        result = result * 26 + (c as usize - 'A' as usize + 1);
    }

    Ok(result)
}

/// Convert column number to column letter(s) (1-indexed)
///
/// # Arguments
///
/// * `col` - Column number where 1=A, 2=B, ..., 26=Z, 27=AA, etc.
///
/// # Returns
///
/// Column letter(s) like "A", "B", "AA", "XFD"
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(get_column_letter(1), "A");
/// assert_eq!(get_column_letter(26), "Z");
/// assert_eq!(get_column_letter(27), "AA");
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_column_letter(mut col: usize) -> String {
    let mut result = String::new();
    while col > 0 {
        col -= 1;
        result.insert(0, (b'A' + (col % 26) as u8) as char);
        col /= 26;
    }
    result
}

/// Parse cell coordinate like "A1" into (row, col) tuple
///
/// # Arguments
///
/// * `coord` - Cell coordinate like "A1", "B2", "AA100"
///
/// # Returns
///
/// Tuple of (row, col) where both are 1-indexed
///
/// # Errors
///
/// Returns error if coordinate is invalid.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(coordinate_from_string("A1")?, (1, 1));
/// assert_eq!(coordinate_from_string("B2")?, (2, 2));
/// assert_eq!(coordinate_from_string("AA100")?, (100, 27));
/// ```
pub fn coordinate_from_string(coord: &str) -> Result<(usize, usize)> {
    let mut col_part = String::new();
    let mut row_part = String::new();
    let mut in_row = false;

    for c in coord.chars() {
        if c.is_ascii_uppercase() {
            if in_row {
                return Err(Error::invalid_cell_reference(coord));
            }
            col_part.push(c);
        } else if c.is_ascii_digit() {
            in_row = true;
            row_part.push(c);
        } else {
            return Err(Error::invalid_cell_reference(coord));
        }
    }

    if col_part.is_empty() || row_part.is_empty() {
        return Err(Error::invalid_cell_reference(coord));
    }

    let col = column_index_from_string(&col_part)?;
    let row = row_part
        .parse::<usize>()
        .map_err(|_| Error::invalid_cell_reference(coord))?;

    if row == 0 {
        return Err(Error::invalid_cell_reference(coord));
    }

    Ok((row, col))
}

/// Convert (row, col) to cell coordinate like "A1"
///
/// # Arguments
///
/// * `row` - Row number (1-indexed)
/// * `col` - Column number (1-indexed)
///
/// # Returns
///
/// Cell coordinate like "A1", "B2", "AA100"
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(get_column_letter(1, 1), "A1");
/// assert_eq!(get_column_letter(2, 2), "B2");
/// assert_eq!(get_column_letter(100, 27), "AA100");
/// ```
#[must_use]
pub fn coordinate_to_string(row: usize, col: usize) -> String {
    format!("{}{}", get_column_letter(col), row)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test column letter to index conversion
    #[test]
    fn test_column_index_from_string() {
        // Single letter columns
        assert_eq!(column_index_from_string("A").unwrap(), 1);
        assert_eq!(column_index_from_string("B").unwrap(), 2);
        assert_eq!(column_index_from_string("Z").unwrap(), 26);

        // Double letter columns
        assert_eq!(column_index_from_string("AA").unwrap(), 27);
        assert_eq!(column_index_from_string("AB").unwrap(), 28);
        assert_eq!(column_index_from_string("AZ").unwrap(), 52);
        assert_eq!(column_index_from_string("BA").unwrap(), 53);

        // Triple letter columns
        assert_eq!(column_index_from_string("AAA").unwrap(), 703);
    }

    /// TDD RED: Test invalid column letters
    #[test]
    fn test_column_index_invalid() {
        assert!(column_index_from_string("").is_err());
        assert!(column_index_from_string("a").is_err());
        assert!(column_index_from_string("1").is_err());
        assert!(column_index_from_string("A1").is_err());
    }

    /// TDD RED: Test column index to letter conversion
    #[test]
    fn test_get_column_letter() {
        // Single letter columns
        assert_eq!(get_column_letter(1), "A");
        assert_eq!(get_column_letter(2), "B");
        assert_eq!(get_column_letter(26), "Z");

        // Double letter columns
        assert_eq!(get_column_letter(27), "AA");
        assert_eq!(get_column_letter(28), "AB");
        assert_eq!(get_column_letter(52), "AZ");
        assert_eq!(get_column_letter(53), "BA");

        // Triple letter columns
        assert_eq!(get_column_letter(703), "AAA");
    }

    /// TDD RED: Test coordinate parsing
    #[test]
    fn test_coordinate_from_string() {
        assert_eq!(coordinate_from_string("A1").unwrap(), (1, 1));
        assert_eq!(coordinate_from_string("B2").unwrap(), (2, 2));
        assert_eq!(coordinate_from_string("Z26").unwrap(), (26, 26));
        assert_eq!(coordinate_from_string("AA100").unwrap(), (100, 27));
        assert_eq!(coordinate_from_string("XFD1048576").unwrap(), (1048576, 16384));
    }

    /// TDD RED: Test invalid coordinates
    #[test]
    fn test_coordinate_invalid() {
        assert!(coordinate_from_string("").is_err());
        assert!(coordinate_from_string("A").is_err());
        assert!(coordinate_from_string("1").is_err());
        assert!(coordinate_from_string("A0").is_err());
        assert!(coordinate_from_string("1A").is_err());
        assert!(coordinate_from_string("a1").is_err());
    }

    /// TDD RED: Test coordinate to string conversion
    #[test]
    fn test_coordinate_to_string() {
        assert_eq!(coordinate_to_string(1, 1), "A1");
        assert_eq!(coordinate_to_string(2, 2), "B2");
        assert_eq!(coordinate_to_string(26, 26), "Z26");
        assert_eq!(coordinate_to_string(100, 27), "AA100");
        assert_eq!(coordinate_to_string(1048576, 16384), "XFD1048576");
    }

    /// TDD RED: Test round-trip conversion
    #[test]
    fn test_coordinate_roundtrip() {
        let coords = vec!["A1", "B2", "Z26", "AA100", "XFD1048576"];
        for coord in coords {
            let (row, col) = coordinate_from_string(coord).unwrap();
            let result = coordinate_to_string(row, col);
            assert_eq!(result, coord);
        }
    }
}
