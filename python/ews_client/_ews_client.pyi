"""Type stubs for ews_client."""

class EwsClient:
    """EWS client for Exchange Web Services."""

    def __init__(self, endpoint: str, username: str, password: str) -> None:
        """
        Create a new EWS client.

        Args:
            endpoint: EWS endpoint URL
            username: Username for authentication
            password: Password for authentication
        """
        ...

__version__: str
