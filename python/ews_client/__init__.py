"""EWS Client - Fast EWS implementation using Rust with Python bindings."""

from ._ews_client import (
    BaseEWSError,
    EWSAuthenticationError,
    EwsClient,
    EWSHTTPError,
    EWSMissingIdError,
    EWSProcessingError,
    EWSProtocolError,
    EWSResponseError,
    EWSSerializationError,
    __version__,
)
from .types import (
    CreateMessageResult,
    FolderHierarchySyncResult,
    FolderInfo,
    # Legacy types (deprecated)
    FolderSyncResult,
    MessageInfo,
    MessageSyncResult,
    SyncMessageInfo,
    SyncMessagesResult,
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
    # New types
    "FolderInfo",
    "FolderHierarchySyncResult",
    "SyncMessageInfo",
    "SyncMessagesResult",
    "CreateMessageResult",
    # Legacy types (deprecated)
    "FolderSyncResult",
    "MessageInfo",
    "MessageSyncResult",
]
