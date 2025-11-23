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
mod get_message;
mod sync_folder_hierarchy;

// Re-export public types
pub use sync_folder_hierarchy::{FolderHierarchySyncResult, FolderInfo};

// Operations will be implemented in phases 2

// Phase 2: Complete operation set
// - Folder operations
// - Message operations
// - Send message
