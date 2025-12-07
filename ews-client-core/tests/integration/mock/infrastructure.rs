//! Infrastructure tests to verify the test framework is working correctly
//!
//! This test file is part of the integration test suite
//! It imports from the common test library defined in tests/lib.rs

#![allow(clippy::expect_used)]

use crate::common::{MockEwsServer, fixtures, test_utils::*};

#[tokio::test]
async fn test_mock_server_starts() {
    let mock = MockEwsServer::new().await;
    let url = mock.url();
    assert!(url.starts_with("http://"));
}

#[tokio::test]
async fn test_mock_server_ews_endpoint() {
    let mock = MockEwsServer::new().await;
    let endpoint = mock.ews_endpoint();
    assert!(endpoint.ends_with("/EWS/Exchange.asmx"));
}

#[tokio::test]
async fn test_mock_server_register_response() {
    let mock = MockEwsServer::new().await;
    let response = fixtures::create_folder_response("test-folder-id");
    mock.register_response(response).await;

    // Verify the mock is ready to receive requests
    let endpoint = mock.ews_endpoint();
    assert!(!endpoint.is_empty());
}

#[test]
fn test_fixtures_create_folder_response() {
    let response = fixtures::create_folder_response("folder-123");
    assert_valid_soap_envelope(&response);
    assert_response_success(&response);
    assert_contains_folder_id(&response, "folder-123");
}

#[test]
fn test_fixtures_error_response() {
    let response = fixtures::error_folder_not_found();
    assert_valid_soap_envelope(&response);
    assert_response_error(&response, "ErrorFolderNotFound");
}

#[test]
fn test_generate_test_ids() {
    let folder_id = generate_test_folder_id();
    let item_id = generate_test_item_id();
    let sync_state = generate_test_sync_state();

    assert!(folder_id.starts_with("folder-"));
    assert!(item_id.starts_with("item-"));
    assert!(sync_state.starts_with("sync-state-"));
}

#[test]
fn test_create_test_client() {
    let _client = create_test_client("http://localhost:3000/EWS/Exchange.asmx");
    // Just verify it was created without panicking
}

#[test]
fn test_all_fixtures_are_valid_soap() {
    let fixture_responses = vec![
        fixtures::create_folder_response("id1"),
        fixtures::delete_folder_response(),
        fixtures::get_folder_response("id2", "Test Folder"),
        fixtures::get_folder_full_response("id3", "parent", "Test Folder", "IPF.Note"),
        fixtures::get_folder_distinguished_response("inbox", "id4"),
        fixtures::batch_get_folder_response(&["id5", "id6"], &["F5", "F6"]),
        fixtures::sync_folder_hierarchy_response("state1", "id7"),
        fixtures::sync_folder_hierarchy_with_changes_response("state2", "c1", "u1", "d1"),
        fixtures::sync_folder_hierarchy_paginated_response("state3", "state4", "id8"),
        fixtures::create_item_response("id9"),
        fixtures::create_calendar_item_response("id_cal"),
        fixtures::create_contact_response("id_contact"),
        fixtures::create_task_response("id_task"),
        fixtures::delete_item_response(),
        fixtures::get_item_response("id10", "Test Subject"),
        fixtures::get_item_with_mime_response("id11", "Subject", "MIME"),
        fixtures::get_item_full_response("id12", "Sub", "From", "from@e.com", true, false, 100, "Normal", "ref"),
        fixtures::get_item_with_recipients_response(
            "id13",
            "Sub",
            "To",
            "to@e.com",
            "Cc",
            "cc@e.com",
            "Bcc",
            "bcc@e.com",
            "RT",
            "rt@e.com",
        ),
        fixtures::get_item_with_extended_properties_response("id14", "tag", "val"),
        fixtures::get_item_with_body_response("id15", "Text", "Body"),
        fixtures::sync_folder_items_response("state5", "id16"),
        fixtures::sync_folder_items_with_changes_response("state6", "c2", "u2", "d2"),
        fixtures::sync_folder_items_paginated_response("state7", "state8", "id17"),
        fixtures::copy_item_response("id18"),
        fixtures::move_item_response("id19"),
        fixtures::copy_folder_response("id20"),
        fixtures::move_folder_response("id21"),
        fixtures::mark_as_junk_response(Some("id22")),
        fixtures::mark_all_items_as_read_response(),
        fixtures::update_item_response("id23"),
        fixtures::update_item_set_field_response("id24"),
        fixtures::update_item_append_field_response("id25"),
        fixtures::update_item_delete_field_response("id26"),
        fixtures::update_item_in_recoverable_items_response("id26"),
        fixtures::update_folder_response("id27"),
        fixtures::send_item_response(),
        fixtures::create_item_send_response(),
        fixtures::find_folder_response("id28", "FindFolder"),
        fixtures::find_folder_paginated_response(&["id29"], &["F29"], 1, false),
        fixtures::find_item_response("id30"),
        fixtures::find_item_paginated_response(&["id31"], 1, false),
        fixtures::error_invalid_folder_id(),
        fixtures::error_folder_not_found(),
        fixtures::error_item_not_found(),
        fixtures::error_access_denied(),
        fixtures::error_server_busy(),
        fixtures::error_invalid_request(),
        fixtures::error_authentication_failed(),
        fixtures::error_quota_exceeded(),
        fixtures::error_insufficient_resources(),
        fixtures::error_mailbox_store_unavailable(),
        fixtures::error_folder_not_empty(),
        fixtures::error_invalid_change_key(),
        fixtures::error_invalid_sync_state(),
        fixtures::error_mark_as_junk_not_supported(),
        fixtures::batch_get_folder_mixed_response("s1", "e1"),
        fixtures::batch_get_item_response(&["i1"], &["s1"]),
        fixtures::batch_delete_item_response(1),
        fixtures::batch_delete_item_mixed_response(1, 1),
        fixtures::batch_update_item_response(&["i2"]),
        fixtures::batch_update_item_mixed_response(&["i3"], 1),
        fixtures::batch_mark_as_junk_response(&["i4"]),
        fixtures::batch_copy_item_response(&["i5"]),
        fixtures::batch_move_item_response(&["i6"]),
        fixtures::batch_copy_folder_response(&["f1"]),
        fixtures::batch_move_folder_response(&["f2"]),
        fixtures::batch_delete_folder_mixed_response(1, 1),
    ];

    for fixture in fixture_responses {
        assert_valid_soap_envelope(&fixture);
    }
}

#[tokio::test]
async fn test_mock_server_batch_response() {
    let mock = MockEwsServer::new().await;
    let response = fixtures::batch_delete_item_mixed_response(2, 1);
    mock.register_response(response).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(mock.ews_endpoint())
        .body("<m:DeleteItem>...</m:DeleteItem>")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(resp.status(), 200);
    let body = resp.text().await.expect("Failed to read body");
    assert_batch_response_mixed(&body, 2, 1);
}
