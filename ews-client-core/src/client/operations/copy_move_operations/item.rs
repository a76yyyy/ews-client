//! Item (message) copy and move operations

use crate::client::{EwsClient, EwsError};
use ews::{
    BaseFolderId, BaseItemId, CopyMoveItemData, ItemResponseMessage, OperationResponse, copy_item::CopyItem,
    move_item::MoveItem,
};

use super::{CopyMoveOperation, CopyMoveOperationBuilder};

impl CopyMoveOperation for MoveItem {
    fn response_to_ids(response: Vec<<Self::Response as OperationResponse>::Message>) -> Vec<String> {
        get_new_item_ids_from_response(response)
    }
}

impl CopyMoveOperation for CopyItem {
    fn response_to_ids(response: Vec<<Self::Response as OperationResponse>::Message>) -> Vec<String> {
        get_new_item_ids_from_response(response)
    }
}

impl CopyMoveOperationBuilder for MoveItem {
    fn operation_builder(client: &EwsClient, destination_id: String, ids: &[&str]) -> Self {
        // `ReturnNewItemIds` was introduced in Exchange Server 2010 SP1.
        // For older versions, we need to set it to None.
        let server_version = Some(client.server_version.load());
        let return_new_item_ids = match server_version {
            Some(v) if v <= ews::server_version::ExchangeServerVersion::Exchange2010 => None,
            _ => Some(true),
        };

        MoveItem {
            inner: CopyMoveItemData {
                to_folder_id: BaseFolderId::FolderId {
                    id: destination_id,
                    change_key: None,
                },
                item_ids: ids
                    .iter()
                    .map(|id| BaseItemId::ItemId {
                        id: (*id).to_string(),
                        change_key: None,
                    })
                    .collect(),
                return_new_item_ids,
            },
        }
    }
}

impl CopyMoveOperationBuilder for CopyItem {
    fn operation_builder(client: &EwsClient, destination_id: String, ids: &[&str]) -> Self {
        // `ReturnNewItemIds` was introduced in Exchange Server 2010 SP1.
        // For older versions, we need to set it to None.
        let server_version = Some(client.server_version.load());
        let return_new_item_ids = match server_version {
            Some(v) if v <= ews::server_version::ExchangeServerVersion::Exchange2010 => None,
            _ => Some(true),
        };

        CopyItem {
            inner: CopyMoveItemData {
                to_folder_id: BaseFolderId::FolderId {
                    id: destination_id,
                    change_key: None,
                },
                item_ids: ids
                    .iter()
                    .map(|id| BaseItemId::ItemId {
                        id: (*id).to_string(),
                        change_key: None,
                    })
                    .collect(),
                return_new_item_ids,
            },
        }
    }
}

impl EwsClient {
    /// Copies one or more messages to a destination folder.
    ///
    /// # Arguments
    ///
    /// * `destination_folder_id` - The EWS ID of the destination folder
    /// * `item_ids` - A slice of EWS message IDs to copy
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the newly copied messages
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The destination folder does not exist
    /// - Any source message does not exist
    /// - Network or authentication errors occur
    /// - The response count doesn't match the request count
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?, Credentials::basic("user", "pass"))?;
    /// let new_ids = client.copy_items("dest_folder_id", &["msg_id_1", "msg_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy_items(&self, destination_folder_id: &str, item_ids: &[&str]) -> Result<Vec<String>, EwsError> {
        self.copy_move_generic::<CopyItem>(destination_folder_id, item_ids)
            .await
    }

    /// Moves one or more messages to a destination folder.
    ///
    /// # Arguments
    ///
    /// * `destination_folder_id` - The EWS ID of the destination folder
    /// * `item_ids` - A slice of EWS message IDs to move
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the moved messages (typically the same as input IDs)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The destination folder does not exist
    /// - Any source message does not exist
    /// - Network or authentication errors occur
    /// - The response count doesn't match the request count
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?, Credentials::basic("user", "pass"))?;
    /// let ids = client.move_items("dest_folder_id", &["msg_id_1", "msg_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn move_items(&self, destination_folder_id: &str, item_ids: &[&str]) -> Result<Vec<String>, EwsError> {
        self.copy_move_generic::<MoveItem>(destination_folder_id, item_ids)
            .await
    }
}

fn get_new_item_ids_from_response(response: Vec<ItemResponseMessage>) -> Vec<String> {
    response
        .into_iter()
        .filter_map(|response_message| {
            response_message.items.inner.first().and_then(|item| {
                let message = item.inner_message();
                message.item_id.as_ref().map(|x| x.id.clone())
            })
        })
        .collect()
}
