"""Type definitions for EWS client."""

from dataclasses import dataclass


@dataclass
class FolderInfo:
    """Information about an EWS folder."""

    folder_id: str
    parent_folder_id: str
    display_name: str
    folder_class: str | None
    total_count: int | None
    unread_count: int | None
    child_folder_count: int | None


@dataclass
class FolderHierarchySyncResult:
    """Result of folder hierarchy synchronization."""

    sync_state: str
    created_folders: list[FolderInfo]
    updated_folders: list[FolderInfo]
    deleted_folder_ids: list[str]
    well_known_folders: dict[str, str] | None


@dataclass
class SyncMessageInfo:
    """Detailed information about a synced message."""

    item_id: str
    is_read: bool | None
    internet_message_id: str | None
    date_time_sent: int | None
    from_: str | None
    subject: str | None
    has_attachments: bool | None
    size: int | None


@dataclass
class SyncMessagesResult:
    """Result of message synchronization."""

    created: list[str]
    updated: list[str]
    deleted: list[str]
    read_status_changed: list[tuple[str, bool]]
    sync_state: str
    includes_last_item: bool


@dataclass
class CreateMessageResult:
    """Result of creating a message."""

    item_id: str


# Legacy types for backward compatibility (deprecated)
@dataclass
class MessageInfo:
    """Information about an EWS message (deprecated, use SyncMessageInfo)."""

    id: str
    subject: str | None
    from_: str | None
    is_read: bool


@dataclass
class FolderSyncResult:
    """Result of folder synchronization (deprecated, use FolderHierarchySyncResult)."""

    folders: list[FolderInfo]
    sync_state: str


@dataclass
class MessageSyncResult:
    """Result of message synchronization (deprecated, use SyncMessagesResult)."""

    messages: list[MessageInfo]
    sync_state: str
