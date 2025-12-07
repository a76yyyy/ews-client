//! Integration tests for item operations using mock EWS server
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

/// Test creating an item with mock server
#[tokio::test]
async fn test_create_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-create-123";

    // Register the mock response
    mock.mock_create_item(item_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("CreateItem", "<t:Message/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test getting an item with mock server
#[tokio::test]
async fn test_get_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-get-456";
    let subject = "Test Subject";

    // Register the mock response
    mock.mock_get_item(item_id, subject).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetItem", &format!("<t:ItemId Id=\"{item_id}\"/>"));

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
    assert_contains_item_id(&body, item_id);
    assert_contains_subject(&body, subject);
}

/// Test deleting an item with mock server
#[tokio::test]
async fn test_delete_item_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response
    mock.mock_delete_item().await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("DeleteItem", "<t:ItemId Id=\"item-to-delete\"/>");

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

/// Test updating an item with mock server
#[tokio::test]
async fn test_update_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-update-789";

    // Register the mock response
    mock.mock_update_item(item_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("UpdateItem", &format!("<t:ItemId Id=\"{item_id}\"/>"));

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
    assert_contains_item_id(&body, item_id);
}

/// Test finding items with mock server
#[tokio::test]
async fn test_find_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-find-101";

    // Register the mock response
    mock.mock_find_item(item_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("FindItem", "<t:ParentFolderId Id=\"inbox\"/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test sending an item with mock server
#[tokio::test]
async fn test_send_item_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response
    mock.mock_send_item().await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("SendItem", "<t:ItemId Id=\"item-to-send\"/>");

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

/// Test copying an item with mock server
#[tokio::test]
async fn test_copy_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-copy-202";

    // Register the mock response
    let response_fixture = fixtures::copy_item_response(item_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("CopyItem", "<t:ItemId Id=\"source-item\"/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test moving an item with mock server
#[tokio::test]
async fn test_move_item_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-move-303";

    // Register the mock response
    let response_fixture = fixtures::move_item_response(item_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MoveItem", "<t:ItemId Id=\"source-item\"/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test syncing folder items with mock server
#[tokio::test]
async fn test_sync_folder_items_with_mock() {
    let mock = MockEwsServer::new().await;
    let sync_state = "sync-state-items";
    let item_id = "item-test-sync-404";

    // Register the mock response
    mock.mock_sync_folder_items(sync_state, item_id).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("SyncFolderItems", "<t:SyncFolderId Id=\"inbox\"/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test marking item as junk with mock server
#[tokio::test]
async fn test_mark_as_junk_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-test-junk-505";

    // Register the mock response
    let response_fixture = fixtures::mark_as_junk_response(Some(item_id));
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MarkAsJunk", "<t:ItemId Id=\"junk-item\"/>");

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
    assert_contains_item_id(&body, item_id);
}

/// Test marking all items as read with mock server
#[tokio::test]
async fn test_mark_all_items_as_read_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response
    let response_fixture = fixtures::mark_all_items_as_read_response();
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MarkAllItemsAsRead", "<t:FolderId Id=\"inbox\"/>");

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

/// Test batch get items with mock server
#[tokio::test]
async fn test_batch_get_items_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-1", "item-2"];
    let subjects = vec!["Subject 1", "Subject 2"];

    // Register the mock response
    let response_fixture = fixtures::batch_get_item_response(&item_ids, &subjects);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetItem", "<t:ItemId Id=\"item-1\"/><t:ItemId Id=\"item-2\"/>");

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
    assert_contains_item_id(&body, item_ids[0]);
    assert_contains_item_id(&body, item_ids[1]);
    assert_contains_subject(&body, subjects[0]);
    assert_contains_subject(&body, subjects[1]);
}

/// Test batch delete items with mock server
#[tokio::test]
async fn test_batch_delete_items_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response
    let response_fixture = fixtures::batch_delete_item_response(2);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("DeleteItem", "<t:ItemId Id=\"item-1\"/><t:ItemId Id=\"item-2\"/>");

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
}

/// Test batch update items with mock server
#[tokio::test]
async fn test_batch_update_items_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-u1", "item-u2"];

    // Register the mock response
    let response_fixture = fixtures::batch_update_item_response(&item_ids);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("UpdateItem", "<t:ItemId Id=\"item-u1\"/><t:ItemId Id=\"item-u2\"/>");

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
    assert_contains_item_id(&body, item_ids[0]);
    assert_contains_item_id(&body, item_ids[1]);
}

/// Test batch mark as junk with mock server
#[tokio::test]
async fn test_batch_mark_as_junk_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-j1", "item-j2"];

    // Register the mock response
    let response_fixture = fixtures::batch_mark_as_junk_response(&item_ids);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MarkAsJunk", "<t:ItemId Id=\"item-j1\"/><t:ItemId Id=\"item-j2\"/>");

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
    assert_contains_item_id(&body, item_ids[0]);
    assert_contains_item_id(&body, item_ids[1]);
}

/// Test batch copy items with mock server
#[tokio::test]
async fn test_batch_copy_items_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-c1", "item-c2"];

    // Register the mock response
    let response_fixture = fixtures::batch_copy_item_response(&item_ids);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("CopyItem", "<t:ItemId Id=\"item-c1\"/><t:ItemId Id=\"item-c2\"/>");

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
    assert_contains_item_id(&body, item_ids[0]);
    assert_contains_item_id(&body, item_ids[1]);
}

/// Test batch move items with mock server
#[tokio::test]
async fn test_batch_move_items_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-m1", "item-m2"];

    // Register the mock response
    let response_fixture = fixtures::batch_move_item_response(&item_ids);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("MoveItem", "<t:ItemId Id=\"item-m1\"/><t:ItemId Id=\"item-m2\"/>");

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
    assert_contains_item_id(&body, item_ids[0]);
    assert_contains_item_id(&body, item_ids[1]);
}

/// Test item not found error with mock server
#[tokio::test]
async fn test_item_not_found_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register an error response
    let response_fixture = fixtures::error_item_not_found();
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetItem", "<t:ItemId Id=\"non-existent\"/>");

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
    assert_response_error(&body, "ErrorItemNotFound");
}

/// Test invalid change key error with mock server
#[tokio::test]
async fn test_invalid_change_key_error_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register an error response
    let response_fixture = fixtures::error_invalid_change_key();
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("UpdateItem", "<t:ItemId Id=\"item-id\" ChangeKey=\"invalid\"/>");

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
    assert_response_error(&body, "ErrorInvalidChangeKey");
}

/// Test paginated item search with mock server
#[tokio::test]
async fn test_find_item_paginated_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_ids = vec!["item-p1", "item-p2", "item-p3"];
    let total_count = 15;
    let includes_last_item_in_range = false;

    // Register the mock response
    let response_fixture = fixtures::find_item_paginated_response(&item_ids, total_count, includes_last_item_in_range);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("FindItem", "<t:ParentFolderId Id=\"inbox\"/>");

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

    // Verify all item IDs are present
    for item_id in &item_ids {
        assert_contains_item_id(&body, item_id);
    }

    // Verify pagination info
    assert!(body.contains(&total_count.to_string()));
}

/// Test sync folder items with changes
#[tokio::test]
async fn test_sync_folder_items_with_changes_mock() {
    let mock = MockEwsServer::new().await;
    let sync_state = "sync-state-items-changes";
    let create_id = "item-created";
    let update_id = "item-updated";
    let delete_id = "item-deleted";

    // Register the mock response
    let response_fixture =
        fixtures::sync_folder_items_with_changes_response(sync_state, create_id, update_id, delete_id);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("SyncFolderItems", "<m:SyncState>old-state</m:SyncState>");

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
    assert_contains_item_id(&body, create_id);
    assert_contains_item_id(&body, update_id);
    assert!(body.contains(delete_id)); // Deleted items might not have full ItemId structure
}

/// Test get item with MIME content
#[tokio::test]
async fn test_get_item_with_mime_with_mock() {
    let mock = MockEwsServer::new().await;
    let item_id = "item-mime-606";
    let subject = "MIME Subject";
    let mime_content = "From: test@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";

    // Register the mock response
    let response_fixture = fixtures::get_item_with_mime_response(item_id, subject, mime_content);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request("GetItem", &format!("<t:ItemId Id=\"{item_id}\"/>"));

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
    assert_contains_item_id(&body, item_id);
    assert_contains_subject(&body, subject);
    assert!(body.contains("MimeContent")); // Verify MIME content is present
}

/// Test batch delete items with mixed results
#[tokio::test]
async fn test_batch_delete_items_mixed_with_mock() {
    let mock = MockEwsServer::new().await;

    // Register the mock response with 2 successes and 1 error
    let response_fixture = fixtures::batch_delete_item_mixed_response(2, 1);
    mock.register_response(response_fixture).await;

    // Send a real HTTP request
    let client = reqwest::Client::new();
    let request_body = create_soap_request(
        "DeleteItem",
        "<t:ItemId Id=\"item-1\"/><t:ItemId Id=\"item-2\"/><t:ItemId Id=\"item-3\"/>",
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
