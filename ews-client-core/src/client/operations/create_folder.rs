use crate::client::{EwsClient, EwsError, process_response_message_class, single_response_or_error};
use ews::{BaseFolderId, Folder, Operation, OperationResponse, create_folder::CreateFolder};

impl EwsClient {
    /// Creates a new folder in the specified parent folder.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The EWS ID of the parent folder
    /// * `name` - The display name for the new folder
    ///
    /// # Returns
    ///
    /// The EWS ID of the newly created folder
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// let folder_id = client.create_folder("parent_folder_id", "New Folder").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_folder(&self, parent_id: &str, name: &str) -> Result<String, EwsError> {
        let op = CreateFolder {
            parent_folder_id: BaseFolderId::FolderId {
                id: parent_id.to_string(),
                change_key: None,
            },
            folders: vec![Folder::Folder {
                folder_id: None,
                parent_folder_id: None,
                folder_class: Some("IPF.Note".to_string()),
                display_name: Some(name.to_string()),
                total_count: None,
                child_folder_count: None,
                extended_property: None,
                unread_count: None,
            }],
        };

        let response = self.make_operation_request(op, Default::default()).await?;

        // Validate the response
        let response_messages = response.into_response_messages();
        let response_class = single_response_or_error(response_messages)?;
        let response_message = process_response_message_class(CreateFolder::NAME, response_class)?;

        let folders = response_message.folders.inner;
        if folders.len() != 1 {
            return Err(EwsError::Processing {
                message: format!("expected exactly one folder in response, got {}", folders.len()),
            });
        }

        let folder_id = match folders.into_iter().next().unwrap() {
            Folder::Folder { folder_id, .. } => match folder_id {
                Some(folder_id) => folder_id.id,
                None => return Err(EwsError::MissingIdInResponse),
            },

            _ => {
                return Err(EwsError::Processing {
                    message: "created folder of unexpected type".to_string(),
                });
            }
        };

        Ok(folder_id)
    }
}

// Note: Integration tests for this module should be placed in the tests/ directory
// and should be marked with #[ignore] by default to avoid requiring a live EWS server for CI.
// Example:
// #[tokio::test]
// #[ignore]
// async fn test_create_folder_integration() {
//     let client = EwsClient::new(...);
//     let folder_id = client.create_folder("parent_id", "Test Folder").await.unwrap();
//     assert!(!folder_id.is_empty());
// }
