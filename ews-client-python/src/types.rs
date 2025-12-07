//! Type conversion between Rust and Python.
//!
//! This module provides type conversion utilities for EWS client types.
//! PyO3 automatically handles conversions for basic types:
//! - `Vec<T>` ↔ `list[T]`
//! - `Option<T>` ↔ `Optional[T]`
//! - `String` ↔ `str`
//! - `Vec<u8>` ↔ `bytes`
//! - `HashMap<K, V>` ↔ `dict[K, V]`
//! - `(T, U)` ↔ `tuple[T, U]`
//!
//! Complex types are implemented as `#[pyclass]` for better type safety and IDE support.
//!
//! See:
//! - `reference/pyo3/guide/src/conversions/tables.md` - Type mapping table
//! - `reference/pyo3/guide/src/conversions/traits.md` - Conversion traits

use ews_client_core::client::operations::{
    CreateMessageResult, FolderHierarchySyncResult, FolderInfo, SyncMessageInfo, SyncMessagesResult,
};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Python wrapper for folder information.
///
/// Represents a single folder in the EWS folder hierarchy.
#[pyclass]
#[derive(Clone)]
pub struct PyFolderInfo {
    /// The EWS folder ID
    #[pyo3(get)]
    pub folder_id: String,
    /// The parent folder ID
    #[pyo3(get)]
    pub parent_folder_id: String,
    /// The display name of the folder
    #[pyo3(get)]
    pub display_name: String,
    /// The folder class (e.g., "IPF.Note" for mail folders)
    #[pyo3(get)]
    pub folder_class: Option<String>,
    /// Total number of items in the folder
    #[pyo3(get)]
    pub total_count: Option<u32>,
    /// Number of unread items in the folder
    #[pyo3(get)]
    pub unread_count: Option<u32>,
    /// Number of child folders
    #[pyo3(get)]
    pub child_folder_count: Option<u32>,
}

impl From<FolderInfo> for PyFolderInfo {
    fn from(info: FolderInfo) -> Self {
        Self {
            folder_id: info.folder_id,
            parent_folder_id: info.parent_folder_id,
            display_name: info.display_name,
            folder_class: info.folder_class,
            total_count: info.total_count,
            unread_count: info.unread_count,
            child_folder_count: info.child_folder_count,
        }
    }
}

/// Python wrapper for folder hierarchy synchronization result.
///
/// Contains the changes to the folder hierarchy since the last sync.
#[pyclass]
#[derive(Clone)]
pub struct PyFolderHierarchySyncResult {
    /// The new sync state token to use for the next sync
    #[pyo3(get)]
    pub sync_state: String,
    /// Folders that were created since the last sync
    #[pyo3(get)]
    pub created_folders: Vec<PyFolderInfo>,
    /// Folders that were updated since the last sync
    #[pyo3(get)]
    pub updated_folders: Vec<PyFolderInfo>,
    /// IDs of folders that were deleted since the last sync
    #[pyo3(get)]
    pub deleted_folder_ids: Vec<String>,
    /// Map of well-known folder IDs to their distinguished names
    /// (e.g., "inbox", "deleteditems", "drafts", etc.)
    #[pyo3(get)]
    pub well_known_folders: Option<HashMap<String, String>>,
}

impl From<FolderHierarchySyncResult> for PyFolderHierarchySyncResult {
    fn from(result: FolderHierarchySyncResult) -> Self {
        Self {
            sync_state: result.sync_state,
            created_folders: result.created_folders.into_iter().map(PyFolderInfo::from).collect(),
            updated_folders: result.updated_folders.into_iter().map(PyFolderInfo::from).collect(),
            deleted_folder_ids: result.deleted_folder_ids,
            well_known_folders: result.well_known_folders,
        }
    }
}

/// Python wrapper for synchronized message information.
///
/// Contains detailed information about a message from a sync operation.
#[pyclass]
#[derive(Clone)]
pub struct PySyncMessageInfo {
    /// The EWS item ID of the message
    #[pyo3(get)]
    pub item_id: String,
    /// Whether the message has been read
    #[pyo3(get)]
    pub is_read: Option<bool>,
    /// The Internet message ID (RFC 2822 Message-ID header)
    #[pyo3(get)]
    pub internet_message_id: Option<String>,
    /// The date and time the message was sent (Unix timestamp in seconds)
    #[pyo3(get)]
    pub date_time_sent: Option<i64>,
    /// The sender's email address
    #[pyo3(get)]
    pub from_: Option<String>,
    /// The message subject
    #[pyo3(get)]
    pub subject: Option<String>,
    /// Whether the message has attachments
    #[pyo3(get)]
    pub has_attachments: Option<bool>,
    /// The size of the message in bytes
    #[pyo3(get)]
    pub size: Option<usize>,
}

impl From<SyncMessageInfo> for PySyncMessageInfo {
    fn from(info: SyncMessageInfo) -> Self {
        Self {
            item_id: info.item_id,
            is_read: info.is_read,
            internet_message_id: info.internet_message_id,
            date_time_sent: info.date_time_sent,
            from_: info.from,
            subject: info.subject,
            has_attachments: info.has_attachments,
            size: info.size,
        }
    }
}

/// Python wrapper for message synchronization result.
///
/// Contains the changes to messages in a folder since the last sync.
#[pyclass]
#[derive(Clone)]
pub struct PySyncMessagesResult {
    /// Message IDs that were created
    #[pyo3(get)]
    pub created: Vec<String>,
    /// Message IDs that were updated
    #[pyo3(get)]
    pub updated: Vec<String>,
    /// Message IDs that were deleted
    #[pyo3(get)]
    pub deleted: Vec<String>,
    /// Message IDs with read status changed (`item_id`, `is_read`)
    #[pyo3(get)]
    pub read_status_changed: Vec<(String, bool)>,
    /// The new sync state token for the next sync
    #[pyo3(get)]
    pub sync_state: String,
    /// Whether there are more changes to fetch
    #[pyo3(get)]
    pub includes_last_item: bool,
}

impl From<SyncMessagesResult> for PySyncMessagesResult {
    fn from(result: SyncMessagesResult) -> Self {
        Self {
            created: result.created,
            updated: result.updated,
            deleted: result.deleted,
            read_status_changed: result.read_status_changed,
            sync_state: result.sync_state,
            includes_last_item: result.includes_last_item,
        }
    }
}

/// Python wrapper for create message result.
///
/// Contains the ID of the newly created message.
#[pyclass]
#[derive(Clone)]
pub struct PyCreateMessageResult {
    /// The EWS ID of the newly created message
    #[pyo3(get)]
    pub item_id: String,
}

impl From<CreateMessageResult> for PyCreateMessageResult {
    fn from(result: CreateMessageResult) -> Self {
        Self {
            item_id: result.item_id,
        }
    }
}
