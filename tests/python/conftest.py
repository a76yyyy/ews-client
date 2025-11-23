"""Pytest configuration and fixtures."""

import pytest


@pytest.fixture
def mock_ews_endpoint():
    """Mock EWS endpoint URL."""
    return "https://outlook.office365.com/EWS/Exchange.asmx"


@pytest.fixture
def mock_credentials():
    """Mock credentials."""
    return {
        "username": "test@example.com",
        "password": "test_password",
    }
