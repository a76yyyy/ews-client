"""Tests for EWS client."""

from ews_client import EwsClient


def test_client_creation(mock_ews_endpoint, mock_credentials):
    """Test client creation."""
    client = EwsClient(
        endpoint=mock_ews_endpoint,
        username=mock_credentials["username"],
        password=mock_credentials["password"],
    )
    assert client is not None
