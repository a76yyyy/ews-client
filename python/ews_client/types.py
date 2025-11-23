"""Type definitions for EWS client."""

from dataclasses import dataclass


@dataclass
class FolderInfo:
    """Information about an EWS folder."""

    id: str
    parent_id: str | None
    display_name: str


@dataclass
class FolderSyncResult:
    """Result of folder synchronization."""

    folders: list[FolderInfo]
    sync_state: str


@dataclass
class MessageInfo:
    """Information about an EWS message."""

    id: str
    subject: str | None
    from_: str | None
    is_read: bool


@dataclass
class MessageSyncResult:
    """Result of message synchronization."""

    messages: list[MessageInfo]
    sync_state: str
