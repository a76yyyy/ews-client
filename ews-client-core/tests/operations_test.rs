//! Integration tests for core EWS operations
//!
//! These tests verify that the operations compile and have the correct signatures.
//! Actual integration tests with a real EWS server would require credentials and
//! a test environment.

#[cfg(test)]
mod tests {
    use ews_client_core::client::{Credentials, EwsClient};
    use url::Url;

    /// Helper to create a test client (won't actually connect)
    fn create_test_client() -> Result<EwsClient, Box<dyn std::error::Error>> {
        let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
        let credentials = Credentials::basic("test@example.com", "password");
        Ok(EwsClient::new(endpoint, credentials)?)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        assert!(client.is_ok());
    }

    #[test]
    fn test_check_connectivity_signature() {
        // This test just verifies the method exists and has the right signature
        let client = create_test_client().unwrap();
        let _future = client.check_connectivity();
        // We don't actually await it since we don't have a real server
    }

    #[test]
    fn test_sync_folder_hierarchy_signature() {
        // This test just verifies the method exists and has the right signature
        let client = create_test_client().unwrap();
        let _future = client.sync_folder_hierarchy(None);
        // We don't actually await it since we don't have a real server
    }

    #[test]
    fn test_get_message_signature() {
        // This test just verifies the method exists and has the right signature
        let client = create_test_client().unwrap();
        let _future = client.get_message("test-id");
        // We don't actually await it since we don't have a real server
    }

    #[test]
    fn test_is_office365() {
        let client = create_test_client().unwrap();
        assert!(client.is_office365());

        // Test with a non-Office365 URL
        let endpoint = Url::parse("https://mail.example.com/EWS/Exchange.asmx").unwrap();
        let credentials = Credentials::basic("test@example.com", "password");
        let client = EwsClient::new(endpoint, credentials).unwrap();
        assert!(!client.is_office365());
    }
}
