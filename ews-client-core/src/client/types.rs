//! Message and folder synchronization utilities.

use serde::{Deserialize, Serialize};

/// Result of folder synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderSyncResult {
    /// List of synchronized folders
    pub folders: Vec<FolderInfo>,
    /// Synchronization state token for incremental sync
    pub sync_state: String,
}

/// Folder information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    /// Folder ID
    pub id: String,
    /// Parent folder ID
    pub parent_id: Option<String>,
    /// Display name of the folder
    pub display_name: String,
}

/// Result of message synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSyncResult {
    /// List of synchronized messages
    pub messages: Vec<MessageInfo>,
    /// Synchronization state token for incremental sync
    pub sync_state: String,
}

/// Message information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    /// Message ID
    pub id: String,
    /// Message subject
    pub subject: Option<String>,
    /// Sender email address
    pub from: Option<String>,
    /// Whether the message has been read
    pub is_read: bool,
}
