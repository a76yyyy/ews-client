use serde::{Deserialize, Serialize};

/// Result of folder synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderSyncResult {
    pub folders: Vec<FolderInfo>,
    pub sync_state: String,
}

/// Folder information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub id: String,
    pub parent_id: Option<String>,
    pub display_name: String,
}

/// Result of message synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSyncResult {
    pub messages: Vec<MessageInfo>,
    pub sync_state: String,
}

/// Message information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub is_read: bool,
}
