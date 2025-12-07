"""EWS Client - Fast EWS implementation using Rust with Python bindings."""

from ._ews_client import (
    BaseEWSError,
    CreateMessageResult,
    EWSAuthenticationError,
    EwsClient,
    EWSHTTPError,
    EWSMissingIdError,
    EWSProcessingError,
    EWSProtocolError,
    EWSResponseError,
    EWSSerializationError,
    FolderHierarchySyncResult,
    FolderInfo,
    SyncMessageInfo,
    SyncMessagesResult,
    __version__,
)

__all__ = [
    "EwsClient",
    "__version__",
    # Error types
    "BaseEWSError",
    "EWSAuthenticationError",
    "EWSHTTPError",
    "EWSProtocolError",
    "EWSResponseError",
    "EWSProcessingError",
    "EWSMissingIdError",
    "EWSSerializationError",
    # Data types
    "FolderInfo",
    "FolderHierarchySyncResult",
    "SyncMessageInfo",
    "SyncMessagesResult",
    "CreateMessageResult",
]
