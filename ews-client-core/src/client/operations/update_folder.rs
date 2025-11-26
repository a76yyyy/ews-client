//! Update a folder's display name.

use crate::client::{
    EwsClient, EwsError, OperationRequestOptions, process_response_message_class, single_response_or_error,
};
use ews::{
    BaseFolderId, Folder, Operation, OperationResponse, PathToElement,
    update_folder::{FolderChange, FolderChanges, UpdateFolder, Updates},
};

impl EwsClient {
    /// Updates a folder's display name.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The EWS ID of the folder to update
    /// * `folder_name` - The new display name for the folder
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The folder does not exist
    /// - Network or authentication errors occur
    /// - The server returns an unexpected response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?, Credentials::basic("user", "pass"))?;
    /// client.update_folder("folder_id", "New Name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_folder(&self, folder_id: &str, folder_name: &str) -> Result<(), EwsError> {
        let update_folder = UpdateFolder {
            folder_changes: FolderChanges {
                folder_change: FolderChange {
                    folder_id: BaseFolderId::FolderId {
                        id: folder_id.to_string(),
                        change_key: None,
                    },
                    updates: Updates::SetFolderField {
                        field_URI: PathToElement::FieldURI {
                            field_URI: "folder:DisplayName".to_string(),
                        },
                        folder: Folder::Folder {
                            display_name: Some(folder_name.to_string()),
                            folder_id: None,
                            parent_folder_id: None,
                            folder_class: None,
                            total_count: None,
                            child_folder_count: None,
                            extended_property: None,
                            unread_count: None,
                        },
                    },
                },
            },
        };

        let response = self
            .make_operation_request(update_folder, OperationRequestOptions::default())
            .await?;
        let response_messages = response.into_response_messages();
        let response_message = single_response_or_error(response_messages)?;
        process_response_message_class(UpdateFolder::NAME, response_message)?;

        Ok(())
    }
}

// Note: Integration tests for this module should be placed in the tests/ directory
// and should be marked with #[ignore] by default to avoid requiring a live EWS server for CI.
// Example:
// #[tokio::test]
// #[ignore]
// async fn test_update_folder_integration() {
//     let client = EwsClient::new(...);
//     let result = client.update_folder("folder_id", "new_name").await;
//     assert!(result.is_ok());
// }
