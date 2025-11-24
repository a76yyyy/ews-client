//! Core EWS operations
//!
//! This module contains implementations of core EWS operations:
//! - check_connectivity: Test connection and authentication
//! - sync_folder_hierarchy: Synchronize folder structure
//! - get_message: Fetch individual messages
//! - Folder operations: create, delete, update, copy, move
//! - Message operations: sync, create, delete, change read status, mark as junk, copy, move
//!
//! Each operation is implemented as a method on EwsClient and returns
//! a Result with appropriate error handling.

mod check_connectivity;

pub mod copy_move_operations;
mod create_folder;
mod delete_folder;
mod get_message;
mod sync_folder_hierarchy;
mod update_folder;

mod change_read_status;
mod create_message;
mod delete_messages;
mod mark_as_junk;
mod sync_messages;

mod send_message;

// Re-export public types
pub use create_message::CreateMessageResult;
pub use sync_folder_hierarchy::{FolderHierarchySyncResult, FolderInfo};
pub use sync_messages::{SyncMessageInfo, SyncMessagesResult};
