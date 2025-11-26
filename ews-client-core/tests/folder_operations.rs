//! Integration tests for folder operations
//!
//! These tests require a live EWS server and are ignored by default.
//! Run with: `cargo test --package ews-client-core -- --ignored`

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing
)]

use ews_client_core::client::{Credentials, EwsClient};

// Helper function to create a test client
// In real tests, you would read credentials from environment variables
#[allow(dead_code)]
fn create_test_client() -> EwsClient {
    let endpoint = std::env::var("EWS_ENDPOINT")
        .unwrap_or_else(|_| "https://outlook.office365.com/EWS/Exchange.asmx".to_string())
        .parse()
        .expect("Invalid EWS endpoint URL");

    let username = std::env::var("EWS_USERNAME").expect("EWS_USERNAME not set");
    let password = std::env::var("EWS_PASSWORD").expect("EWS_PASSWORD not set");

    let credentials = Credentials::basic(&username, &password);

    EwsClient::new(endpoint, credentials).expect("Failed to create EWS client")
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_create_and_delete_folder() {
    let client = create_test_client();

    // Get inbox folder ID (you would need to implement get_folder or use a known ID)
    let parent_folder_id = "inbox"; // or use a real folder ID

    // Create a test folder
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let folder_name = format!("Test Folder {timestamp}");
    let folder_id = client
        .create_folder(parent_folder_id, &folder_name)
        .await
        .expect("Failed to create folder");

    assert!(!folder_id.is_empty(), "Created folder should have an ID");

    // Delete the test folder
    client
        .delete_folder(&[&folder_id])
        .await
        .expect("Failed to delete folder");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_update_folder() {
    let client = create_test_client();

    // Create a test folder first
    let parent_folder_id = "inbox";
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let original_name = format!("Test Folder {timestamp}");
    let folder_id = client
        .create_folder(parent_folder_id, &original_name)
        .await
        .expect("Failed to create folder");

    // Update the folder name
    let new_name = format!("{original_name} - Updated");
    client
        .update_folder(&folder_id, &new_name)
        .await
        .expect("Failed to update folder");

    // Clean up
    client
        .delete_folder(&[&folder_id])
        .await
        .expect("Failed to delete folder");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_copy_folders() {
    let client = create_test_client();

    // Create a test folder
    let parent_folder_id = "inbox";
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let folder_name = format!("Test Folder {timestamp}");
    let folder_id = client
        .create_folder(parent_folder_id, &folder_name)
        .await
        .expect("Failed to create folder");

    // Copy the folder (to the same parent for simplicity)
    let new_ids = client
        .copy_folders(parent_folder_id, &[&folder_id])
        .await
        .expect("Failed to copy folder");

    assert_eq!(new_ids.len(), 1, "Should have one copied folder");
    assert_ne!(new_ids[0], folder_id, "Copied folder should have a different ID");

    // Clean up both folders
    let mut all_ids = vec![folder_id];
    all_ids.extend(new_ids);
    let id_refs: Vec<&str> = all_ids.iter().map(std::string::String::as_str).collect();
    client.delete_folder(&id_refs).await.expect("Failed to delete folders");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_move_folders() {
    let client = create_test_client();

    // Create a test folder
    let parent_folder_id = "inbox";
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let folder_name = format!("Test Folder {timestamp}");
    let folder_id = client
        .create_folder(parent_folder_id, &folder_name)
        .await
        .expect("Failed to create folder");

    // Create a destination folder
    let timestamp2 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let dest_folder_name = format!("Dest Folder {timestamp2}");
    let dest_folder_id = client
        .create_folder(parent_folder_id, &dest_folder_name)
        .await
        .expect("Failed to create destination folder");

    // Move the folder
    let moved_ids = client
        .move_folders(&dest_folder_id, &[&folder_id])
        .await
        .expect("Failed to move folder");

    assert_eq!(moved_ids.len(), 1, "Should have one moved folder");

    // Clean up (delete the destination folder, which should also delete the moved folder)
    client
        .delete_folder(&[&dest_folder_id])
        .await
        .expect("Failed to delete destination folder");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_delete_multiple_folders() {
    let client = create_test_client();

    let parent_folder_id = "inbox";

    // Create multiple test folders
    let timestamp1 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let folder_name_1 = format!("Test Folder 1 {timestamp1}");
    let folder_id_1 = client
        .create_folder(parent_folder_id, &folder_name_1)
        .await
        .expect("Failed to create folder 1");

    let timestamp2 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let folder_name_2 = format!("Test Folder 2 {timestamp2}");
    let folder_id_2 = client
        .create_folder(parent_folder_id, &folder_name_2)
        .await
        .expect("Failed to create folder 2");

    // Delete both folders at once
    client
        .delete_folder(&[&folder_id_1, &folder_id_2])
        .await
        .expect("Failed to delete folders");
}
