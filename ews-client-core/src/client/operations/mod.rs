// Core EWS operations
//
// This module contains implementations of core EWS operations:
// - check_connectivity: Test connection and authentication
// - sync_folder_hierarchy: Synchronize folder structure
// - get_message: Fetch individual messages
//
// Each operation is implemented as a method on EwsClient and returns
// a Result with appropriate error handling.

mod check_connectivity;
pub mod copy_move_operations;
mod create_folder;
mod delete_folder;
mod get_message;
mod sync_folder_hierarchy;
mod update_folder;

// Re-export public types
pub use sync_folder_hierarchy::{FolderHierarchySyncResult, FolderInfo};

// Phase 2.1: Folder operations - COMPLETED
// Phase 2.2: Message operations - TODO
// Phase 2.3: Send message - TODO
