//! Mark messages as junk or not junk.

use crate::client::{EwsClient, EwsError, process_response_message_class, validate_response_message_count};
use ews::{BaseItemId, Operation, OperationResponse, mark_as_junk::MarkAsJunk};

impl EwsClient {
    /// Marks one or more messages as junk or not junk.
    ///
    /// Uses the MarkAsJunk operation (Exchange 2013+).
    /// For older versions, falls back to moving items to the junk folder.
    ///
    /// # Arguments
    ///
    /// * `item_ids` - A slice of EWS message IDs to mark
    /// * `is_junk` - Whether to mark as junk (true) or not junk (false)
    /// * `legacy_junk_folder_id` - The folder ID to move to for older Exchange versions
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the successfully updated messages
    pub async fn mark_as_junk(
        &self,
        item_ids: &[&str],
        is_junk: bool,
        legacy_junk_folder_id: &str,
    ) -> Result<Vec<String>, EwsError> {
        // The `MarkAsJunk` operation was added in Exchange 2013
        let server_version = Some(self.server_version.load());
        let use_mark_as_junk = server_version
            .map(|v| v >= ews::server_version::ExchangeServerVersion::Exchange2013)
            .unwrap_or(true); // Default to true if we can't determine version

        if use_mark_as_junk {
            // Try modern MarkAsJunk operation
            self.mark_as_junk_modern(item_ids, is_junk).await
        } else if !legacy_junk_folder_id.is_empty() {
            // Fall back to moving items for older Exchange versions
            self.move_items(legacy_junk_folder_id, item_ids).await
        } else {
            Err(EwsError::Processing {
                message: "Unable to mark as junk: MarkAsJunk operation not supported on this Exchange version and no legacy junk folder ID provided".to_string(),
            })
        }
    }

    async fn mark_as_junk_modern(&self, item_ids: &[&str], is_junk: bool) -> Result<Vec<String>, EwsError> {
        let item_ids_vec: Vec<BaseItemId> = item_ids
            .iter()
            .map(|id| BaseItemId::ItemId {
                id: id.to_string(),
                change_key: None,
            })
            .collect();

        let mark_as_junk = MarkAsJunk {
            is_junk,
            move_item: true,
            item_ids: item_ids_vec,
        };

        let response = self.make_operation_request(mark_as_junk, Default::default()).await?;

        let response_messages = response.into_response_messages();

        // Validate response count
        validate_response_message_count(&response_messages, item_ids.len())?;

        // Collect successfully updated message IDs
        let mut updated_ids = Vec::new();
        for response_message in response_messages {
            match process_response_message_class(MarkAsJunk::NAME, response_message) {
                Ok(message) => {
                    // Extract the moved item ID from the response
                    updated_ids.push(message.moved_item_id.id);
                }
                Err(err) => {
                    log::warn!("Error marking message as junk: {:?}", err);
                }
            }
        }

        Ok(updated_ids)
    }
}
