//! Integration tests for folder operations using mock EWS server
//!
//! These tests use the `MockEwsServer` to simulate EWS responses without requiring
//! a real Exchange server. This allows for fast, reliable testing in CI/CD environments.
//!
//! These tests send real HTTP requests to the mock server and verify the responses.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing,
    clippy::print_stdout
)]

use crate::common::{MockEwsServer, fixtures, test_utils::*};
use ews_client_core::client::{Credentials, EwsClient};

/// Helper function to create a SOAP request body for testing
fn create_soap_request(operation: &str, body_content: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <m:{operation}>
      {body_content}
    </m:{operation}>
  </soap:Body>
</soap:Envelope>"#
    )
}

/// Test check connectivity with mock server (using `EwsClient`)
#[tokio::test]
async fn test_check_connectivity_client() {
    let mock = MockEwsServer::new().await;

    // Register the mock response for msgfolderroot
    // check_connectivity requests the "msgfolderroot" distinguished folder
    let response_fixture = fixtures::get_folder_distinguished_response("msgfolderroot", "root-id");
    mock.register_operation("GetFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    let result = client.check_connectivity().await;
    assert!(result.is_ok(), "check_connectivity failed: {:?}", result.err());
}

/// Test creating a folder with mock server
#[tokio::test]
async fn test_create_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-create-123";
    let parent_id = "inbox";
    let folder_name = "Test Folder";

    // Register the mock response
    let response_fixture = fixtures::create_folder_response(folder_id);
    mock.register_operation("CreateFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to create folder
    let result = client.create_folder(parent_id, folder_name).await;

    // Verify response
    assert!(result.is_ok(), "create_folder failed: {:?}", result.err());
    let created_id = result.unwrap();
    assert_eq!(created_id, folder_id);
}

/// Test getting a folder with mock server
#[tokio::test]
async fn test_get_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-get-456";
    let display_name = "Test Folder";

    // Register the mock response
    mock.mock_get_folder(folder_id, display_name).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetFolder", &format!("<t:FolderId Id=\"{folder_id}\"/>"));

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");

    // Verify the response contains expected data
    assert_valid_soap_envelope(&body);
    assert_response_success(&body);
    assert_contains_folder_id(&body, folder_id);
    assert_contains_display_name(&body, display_name);
}

/// Test deleting a folder with mock server
#[tokio::test]
async fn test_delete_folder_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response
    let response_fixture = fixtures::delete_folder_response();
    mock.register_operation("DeleteFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to delete folder
    let result = client.delete_folder(&["folder-to-delete"]).await;

    // Verify response
    assert!(result.is_ok(), "delete_folder failed: {:?}", result.err());
}

/// Test updating a folder with mock server
#[tokio::test]
async fn test_update_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-update-789";

    // Register the mock response
    let response_fixture = fixtures::update_folder_response(folder_id);
    mock.register_operation("UpdateFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to update folder
    let result = client.update_folder(folder_id, "New Name").await;

    // Verify response
    assert!(result.is_ok(), "update_folder failed: {:?}", result.err());
}

/// Test finding folders with mock server
#[tokio::test]
async fn test_find_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-find-101";
    let display_name = "Found Folder";

    // Register the mock response
    mock.mock_find_folder(folder_id, display_name).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("FindFolder", "<t:ParentFolderId Id=\"inbox\"/>");

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");

    // Verify the response contains expected data
    assert_valid_soap_envelope(&body);
    assert_response_success(&body);
    assert_contains_folder_id(&body, folder_id);
    assert_contains_display_name(&body, display_name);
}

/// Test copying a folder with mock server
#[tokio::test]
async fn test_copy_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-copy-202";

    // Register the mock response
    let response_fixture = fixtures::copy_folder_response(folder_id);
    mock.register_operation("CopyFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to copy folder
    let result = client.copy_folders("dest-folder", &["source-folder"]).await;

    // Verify response
    assert!(result.is_ok(), "copy_folders failed: {:?}", result.err());
    let copied_ids = result.unwrap();
    assert_eq!(copied_ids.len(), 1);
    assert_eq!(copied_ids[0], folder_id);
}

/// Test moving a folder with mock server
#[tokio::test]
async fn test_move_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-move-303";

    // Register the mock response
    let response_fixture = fixtures::move_folder_response(folder_id);
    mock.register_operation("MoveFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to move folder
    let result = client.move_folders("dest-folder", &["source-folder"]).await;

    // Verify response
    assert!(result.is_ok(), "move_folders failed: {:?}", result.err());
    let moved_ids = result.unwrap();
    assert_eq!(moved_ids.len(), 1);
    // MoveFolder returns the new ID with "moved-" prefix
    assert_eq!(moved_ids[0], format!("moved-{folder_id}"));
}

/// Test syncing folder hierarchy with mock server
#[tokio::test]
async fn test_sync_folder_hierarchy_with_mock() {
    let mock = MockEwsServer::new().await;
    let sync_state = "sync-state-initial";
    let folder_id = "folder-test-sync-404";

    // When sync_state is None, the client will first call GetFolder to get well-known folders
    // We need to register responses for both GetFolder and SyncFolderHierarchy

    // Register GetFolder response for well-known folders (msgfolderroot, inbox, etc.)
    let get_folder_response = fixtures::batch_get_folder_response(
        &[
            "msgfolderroot",
            "inbox",
            "deleteditems",
            "drafts",
            "outbox",
            "sentitems",
            "junkemail",
            "archive",
        ],
        &[
            "Root",
            "Inbox",
            "Deleted Items",
            "Drafts",
            "Outbox",
            "Sent Items",
            "Junk Email",
            "Archive",
        ],
    );
    mock.register_operation("GetFolder", get_folder_response).await;

    // Register SyncFolderHierarchy response
    let sync_response = fixtures::sync_folder_hierarchy_response(sync_state, folder_id);
    mock.register_operation("SyncFolderHierarchy", sync_response).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to sync folder hierarchy
    let result = client.sync_folder_hierarchy(None).await;

    // Verify response
    assert!(result.is_ok(), "sync_folder_hierarchy failed: {:?}", result.err());
    let sync_result = result.unwrap();
    assert_eq!(sync_result.sync_state, sync_state);

    // Verify we got some changes
    assert!(
        !sync_result.created_folders.is_empty()
            || !sync_result.updated_folders.is_empty()
            || !sync_result.deleted_folder_ids.is_empty()
    );
}

/// Test batch folder operations with mock server
#[tokio::test]
async fn test_batch_get_folders_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_ids = vec!["folder-1", "folder-2"];
    let display_names = vec!["Folder 1", "Folder 2"];

    // Register the mock response
    let response_fixture = fixtures::batch_get_folder_response(&folder_ids, &display_names);
    mock.register_operation("GetFolder", response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request(
        "GetFolder",
        "<t:FolderId Id=\"folder-1\"/><t:FolderId Id=\"folder-2\"/>",
    );

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");

    // Verify the response contains expected data
    assert_valid_soap_envelope(&body);
    assert_batch_response_success(&body, 2);
    assert_contains_folder_id(&body, folder_ids[0]);
    assert_contains_folder_id(&body, folder_ids[1]);
    assert_contains_display_name(&body, display_names[0]);
    assert_contains_display_name(&body, display_names[1]);
}

/// Test error handling with mock server - folder not found
#[tokio::test]
async fn test_folder_not_found_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register an error response
    let response_fixture = fixtures::error_folder_not_found();
    mock.register_operation("GetFolder", response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetFolder", "<t:FolderId Id=\"non-existent\"/>");

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response
    assert_eq!(response.status(), 200); // EWS returns 200 even for errors
    let body = response.text().await.expect("Failed to read response body");

    // Verify the response contains expected error
    assert_valid_soap_envelope(&body);
    assert_response_error(&body, "ErrorFolderNotFound");
}

/// Test error handling with mock server - access denied
#[tokio::test]
async fn test_access_denied_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register an error response
    let response_fixture = fixtures::error_access_denied();
    mock.register_operation("DeleteFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to delete folder (should fail)
    let result = client.delete_folder(&["protected-folder"]).await;

    // Verify response
    assert!(result.is_err());
    // We could check the specific error type if EwsError exposes it
}

/// Test authentication error with mock server
#[tokio::test]
async fn test_authentication_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register an authentication error response
    mock.register_auth_error().await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetFolder", "<t:FolderId Id=\"inbox\"/>");

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response - should be 401 Unauthorized
    assert_eq!(response.status(), 401);
    assert!(response.headers().contains_key("www-authenticate"));
}

/// Test server error with mock server
#[tokio::test]
async fn test_server_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register a server error response
    mock.register_server_error().await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetFolder", "<t:FolderId Id=\"any-folder\"/>");

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response - should be 500 Internal Server Error
    assert_eq!(response.status(), 500);
}

/// Test paginated folder search with mock server
#[tokio::test]
async fn test_find_folder_paginated_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_ids = vec!["folder-p1", "folder-p2", "folder-p3"];
    let display_names = vec!["Paginated 1", "Paginated 2", "Paginated 3"];
    let total_count = 10;
    let includes_last_item_in_range = false;

    // Register the mock response
    let response_fixture =
        fixtures::find_folder_paginated_response(&folder_ids, &display_names, total_count, includes_last_item_in_range);
    mock.register_operation("FindFolder", response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("FindFolder", "<t:ParentFolderId Id=\"inbox\"/>");

    let response = client
        .post(mock.ews_endpoint())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(request_body)
        .send()
        .await
        .expect("Failed to send request");

    // Verify response
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");

    // Verify the response contains expected data
    assert_valid_soap_envelope(&body);
    assert_response_success(&body);

    // Verify all folder IDs and names are present
    for folder_id in &folder_ids {
        assert_contains_folder_id(&body, folder_id);
    }
    for display_name in &display_names {
        assert_contains_display_name(&body, display_name);
    }

    // Verify pagination info
    assert!(body.contains(&total_count.to_string()));
}

/// Test sync folder hierarchy with changes
#[tokio::test]
async fn test_sync_folder_hierarchy_with_changes_mock() {
    let mock = MockEwsServer::new().await;
    let sync_state = "sync-state-with-changes";
    let create_id = "folder-created";
    let update_id = "folder-updated";
    let delete_id = "folder-deleted";

    // Register SyncFolderHierarchy response
    let sync_response =
        fixtures::sync_folder_hierarchy_with_changes_response(sync_state, create_id, update_id, delete_id);
    mock.register_operation("SyncFolderHierarchy", sync_response).await;

    // Register GetFolder response for fetching folder details
    // The client will call GetFolder to get full details of created and updated folders
    let get_folder_response =
        fixtures::batch_get_folder_response(&[create_id, update_id], &["NewFolder", "UpdatedFolder"]);
    mock.register_operation("GetFolder", get_folder_response).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to sync folder hierarchy
    let result = client.sync_folder_hierarchy(Some("old-state".to_string())).await;

    // Verify response
    assert!(result.is_ok(), "sync_folder_hierarchy failed: {:?}", result.err());
    let sync_result = result.unwrap();
    assert_eq!(sync_result.sync_state, sync_state);

    // Verify all change types are present
    // Note: The exact verification depends on how EwsClient parses the response
    // For now we just check that we got changes
    assert!(!sync_result.created_folders.is_empty());
    assert!(!sync_result.updated_folders.is_empty());
    assert!(!sync_result.deleted_folder_ids.is_empty());
}

/// Test batch delete folders with mixed results
#[tokio::test]
async fn test_batch_delete_folders_mixed_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response with 2 successes and 1 error
    let response_fixture = fixtures::batch_delete_folder_mixed_response(2, 1);
    mock.register_operation("DeleteFolder", response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    // Use EwsClient to delete folders
    // Note: delete_folder currently returns Ok(()) if some fail (it logs errors),
    // or returns error if all fail? The implementation seems to return Ok(()) if some fail with ErrorItemNotFound,
    // but might return error for other failures.
    // The fixture returns ErrorItemNotFound for the failure case.
    let result = client.delete_folder(&["folder-1", "folder-2", "folder-3"]).await;

    // Verify response
    assert!(result.is_ok(), "delete_folder failed: {:?}", result.err());
}
