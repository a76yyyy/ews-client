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
    mock.register_response(response_fixture).await;

    let client = EwsClient::new(mock.ews_endpoint().parse().unwrap(), Credentials::basic("user", "pass")).unwrap();

    let result = client.check_connectivity().await;
    assert!(result.is_ok(), "check_connectivity failed: {:?}", result.err());
}

/// Test creating a folder with mock server
#[tokio::test]
async fn test_create_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-create-123";

    // Register the mock response
    mock.mock_create_folder(folder_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("CreateFolder", "<t:FolderId Id=\"inbox\"/>");

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
    mock.mock_delete_folder().await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("DeleteFolder", "<t:FolderId Id=\"folder-to-delete\"/>");

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
}

/// Test updating a folder with mock server
#[tokio::test]
async fn test_update_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-update-789";

    // Register the mock response
    let response_fixture = fixtures::update_folder_response(folder_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("UpdateFolder", &format!("<t:FolderId Id=\"{folder_id}\"/>"));

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
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("CopyFolder", "<t:FolderId Id=\"source-folder\"/>");

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
}

/// Test moving a folder with mock server
#[tokio::test]
async fn test_move_folder_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_id = "folder-test-move-303";

    // Register the mock response
    let response_fixture = fixtures::move_folder_response(folder_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MoveFolder", "<t:FolderId Id=\"source-folder\"/>");

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
}

/// Test syncing folder hierarchy with mock server
#[tokio::test]
async fn test_sync_folder_hierarchy_with_mock() {
    let mock = MockEwsServer::new().await;
    let sync_state = "sync-state-initial";
    let folder_id = "folder-test-sync-404";

    // Register the mock response
    mock.mock_sync_folder_hierarchy(sync_state, folder_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("SyncFolderHierarchy", "");

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
    assert_contains_sync_state(&body, sync_state);
    assert_contains_folder_id(&body, folder_id);
}

/// Test batch folder operations with mock server
#[tokio::test]
async fn test_batch_get_folders_with_mock() {
    let mock = MockEwsServer::new().await;
    let folder_ids = vec!["folder-1", "folder-2"];
    let display_names = vec!["Folder 1", "Folder 2"];

    // Register the mock response
    let response_fixture = fixtures::batch_get_folder_response(&folder_ids, &display_names);
    mock.register_response(response_fixture).await;

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
    mock.register_response(response_fixture).await;

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
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("DeleteFolder", "<t:FolderId Id=\"protected-folder\"/>");

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
    assert_response_error(&body, "ErrorAccessDenied");
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
    mock.register_response(response_fixture).await;

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

    // Register the mock response
    let response_fixture =
        fixtures::sync_folder_hierarchy_with_changes_response(sync_state, create_id, update_id, delete_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("SyncFolderHierarchy", "<m:SyncState>old-state</m:SyncState>");

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
    assert_contains_sync_state(&body, sync_state);

    // Verify all change types are present
    assert_contains_folder_id(&body, create_id);
    assert_contains_folder_id(&body, update_id);
    assert!(body.contains(delete_id)); // Deleted items might not have full FolderId structure
}

/// Test batch delete folders with mixed results
#[tokio::test]
async fn test_batch_delete_folders_mixed_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response with 2 successes and 1 error
    let response_fixture = fixtures::batch_delete_folder_mixed_response(2, 1);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request(
        "DeleteFolder",
        "<t:FolderId Id=\"folder-1\"/><t:FolderId Id=\"folder-2\"/><t:FolderId Id=\"folder-3\"/>",
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
    assert_batch_response_mixed(&body, 2, 1);
}
