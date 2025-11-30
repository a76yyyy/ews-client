//! Update item properties.

use crate::client::{EwsClient, EwsError, process_response_message_class};
use ews::{
    MessageDisposition, Operation, OperationResponse,
    update_item::{ConflictResolution, ItemChange, UpdateItem},
};

impl EwsClient {
    /// Updates one or more items.
    ///
    /// # Arguments
    ///
    /// * `item_changes` - A list of changes to apply to items
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the successfully updated items
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network or authentication errors occur
    /// - The server returns an unexpected response
    pub async fn update_item(&self, item_changes: Vec<ItemChange>) -> Result<Vec<String>, EwsError> {
        let update_item = UpdateItem {
            item_changes,
            message_disposition: MessageDisposition::SaveOnly,
            conflict_resolution: Some(ConflictResolution::AlwaysOverwrite),
        };

        let response = self.make_update_item_request(update_item).await?;

        let response_messages = response.into_response_messages();

        // Partition responses into successes and errors
        let (successes, errors): (Vec<_>, Vec<_>) = response_messages
            .into_iter()
            .enumerate()
            .map(|(index, r)| (index, process_response_message_class(UpdateItem::NAME, r)))
            .partition(|(_, result)| result.is_ok());

        // Collect successfully updated message IDs
        let updated_ids: Vec<String> = successes
            .into_iter()
            .flat_map(|(_, success)| match success {
                Ok(message) => message.items.inner,
                Err(_) => vec![],
            })
            .filter_map(|item| item.inner_message().item_id.as_ref().map(|id| id.id.clone()))
            .collect();

        // If there were errors, log them
        if !errors.is_empty() {
            let num_errs = errors.len();
            if let Some((index, first_err)) = errors.first()
                && let Err(first_error) = first_err
            {
                log::warn!("update_item: {num_errs} updates failed; first error (at index {index}): {first_error:?}");
            }
        }

        Ok(updated_ids)
    }
}
