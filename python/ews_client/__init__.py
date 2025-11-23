"""EWS Client - Fast EWS implementation using Rust with Python bindings."""

from ._ews_client import EwsClient, __version__
from .types import FolderInfo, FolderSyncResult, MessageInfo, MessageSyncResult

__all__ = [
    "EwsClient",
    "__version__",
    "FolderInfo",
    "FolderSyncResult",
    "MessageInfo",
    "MessageSyncResult",
]
