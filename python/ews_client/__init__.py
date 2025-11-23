"""EWS Client - Fast EWS implementation using Rust with Python bindings."""

from ._ews_client import EwsClient, __version__
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
