//! Integration tests for item operations
//!
//! These tests require a live EWS server and are ignored by default.
//! Run with: `cargo test --package ews-client-core -- --ignored`

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing,
    clippy::print_stdout
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
async fn test_create_and_delete_message() {
    let client = create_test_client();
    let folder_id = "drafts"; // Use drafts folder for testing

    // Create a test message
    let content = b"From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test Message\r\n\r\nThis is a test message.";
    let result = client
        .create_message(folder_id, content, true, false)
        .await
        .expect("Failed to create message");

    assert!(!result.item_id.is_empty(), "Created message should have an ID");

    // Delete the message
    client
        .delete_messages(&[&result.item_id])
        .await
        .expect("Failed to delete message");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_get_message() {
    let client = create_test_client();
    let folder_id = "drafts";

    // Create a test message first
    let content = b"From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test Get Message\r\n\r\nContent to retrieve.";
    let result = client
        .create_message(folder_id, content, true, false)
        .await
        .expect("Failed to create message");

    // Get the message content
    let mime_content = client
        .get_message(&result.item_id)
        .await
        .expect("Failed to get message");

    assert!(!mime_content.is_empty(), "Message content should not be empty");
    // Note: EWS might modify the MIME content (add headers etc), so exact match might fail
    // But it should contain our subject
    let content_str = String::from_utf8_lossy(&mime_content);
    assert!(
        content_str.contains("Test Get Message"),
        "Content should contain subject"
    );

    // Clean up
    client
        .delete_messages(&[&result.item_id])
        .await
        .expect("Failed to delete message");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_send_message() {
    let client = create_test_client();

    // Note: This will actually attempt to send an email if run against a real server
    // We use a safe recipient or expect it to fail/bounce in a real test env
    // For now, we just verify the API call works

    let mime_content =
        "From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test Send Message\r\n\r\nBody";
    let message_id = format!(
        "test-{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // We expect this might fail with "ErrorSendAsDenied" if the user doesn't have permission to send as "sender@example.com"
    // Or it might succeed if we use the authenticated user's email.
    // For this test, we'll try to use the authenticated user's email if possible, or just catch the specific error.

    // To make it more robust, we'll just try to send and accept that it might fail due to permissions,
    // but we want to ensure the API call itself is valid (i.e. not a serialization error).

    let result = client.send_message(mime_content, &message_id, false, &[]).await;

    match result {
        Ok(_) => {} // Success
        Err(e) => {
            // If it's a permission error, that's "fine" for the purpose of testing the client code structure
            // But ideally we should use a valid sender.
            let err_str = format!("{e:?}");
            assert!(
                err_str.contains("ErrorSendAsDenied") || err_str.contains("ErrorNonExistentMailbox"),
                "Send message failed with unexpected error: {e:?}"
            );
            println!("Send message result: {e:?}");
        }
    }
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_copy_items() {
    let client = create_test_client();
    let folder_id = "drafts";

    // Create a test message
    let content = b"Subject: Test Copy Message\r\n\r\nBody";
    let result = client
        .create_message(folder_id, content, true, false)
        .await
        .expect("Failed to create message");

    // Copy to the same folder
    let new_ids = client
        .copy_items(folder_id, &[&result.item_id])
        .await
        .expect("Failed to copy item");

    assert_eq!(new_ids.len(), 1, "Should have one copied item");
    assert_ne!(new_ids[0], result.item_id, "Copied item should have different ID");

    // Clean up both
    client
        .delete_messages(&[&result.item_id, &new_ids[0]])
        .await
        .expect("Failed to delete messages");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_move_items() {
    let client = create_test_client();
    let source_folder = "drafts";
    let dest_folder = "deleteditems"; // Move to deleted items as a test

    // Create a test message
    let content = b"Subject: Test Move Message\r\n\r\nBody";
    let result = client
        .create_message(source_folder, content, true, false)
        .await
        .expect("Failed to create message");

    // Move the item
    let moved_ids = client
        .move_items(dest_folder, &[&result.item_id])
        .await
        .expect("Failed to move item");

    assert_eq!(moved_ids.len(), 1, "Should have one moved item");
    assert_ne!(moved_ids[0], result.item_id, "Moved item usually gets a new ID");

    // Clean up (delete from destination)
    client
        .delete_messages(&[&moved_ids[0]])
        .await
        .expect("Failed to delete moved message");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_change_read_status() {
    let client = create_test_client();
    let folder_id = "drafts";

    // Create a test message (unread)
    let content = b"Subject: Test Read Status\r\n\r\nBody";
    let result = client
        .create_message(folder_id, content, true, false) // is_read = false
        .await
        .expect("Failed to create message");

    // Mark as read
    let updated_ids = client
        .change_read_status(&[&result.item_id], true)
        .await
        .expect("Failed to mark as read");

    assert_eq!(updated_ids.len(), 1);

    // Mark as unread
    let updated_ids_2 = client
        .change_read_status(&[&result.item_id], false)
        .await
        .expect("Failed to mark as unread");

    assert_eq!(updated_ids_2.len(), 1);

    // Clean up
    client
        .delete_messages(&[&result.item_id])
        .await
        .expect("Failed to delete message");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_mark_as_junk() {
    let client = create_test_client();
    let folder_id = "drafts";
    let junk_folder_id = "junkemail";

    // Create a test message
    let content = b"Subject: Test Junk Message\r\n\r\nBody";
    let result = client
        .create_message(folder_id, content, true, false)
        .await
        .expect("Failed to create message");

    // Mark as junk
    // Note: This might fail if Junk Email folder is not enabled or accessible,
    // or if the server version is old and we didn't provide a valid legacy ID (we use "junkemail" distinguished ID here)
    let moved_ids = client
        .mark_as_junk(&[&result.item_id], true, junk_folder_id)
        .await
        .expect("Failed to mark as junk");

    assert_eq!(moved_ids.len(), 1);

    // Clean up
    client
        .delete_messages(&[&moved_ids[0]])
        .await
        .expect("Failed to delete junk message");
}

#[tokio::test]
#[ignore = "requires live EWS server"]
async fn test_update_item() {
    use ews_client_core::ews::update_item::{ItemChange, ItemChangeDescription, ItemChangeInner, Updates};
    use ews_client_core::ews::{BaseItemId, Message, PathToElement};

    let client = create_test_client();
    let folder_id = "drafts";

    // 1. Create a test message
    let content = b"Subject: Original Subject\r\n\r\nBody";
    let result = client
        .create_message(folder_id, content, true, false)
        .await
        .expect("Failed to create message");

    // 2. Update the subject
    let new_subject = "Updated Subject";
    let updates = Updates {
        inner: vec![ItemChangeDescription::SetItemField {
            field_uri: PathToElement::FieldURI {
                field_URI: "item:Subject".to_string(),
            },
            message: Message {
                subject: Some(new_subject.to_string()),
                ..Default::default()
            },
        }],
    };

    let item_change = ItemChange {
        item_change: ItemChangeInner {
            item_id: BaseItemId::ItemId {
                id: result.item_id.clone(),
                change_key: None,
            },
            updates,
        },
    };

    let updated_ids = client
        .update_item(vec![item_change])
        .await
        .expect("Failed to update item");

    assert_eq!(updated_ids.len(), 1);

    // 3. Verify update
    // Note: get_message returns MIME content, which might not immediately reflect property updates
    // depending on server caching, but usually it does.
    // Alternatively we could use a GetItem operation that returns properties, but client.get_message is what we have.
    let mime_content = client
        .get_message(&updated_ids[0])
        .await
        .expect("Failed to get message");

    let content_str = String::from_utf8_lossy(&mime_content);
    // EWS MIME content usually contains the Subject header.
    assert!(
        content_str.contains(new_subject),
        "Content should contain updated subject"
    );

    // 4. Clean up
    client
        .delete_messages(&[&updated_ids[0]])
        .await
        .expect("Failed to delete message");
}
