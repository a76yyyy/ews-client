//! Deletes one or more messages from the server.

use crate::client::{EwsClient, EwsError, process_response_message_class, validate_response_message_count};
use ews::{
    BaseItemId, DeleteType, Operation, OperationResponse,
    delete_item::DeleteItem,
    response::{ResponseCode, ResponseError},
};
impl EwsClient {
    /// Deletes one or more messages from the server.
    ///
    /// Messages that don't exist on the server are silently ignored (following
    /// the pattern from the reference implementation).
    ///
    /// # Arguments
    ///
    /// * `item_ids` - A slice of EWS message IDs to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the operation succeeds (even if some messages don't exist)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// client.delete_messages(&["msg_id_1", "msg_id_2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_messages(&self, item_ids: &[&str]) -> Result<(), EwsError> {
        let item_ids_vec: Vec<BaseItemId> = item_ids
            .iter()
            .map(|id| BaseItemId::ItemId {
                id: id.to_string(),
                change_key: None,
            })
            .collect();

        let delete_item = DeleteItem {
            item_ids: item_ids_vec,
            delete_type: DeleteType::HardDelete,
            send_meeting_cancellations: None,
            affected_task_occurrences: None,
            suppress_read_receipts: None,
        };

        let response = self.make_operation_request(delete_item, Default::default()).await?;

        let response_messages = response.into_response_messages();

        // Validate response count matches request count
        validate_response_message_count(&response_messages, item_ids.len())?;

        // Check each response message for errors, pairing with original IDs for better error messages
        response_messages
            .into_iter()
            .zip(item_ids.iter())
            .try_for_each(|(response_message, &ews_id)| {
                if let Err(err) = process_response_message_class(DeleteItem::NAME, response_message) {
                    if matches!(
                        err,
                        EwsError::ResponseError(ResponseError {
                            response_code: ResponseCode::ErrorItemNotFound,
                            ..
                        })
                    ) {
                        // The message was not found on the server. This can happen if:
                        // - The message was already deleted in a previous attempt
                        // - The message was deleted by another client
                        // - The message ID is stale or invalid
                        // We silently ignore this error to allow the caller to clean up
                        // their local state without failing the entire operation.
                        log::warn!(
                            "message not found on EWS server during delete (may have been already deleted): {}",
                            ews_id
                        );
                        Ok(())
                    } else {
                        Err(EwsError::Processing {
                            message: format!("error while attempting to delete message {}: {:?}", ews_id, err),
                        })
                    }
                } else {
                    Ok(())
                }
            })?;

        Ok(())
    }
}
