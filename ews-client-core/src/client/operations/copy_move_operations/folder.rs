//! Folder copy and move operations

use crate::client::{EwsClient, EwsError};
use ews::{
    BaseFolderId, CopyMoveFolderData, Folder, FolderResponseMessage, OperationResponse, copy_folder::CopyFolder,
    move_folder::MoveFolder,
};

use super::{CopyMoveOperation, CopyMoveOperationBuilder};

impl CopyMoveOperation for MoveFolder {
    fn response_to_ids(response: Vec<<Self::Response as OperationResponse>::Message>) -> Vec<String> {
        get_new_folder_ids_from_response(response)
    }
}

impl CopyMoveOperation for CopyFolder {
    fn response_to_ids(response: Vec<<Self::Response as OperationResponse>::Message>) -> Vec<String> {
        get_new_folder_ids_from_response(response)
    }
}

impl CopyMoveOperationBuilder for MoveFolder {
    fn operation_builder(_client: &EwsClient, destination_id: String, ids: &[&str]) -> Self {
        MoveFolder {
            inner: CopyMoveFolderData {
                to_folder_id: BaseFolderId::FolderId {
                    id: destination_id,
                    change_key: None,
                },
                folder_ids: ids
                    .iter()
                    .map(|id| BaseFolderId::FolderId {
                        id: id.to_string(),
                        change_key: None,
                    })
                    .collect(),
            },
        }
    }
}

impl CopyMoveOperationBuilder for CopyFolder {
    fn operation_builder(_client: &EwsClient, destination_id: String, ids: &[&str]) -> Self {
        CopyFolder {
            inner: CopyMoveFolderData {
                to_folder_id: BaseFolderId::FolderId {
                    id: destination_id,
                    change_key: None,
                },
                folder_ids: ids
                    .iter()
                    .map(|id| BaseFolderId::FolderId {
                        id: id.to_string(),
                        change_key: None,
                    })
                    .collect(),
            },
        }
    }
}

impl EwsClient {
    /// Copies one or more folders to a destination folder.
    ///
    /// # Arguments
    ///
    /// * `destination_folder_id` - The EWS ID of the destination folder
    /// * `folder_ids` - A slice of EWS folder IDs to copy
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the newly copied folders
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// let new_ids = client.copy_folders("dest_folder_id", &["folder_id_1", "folder_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy_folders(
        &self,
        destination_folder_id: &str,
        folder_ids: &[&str],
    ) -> Result<Vec<String>, EwsError> {
        self.copy_move_generic::<CopyFolder>(destination_folder_id, folder_ids)
            .await
    }

    /// Moves one or more folders to a destination folder.
    ///
    /// # Arguments
    ///
    /// * `destination_folder_id` - The EWS ID of the destination folder
    /// * `folder_ids` - A slice of EWS folder IDs to move
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the moved folders (typically the same as input IDs)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// let ids = client.move_folders("dest_folder_id", &["folder_id_1", "folder_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn move_folders(
        &self,
        destination_folder_id: &str,
        folder_ids: &[&str],
    ) -> Result<Vec<String>, EwsError> {
        self.copy_move_generic::<MoveFolder>(destination_folder_id, folder_ids)
            .await
    }
}

fn get_new_folder_ids_from_response(response: Vec<FolderResponseMessage>) -> Vec<String> {
    response
        .into_iter()
        .filter_map(|response_message| {
            response_message.folders.inner.first().and_then(|folder| match folder {
                Folder::Folder { folder_id, .. } => folder_id.as_ref().map(|x| x.id.clone()),
                _ => None,
            })
        })
        .collect()
}
