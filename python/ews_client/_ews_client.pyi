"""Type stubs for ews_client."""

from .types import (
    CreateMessageResult,
    FolderHierarchySyncResult,
    SyncMessagesResult,
)

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
