use crate::client::{EwsClient, EwsError, process_response_message_class};
use ews::{
    BaseFolderId, DeleteType, Operation, OperationResponse,
    delete_folder::DeleteFolder,
    response::{ResponseCode, ResponseError},
};

impl EwsClient {
    /// Deletes one or more folders.
    ///
    /// # Arguments
    ///
    /// * `folder_ids` - A slice of EWS folder IDs to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// client.delete_folder(&["folder_id_1", "folder_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_folder(&self, folder_ids: &[&str]) -> Result<(), EwsError> {
        let folder_ids: Vec<BaseFolderId> = folder_ids
            .iter()
            .map(|id| BaseFolderId::FolderId {
                id: id.to_string(),
                change_key: None,
            })
            .collect();

        let delete_folder = DeleteFolder {
            delete_type: DeleteType::HardDelete,
            folder_ids,
        };

        let response = self.make_operation_request(delete_folder, Default::default()).await?;

        let response_messages = response.into_response_messages();
        for response_message in response_messages {
            if let Err(err) = process_response_message_class(DeleteFolder::NAME, response_message) {
                match err {
                    EwsError::ResponseError(ResponseError {
                        response_code: ResponseCode::ErrorItemNotFound,
                        ..
                    }) => {
                        // Folder was already deleted on the server, ignore the error
                        log::warn!("found folder that was deleted from the EWS server but not locally");
                    }
                    _ => return Err(err),
                }
            }
        }

        Ok(())
    }
}

// Note: Integration tests for this module should be placed in the tests/ directory
// and should be marked with #[ignore] by default to avoid requiring a live EWS server for CI.
// Example:
// #[tokio::test]
// #[ignore]
// async fn test_delete_folder_integration() {
//     let client = EwsClient::new(...);
//     let result = client.delete_folder(&["folder_id_1", "folder_id_2"]).await;
//     assert!(result.is_ok());
// }
