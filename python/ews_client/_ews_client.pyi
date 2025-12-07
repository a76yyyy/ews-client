"""Type stubs for ews_client.

This module contains all EWS client types and the main EwsClient class.
All types are implemented in Rust via PyO3 and exported to Python.
"""

# Exception classes (defined in Rust via create_exception! macro)
class BaseEWSError(Exception):
    """Base exception for all EWS client errors.

    All EWS-specific exceptions inherit from this class, allowing
    users to catch all EWS errors with a single except clause.

    Example:
        try:
            await client.sync_folder_hierarchy()
        except BaseEWSError as e:
            print(f"EWS error: {e}")
    """

    pass

class EWSAuthenticationError(BaseEWSError):
    """Authentication failure (401, invalid credentials, etc.).

    Raised when:
    - Invalid username or password
    - Credentials have expired
    - Server returns 401 Unauthorized
    """

    pass

class EWSHTTPError(BaseEWSError):
    """HTTP transport error (network, connection, etc.).

    Raised when:
    - Network connection fails
    - DNS resolution fails
    - SSL/TLS certificate validation fails
    - HTTP request timeout
    """

    pass

class EWSProtocolError(BaseEWSError):
    """EWS protocol error (SOAP parsing, XML issues, etc.).

    Raised when:
    - SOAP envelope parsing fails
    - XML deserialization fails
    - Invalid EWS protocol response
    """

    pass

class EWSResponseError(BaseEWSError):
    """EWS response contained an error code.

    Raised when:
    - Exchange server returns an error response
    - Item not found
    - Folder not found
    - Invalid operation for the current state
    """

    pass

class EWSProcessingError(BaseEWSError):
    """Error processing response data (validation, unexpected format, etc.).

    Raised when:
    - Response data validation fails
    - Unexpected response structure
    - Missing required fields in response
    - Data transformation fails
    """

    pass

class EWSMissingIdError(BaseEWSError):
    """Missing required ID in response from Exchange.

    Raised when:
    - Exchange server response doesn't include expected ID
    - Folder ID missing from response
    - Item ID missing from response
    """

    pass

class EWSSerializationError(BaseEWSError):
    """JSON serialization/deserialization error.

    Raised when:
    - JSON encoding fails
    - JSON decoding fails
    - Invalid JSON format
    """

    pass

# Data types (implemented as #[pyclass] in Rust)

class FolderInfo:
    """Information about an EWS folder.

    Represents a single folder in the EWS folder hierarchy.
    All fields are read-only.
    """

    folder_id: str
    """The EWS folder ID."""

    parent_folder_id: str
    """The parent folder ID."""

    display_name: str
    """The display name of the folder."""

    folder_class: str | None
    """The folder class (e.g., "IPF.Note" for mail folders)."""

    total_count: int | None
    """Total number of items in the folder."""

    unread_count: int | None
    """Number of unread items in the folder."""

    child_folder_count: int | None
    """Number of child folders."""

class FolderHierarchySyncResult:
    """Result of folder hierarchy synchronization.

    Contains the changes to the folder hierarchy since the last sync.
    All fields are read-only.
    """

    sync_state: str
    """The new sync state token to use for the next sync."""

    created_folders: list[FolderInfo]
    """Folders that were created since the last sync."""

    updated_folders: list[FolderInfo]
    """Folders that were updated since the last sync."""

    deleted_folder_ids: list[str]
    """IDs of folders that were deleted since the last sync."""

    well_known_folders: dict[str, str] | None
    """Map of well-known folder IDs to their distinguished names
    (e.g., "inbox", "deleteditems", "drafts", etc.)."""

class SyncMessageInfo:
    """Detailed information about a synced message.

    Contains detailed information about a message from a sync operation.
    All fields are read-only.
    """

    item_id: str
    """The EWS item ID of the message."""

    is_read: bool | None
    """Whether the message has been read."""

    internet_message_id: str | None
    """The Internet message ID (RFC 2822 Message-ID header)."""

    date_time_sent: int | None
    """The date and time the message was sent (Unix timestamp in seconds)."""

    from_: str | None
    """The sender's email address."""

    subject: str | None
    """The message subject."""

    has_attachments: bool | None
    """Whether the message has attachments."""

    size: int | None
    """The size of the message in bytes."""

class SyncMessagesResult:
    """Result of message synchronization.

    Contains the changes to messages in a folder since the last sync.
    All fields are read-only.
    """

    created: list[str]
    """Message IDs that were created."""

    updated: list[str]
    """Message IDs that were updated."""

    deleted: list[str]
    """Message IDs that were deleted."""

    read_status_changed: list[tuple[str, bool]]
    """Message IDs with read status changed (`item_id`, `is_read`)."""

    sync_state: str
    """The new sync state token for the next sync."""

    includes_last_item: bool
    """Whether there are more changes to fetch."""

class CreateMessageResult:
    """Result of creating a message.

    Contains the ID of the newly created message.
    All fields are read-only.
    """

    item_id: str
    """The EWS ID of the newly created message."""

class EwsClient:
    """EWS client for Exchange Web Services."""

    def __init__(self, endpoint: str, username: str, password: str) -> None:
        """
        Create a new EWS client.

        Args:
            endpoint: EWS endpoint URL
            username: Username for authentication
            password: Password for authentication

        Raises:
            ValueError: If endpoint URL is invalid
            RuntimeError: If client creation fails
        """
        ...

    async def check_connectivity(self) -> None:
        """
        Test connection and authentication to the EWS server.

        Raises:
            Exception: If connection or authentication fails
        """
        ...

    async def sync_folder_hierarchy(self, sync_state: str | None = None) -> FolderHierarchySyncResult:
        """
        Synchronize folder hierarchy.

        Args:
            sync_state: Sync state from previous sync, None for initial sync

        Returns:
            FolderHierarchySyncResult with created, updated, and deleted folders

        Raises:
            Exception: If synchronization fails
        """
        ...

    async def create_folder(self, parent_id: str, name: str) -> str:
        """
        Create a new folder.

        Args:
            parent_id: Parent folder ID
            name: Display name for the new folder

        Returns:
            ID of the newly created folder

        Raises:
            Exception: If folder creation fails
        """
        ...

    async def update_folder(self, folder_id: str, folder_name: str) -> None:
        """
        Update folder display name.

        Args:
            folder_id: Folder ID to update
            folder_name: New display name

        Raises:
            Exception: If update fails
        """
        ...

    async def delete_folder(self, folder_ids: list[str]) -> None:
        """
        Delete one or more folders.

        Args:
            folder_ids: List of folder IDs to delete

        Raises:
            Exception: If deletion fails
        """
        ...

    async def copy_folders(self, destination_folder_id: str, folder_ids: list[str]) -> list[str]:
        """
        Copy folders to a destination folder.

        Args:
            destination_folder_id: Destination folder ID
            folder_ids: List of folder IDs to copy

        Returns:
            List of new folder IDs

        Raises:
            Exception: If copy fails
        """
        ...

    async def move_folders(self, destination_folder_id: str, folder_ids: list[str]) -> list[str]:
        """
        Move folders to a destination folder.

        Args:
            destination_folder_id: Destination folder ID
            folder_ids: List of folder IDs to move

        Returns:
            List of folder IDs (typically same as input)

        Raises:
            Exception: If move fails
        """
        ...

    async def sync_messages(self, folder_id: str, sync_state: str | None = None) -> SyncMessagesResult:
        """
        Synchronize messages in a folder.

        Args:
            folder_id: Folder ID to sync
            sync_state: Sync state from previous sync, None for initial sync

        Returns:
            SyncMessagesResult with created, updated, and deleted messages

        Raises:
            Exception: If synchronization fails
        """
        ...

    async def get_message(self, message_id: str) -> bytes:
        """
        Get message MIME content.

        Args:
            message_id: Message ID

        Returns:
            MIME content as bytes

        Raises:
            Exception: If message retrieval fails
        """
        ...

    async def create_message(
        self, folder_id: str, content: bytes, is_draft: bool, is_read: bool
    ) -> CreateMessageResult:
        """
        Create a message.

        Args:
            folder_id: Folder ID to create message in
            content: MIME content
            is_draft: Whether message is a draft
            is_read: Whether message should be marked as read

        Returns:
            CreateMessageResult with new message ID

        Raises:
            Exception: If message creation fails
        """
        ...

    async def send_message(
        self,
        mime_content: str,
        message_id: str,
        should_request_dsn: bool,
        bcc_recipients: list[tuple[str | None, str | None]],
    ) -> None:
        """
        Send a message.

        Args:
            mime_content: MIME content of the message
            message_id: Internet Message ID
            should_request_dsn: Whether to request delivery status notification
            bcc_recipients: List of (name, email) tuples for BCC recipients

        Raises:
            Exception: If sending fails
        """
        ...

    async def delete_messages(self, item_ids: list[str]) -> None:
        """
        Delete one or more messages.

        Args:
            item_ids: List of message IDs to delete

        Raises:
            Exception: If deletion fails
        """
        ...

    async def change_read_status(self, item_ids: list[str], is_read: bool) -> list[str]:
        """
        Change read status of messages.

        Args:
            item_ids: List of message IDs
            is_read: Whether to mark as read (True) or unread (False)

        Returns:
            List of successfully updated message IDs

        Raises:
            Exception: If operation fails
        """
        ...

    async def change_read_status_all(self, folder_ids: list[str], is_read: bool, suppress_read_receipts: bool) -> None:
        """
        Mark all messages in folders as read or unread.

        Args:
            folder_ids: List of folder IDs
            is_read: Whether to mark as read (True) or unread (False)
            suppress_read_receipts: Whether to suppress read receipts

        Raises:
            Exception: If operation fails
        """
        ...

    async def mark_as_junk(self, item_ids: list[str], is_junk: bool, legacy_junk_folder_id: str) -> list[str]:
        """
        Mark messages as junk or not junk.

        Args:
            item_ids: List of message IDs
            is_junk: Whether to mark as junk (True) or not junk (False)
            legacy_junk_folder_id: Junk folder ID for legacy Exchange versions

        Returns:
            List of successfully updated message IDs

        Raises:
            Exception: If operation fails
        """
        ...

    async def copy_items(self, destination_folder_id: str, item_ids: list[str]) -> list[str]:
        """
        Copy messages to a destination folder.

        Args:
            destination_folder_id: Destination folder ID
            item_ids: List of message IDs to copy

        Returns:
            List of new message IDs

        Raises:
            Exception: If copy fails
        """
        ...

    async def move_items(self, destination_folder_id: str, item_ids: list[str]) -> list[str]:
        """
        Move messages to a destination folder.

        Args:
            destination_folder_id: Destination folder ID
            item_ids: List of message IDs to move

        Returns:
            List of message IDs (typically same as input)

        Raises:
            Exception: If move fails
        """
        ...

__version__: str
