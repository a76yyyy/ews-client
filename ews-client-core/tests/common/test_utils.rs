#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(dead_code)]

//! Test utilities and helper functions
//!
//! Provides utilities for creating test clients, assertions, and data validation.

use ews_client_core::client::{Credentials, EwsClient};
use url::Url;

/// Create a test EWS client with the given endpoint
///
/// # Arguments
/// * `endpoint` - The EWS endpoint URL
///
/// # Panics
/// Panics if the endpoint URL is invalid or client creation fails
///
/// # Example
/// ```ignore
/// let client = create_test_client("http://localhost:3000/EWS/Exchange.asmx");
/// ```
pub fn create_test_client(endpoint: &str) -> EwsClient {
    let url = Url::parse(endpoint).expect("Invalid endpoint URL");
    let credentials = Credentials::Basic {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
    };

    EwsClient::new(url, credentials).expect("Failed to create test client")
}

/// Create a test EWS client with `OAuth2` credentials
///
/// # Arguments
/// * `endpoint` - The EWS endpoint URL
/// * `token` - The `OAuth2` bearer token
///
/// # Panics
/// Panics if the endpoint URL is invalid or client creation fails
pub fn create_test_client_oauth2(endpoint: &str, token: &str) -> EwsClient {
    let url = Url::parse(endpoint).expect("Invalid endpoint URL");
    let credentials = Credentials::OAuth2 {
        token: token.to_string(),
    };

    EwsClient::new(url, credentials).expect("Failed to create test client")
}

/// Generate a unique test folder ID
///
/// # Example
/// ```ignore
/// let folder_id = generate_test_folder_id();
/// assert!(folder_id.starts_with("folder-"));
/// ```
/// # Panics
/// Panics if the system time is before the UNIX epoch
pub fn generate_test_folder_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("folder-{timestamp}")
}

/// Generate a unique test item ID
///
/// # Example
/// ```ignore
/// let item_id = generate_test_item_id();
/// assert!(item_id.starts_with("item-"));
/// ```
/// # Panics
/// Panics if the system time is before the UNIX epoch
pub fn generate_test_item_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("item-{timestamp}")
}

/// Generate a unique test sync state token
/// # Panics
/// Panics if the system time is before the UNIX epoch
pub fn generate_test_sync_state() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("sync-state-{timestamp}")
}

/// Assert that a string contains a valid SOAP envelope
/// # Panics
/// Panics if the response is not a valid SOAP envelope
pub fn assert_valid_soap_envelope(response: &str) {
    assert!(response.contains("<?xml"), "Response should start with XML declaration");
    assert!(
        response.contains("<s:Envelope"),
        "Response should contain SOAP envelope"
    );
    assert!(
        response.contains("</s:Envelope>"),
        "Response should close SOAP envelope"
    );
}

/// Assert that a response contains a success response code
/// # Panics
/// Panics if the response does not contain a success code
pub fn assert_response_success(response: &str) {
    assert!(
        response.contains("NoError"),
        "Response should contain NoError response code"
    );
    assert!(
        response.contains("Success"),
        "Response should have Success response class"
    );
}

/// Assert that a response contains an error response code
/// # Panics
/// Panics if the response does not contain the expected error code
pub fn assert_response_error(response: &str, error_code: &str) {
    assert!(
        response.contains(error_code),
        "Response should contain error code: {error_code}"
    );
    assert!(response.contains("Error"), "Response should have Error response class");
}

/// Assert that a response contains a specific folder ID
/// # Panics
/// Panics if the response does not contain the expected folder ID
pub fn assert_contains_folder_id(response: &str, folder_id: &str) {
    assert!(
        response.contains(folder_id),
        "Response should contain folder ID: {folder_id}"
    );
}

/// Assert that a response contains a specific item ID
/// # Panics
/// Panics if the response does not contain the expected item ID
pub fn assert_contains_item_id(response: &str, item_id: &str) {
    assert!(response.contains(item_id), "Response should contain item ID: {item_id}");
}

/// Assert that a response contains a specific display name
/// # Panics
/// Panics if the response does not contain the expected display name
pub fn assert_contains_display_name(response: &str, display_name: &str) {
    assert!(
        response.contains(display_name),
        "Response should contain display name: {display_name}"
    );
}

/// Assert that a response contains a specific subject
/// # Panics
/// Panics if the response does not contain the expected subject
pub fn assert_contains_subject(response: &str, subject: &str) {
    assert!(response.contains(subject), "Response should contain subject: {subject}");
}

/// Assert that a response contains a sync state token
/// # Panics
/// Panics if the response does not contain the expected sync state
pub fn assert_contains_sync_state(response: &str, sync_state: &str) {
    assert!(
        response.contains(sync_state),
        "Response should contain sync state: {sync_state}"
    );
}

/// Assert that a response contains a SOAP fault
/// # Panics
/// Panics if the response does not contain the expected fault
pub fn assert_response_fault(response: &str, fault_code: &str, fault_string: &str) {
    assert!(response.contains("<s:Fault"), "Response should contain SOAP Fault");
    assert!(
        response.contains(fault_code),
        "Response should contain fault code: {fault_code}"
    );
    assert!(
        response.contains(fault_string),
        "Response should contain fault string: {fault_string}"
    );
}

/// Assert that a batch response contains the expected number of success messages
/// # Panics
/// Panics if the response does not contain the expected number of success messages
pub fn assert_batch_response_success(response: &str, count: usize) {
    let success_matches = response.matches("ResponseClass=\"Success\"").count();
    assert_eq!(
        success_matches, count,
        "Expected {count} success responses, found {success_matches}"
    );
    assert!(response.contains("NoError"), "Response should contain NoError");
}

/// Assert that a batch response contains the expected number of success and error messages
/// # Panics
/// Panics if the response does not contain the expected number of success/error messages
pub fn assert_batch_response_mixed(response: &str, success_count: usize, error_count: usize) {
    let success_matches = response.matches("ResponseClass=\"Success\"").count();
    let error_matches = response.matches("ResponseClass=\"Error\"").count();
    assert_eq!(
        success_matches, success_count,
        "Expected {success_count} success responses, found {success_matches}"
    );
    assert_eq!(
        error_matches, error_count,
        "Expected {error_count} error responses, found {error_matches}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_folder_id() {
        let id1 = generate_test_folder_id();

        assert!(id1.starts_with("folder-"));
        // Verify the ID contains a timestamp
        assert!(id1.len() > "folder-".len());
    }

    #[test]
    fn test_generate_test_item_id() {
        let id = generate_test_item_id();
        assert!(id.starts_with("item-"));
    }

    #[test]
    fn test_generate_test_sync_state() {
        let state = generate_test_sync_state();
        assert!(state.starts_with("sync-state-"));
    }

    #[test]
    fn test_assert_valid_soap_envelope() {
        let valid_response = r#"<?xml version="1.0"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
  <s:Body></s:Body>
</s:Envelope>"#;

        assert_valid_soap_envelope(valid_response);
    }

    #[test]
    fn test_assert_response_success() {
        let response = r#"<m:ResponseCode>NoError</m:ResponseCode>
<m:GetFolderResponseMessage ResponseClass="Success">"#;

        assert_response_success(response);
    }

    #[test]
    fn test_assert_response_error() {
        let response = r#"<m:ResponseCode>ErrorFolderNotFound</m:ResponseCode>
<m:GetFolderResponseMessage ResponseClass="Error">"#;

        assert_response_error(response, "ErrorFolderNotFound");
    }

    #[test]
    fn test_assert_contains_folder_id() {
        let response = r#"<t:FolderId Id="test-folder-123" />"#;
        assert_contains_folder_id(response, "test-folder-123");
    }

    #[test]
    fn test_assert_contains_item_id() {
        let response = r#"<t:ItemId Id="test-item-456" />"#;
        assert_contains_item_id(response, "test-item-456");
    }

    #[test]
    fn test_assert_contains_display_name() {
        let response = r"<t:DisplayName>My Folder</t:DisplayName>";
        assert_contains_display_name(response, "My Folder");
    }

    #[test]
    fn test_assert_contains_subject() {
        let response = r"<t:Subject>Test Email</t:Subject>";
        assert_contains_subject(response, "Test Email");
    }

    #[test]
    fn test_assert_contains_sync_state() {
        let response = r"<m:SyncState>sync-state-12345</m:SyncState>";
        assert_contains_sync_state(response, "sync-state-12345");
    }
}
