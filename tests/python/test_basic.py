"""
Basic integration tests for XlsXpress Python API

Following TDD principles:
1. Write test first (RED)
2. Implement minimal code (GREEN)
3. Refactor (BLUE)
"""

import pytest


def test_import_xlsxpress():
    """Test that xlsxpress can be imported."""
    import xlsxpress

    assert xlsxpress.__version__ == "0.1.0"


def test_import_compat_module():
    """Test that compatibility module can be imported."""
    from xlsxpress import compat

    assert compat is not None


# TODO: Add more tests as features are implemented following TDD
# Phase 2: Reader tests
# Phase 3: Writer tests
# Phase 4: Compatibility tests
