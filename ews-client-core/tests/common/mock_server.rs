#![allow(dead_code)]
//! Mock EWS server implementation using wiremock
//!
//! Provides a mock HTTP server that simulates EWS (Exchange Web Services) responses.
//! This allows testing the EWS client without requiring a real Exchange server.

use super::fixtures;
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A mock EWS server for testing
///
/// This server simulates an EWS endpoint and can be configured to return
/// specific SOAP responses for different operations.
///
/// The server automatically resets between tests when dropped.
pub struct MockEwsServer {
    server: MockServer,
}

const EWS_PATH: &str = "/EWS/Exchange.asmx";

impl MockEwsServer {
    /// Create a new mock EWS server
    ///
    /// The server will listen on a random available port.
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    /// Get the base URL of the mock server
    ///
    /// # Example
    /// ```ignore
    /// let mock = MockEwsServer::new().await;
    /// let url = mock.url(); // e.g., "http://127.0.0.1:12345"
    /// ```
    pub fn url(&self) -> String {
        self.server.uri()
    }

    /// Get the EWS endpoint URL
    pub fn ews_endpoint(&self) -> String {
        format!("{}{}", self.url(), EWS_PATH)
    }

    /// Register a mock response for EWS operations
    ///
    /// Returns immediately after mounting the mock.
    pub async fn register_response(&self, response_body: String) {
        self.register_response_with_status(200, response_body).await;
    }

    /// Register a mock response for `GetFolder` operation
    pub async fn mock_get_folder(&self, folder_id: &str, display_name: &str) {
        let response = fixtures::get_folder_response(folder_id, display_name);
        self.register_operation("GetFolder", response).await;
    }

    /// Register a mock response for `CreateFolder` operation
    pub async fn mock_create_folder(&self, folder_id: &str) {
        let response = fixtures::create_folder_response(folder_id);
        self.register_operation("CreateFolder", response).await;
    }

    /// Register a mock response for `DeleteFolder` operation
    pub async fn mock_delete_folder(&self) {
        let response = fixtures::delete_folder_response();
        self.register_operation("DeleteFolder", response).await;
    }

    /// Register a mock response for `GetItem` operation
    pub async fn mock_get_item(&self, item_id: &str, subject: &str) {
        let response = fixtures::get_item_response(item_id, subject);
        self.register_operation("GetItem", response).await;
    }

    /// Register a mock response for `CreateItem` operation
    pub async fn mock_create_item(&self, item_id: &str) {
        let response = fixtures::create_item_response(item_id);
        self.register_operation("CreateItem", response).await;
    }

    /// Register a mock response for `DeleteItem` operation
    pub async fn mock_delete_item(&self) {
        let response = fixtures::delete_item_response();
        self.register_operation("DeleteItem", response).await;
    }

    /// Register a mock response for `UpdateItem` operation
    pub async fn mock_update_item(&self, item_id: &str) {
        let response = fixtures::update_item_response(item_id);
        self.register_operation("UpdateItem", response).await;
    }

    /// Register a mock response for `SendItem` operation
    pub async fn mock_send_item(&self) {
        let response = fixtures::send_item_response();
        self.register_operation("SendItem", response).await;
    }

    /// Register a mock response for `FindItem` operation
    pub async fn mock_find_item(&self, item_id: &str) {
        let response = fixtures::find_item_response(item_id);
        self.register_operation("FindItem", response).await;
    }

    /// Register a mock response for `FindFolder` operation
    pub async fn mock_find_folder(&self, folder_id: &str, display_name: &str) {
        let response = fixtures::find_folder_response(folder_id, display_name);
        self.register_operation("FindFolder", response).await;
    }

    /// Register a mock response for `SyncFolderHierarchy` operation
    pub async fn mock_sync_folder_hierarchy(&self, sync_state: &str, folder_id: &str) {
        let response = fixtures::sync_folder_hierarchy_response(sync_state, folder_id);
        self.register_operation("SyncFolderHierarchy", response).await;
    }

    /// Register a mock response for `SyncFolderItems` operation
    pub async fn mock_sync_folder_items(&self, sync_state: &str, item_id: &str) {
        let response = fixtures::sync_folder_items_response(sync_state, item_id);
        self.register_operation("SyncFolderItems", response).await;
    }

    /// Helper to register an operation-specific response
    async fn register_operation(&self, operation: &str, response_body: String) {
        Mock::given(method("POST"))
            .and(path(EWS_PATH))
            .and(body_string_contains(format!("<m:{operation}>")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(response_body)
                    .append_header("Content-Type", "text/xml; charset=utf-8"),
            )
            .mount(&self.server)
            .await;
    }

    /// Register a response with custom status code
    pub async fn register_response_with_status(&self, status_code: u16, response_body: String) {
        Mock::given(method("POST"))
            .and(path(EWS_PATH))
            .respond_with(
                ResponseTemplate::new(status_code)
                    .set_body_string(response_body)
                    .append_header("Content-Type", "text/xml; charset=utf-8"),
            )
            .mount(&self.server)
            .await;
    }

    /// Register an authentication error response (401 Unauthorized)
    pub async fn register_auth_error(&self) {
        Mock::given(method("POST"))
            .and(path(EWS_PATH))
            .respond_with(ResponseTemplate::new(401).append_header("WWW-Authenticate", "Basic realm=\"test\""))
            .mount(&self.server)
            .await;
    }

    /// Register a server error response (500 Internal Server Error)
    pub async fn register_server_error(&self) {
        Mock::given(method("POST"))
            .and(path(EWS_PATH))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&self.server)
            .await;
    }

    /// Register a connection timeout response (504 Gateway Timeout)
    pub async fn register_timeout(&self) {
        Mock::given(method("POST"))
            .and(path(EWS_PATH))
            .respond_with(ResponseTemplate::new(504).set_body_string("Gateway Timeout"))
            .mount(&self.server)
            .await;
    }

    /// Reset all registered mocks
    pub async fn reset(&self) {
        self.server.reset().await;
    }
}

// Note: Default is not implemented because MockEwsServer::new() is async
// Use MockEwsServer::new().await instead

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_server_creation() {
        let mock = MockEwsServer::new().await;
        let url = mock.url();
        assert!(url.starts_with("http://"));
    }

    #[tokio::test]
    async fn test_ews_endpoint_url() {
        let mock = MockEwsServer::new().await;
        let endpoint = mock.ews_endpoint();
        assert!(endpoint.ends_with("/EWS/Exchange.asmx"));
    }

    #[tokio::test]
    async fn test_mock_get_folder() {
        let mock = MockEwsServer::new().await;
        mock.mock_get_folder("folder-id", "Test Folder").await;

        let client = reqwest::Client::new();
        let response = client
            .post(mock.ews_endpoint())
            .body(r"<m:GetFolder>...</m:GetFolder>")
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body = response.text().await.expect("Failed to read body");
        assert!(body.contains("folder-id"));
        assert!(body.contains("Test Folder"));
    }
}
