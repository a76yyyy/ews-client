//! Marks one or more messages as read or unread.

use crate::client::{EwsClient, EwsError, process_response_message_class, single_response_or_error};
use ews::{
    BaseItemId, Message, MessageDisposition, Operation, OperationResponse, PathToElement,
    update_item::{ConflictResolution, ItemChange, ItemChangeDescription, ItemChangeInner, UpdateItem, Updates},
};
impl EwsClient {
    /// Marks one or more messages as read or unread.
    ///
    /// # Arguments
    ///
    /// * `item_ids` - A slice of EWS message IDs to update
    /// * `is_read` - Whether to mark as read (true) or unread (false)
    ///
    /// # Returns
    ///
    /// A vector of EWS IDs for the successfully updated messages
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// let updated_ids = client.change_read_status(&["msg_id_1", "msg_id_2"], true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn change_read_status(&self, item_ids: &[&str], is_read: bool) -> Result<Vec<String>, EwsError> {
        let item_changes: Vec<ItemChange> = item_ids
            .iter()
            .map(|id| {
                let updates = Updates {
                    inner: vec![ItemChangeDescription::SetItemField {
                        field_uri: PathToElement::FieldURI {
                            field_URI: "message:IsRead".to_string(),
                        },
                        message: Message {
                            is_read: Some(is_read),
                            ..Default::default()
                        },
                    }],
                };

                ItemChange {
                    item_change: ItemChangeInner {
                        item_id: BaseItemId::ItemId {
                            id: id.to_string(),
                            change_key: None,
                        },
                        updates,
                    },
                }
            })
            .collect();

        let update_item = UpdateItem {
            item_changes,
            message_disposition: MessageDisposition::SaveOnly,
            // Use AlwaysOverwrite since we don't have change keys
            conflict_resolution: Some(ConflictResolution::AlwaysOverwrite),
        };

        let response = self.make_update_item_request(update_item).await?;

        let response_messages = response.into_response_messages();

        // Validate response count
        if response_messages.len() != item_ids.len() {
            return Err(EwsError::UnexpectedResponseMessageCount {
                expected: item_ids.len(),
                actual: response_messages.len(),
            });
        }

        // Partition responses into successes and errors
        let (successes, errors): (Vec<_>, Vec<_>) = response_messages
            .into_iter()
            .enumerate()
            .map(|(index, r)| (index, process_response_message_class(UpdateItem::NAME, r)))
            .partition(|(_, result)| result.is_ok());

        // Collect successfully updated message IDs
        let updated_ids: Vec<String> = successes
            .into_iter()
            .flat_map(|(_, success)| {
                let message = success.expect("partition should only populate this with okays");
                message.items.inner.into_iter()
            })
            .filter_map(|item| item.inner_message().item_id.as_ref().map(|id| id.id.clone()))
            .collect();

        // If there were errors, log them but still return the successful IDs
        if !errors.is_empty() {
            let num_errs = errors.len();
            let (index, ref first_err) = errors[0];
            let first_error = first_err
                .as_ref()
                .expect_err("partition should only populate this with errs");
            log::warn!(
                "change_read_status: {} of {} messages failed to update; first error (at index {}): {:?}",
                num_errs,
                item_ids.len(),
                index,
                first_error
            );
        }

        Ok(updated_ids)
    }

    /// Marks all messages in one or more folders as read or unread.
    ///
    /// # Arguments
    ///
    /// * `folder_ids` - A slice of EWS folder IDs to update
    /// * `is_read` - Whether to mark as read (true) or unread (false)
    /// * `suppress_read_receipts` - Whether to suppress read receipts
    ///
    /// # Returns
    ///
    /// Ok(()) if the operation succeeds
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// client.change_read_status_all(&["folder_id_1"], true, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn change_read_status_all(
        &self,
        folder_ids: &[&str],
        is_read: bool,
        suppress_read_receipts: bool,
    ) -> Result<(), EwsError> {
        use ews::BaseFolderId;
        use ews::mark_all_read::MarkAllItemsAsRead;

        // The `MarkAllItemsAsRead` operation was added in Exchange 2013
        let server_version = Some(self.server_version.load());
        if let Some(version) = server_version
            && version < ews::server_version::ExchangeServerVersion::Exchange2013
        {
            return Err(EwsError::Processing {
                message: format!(
                    "MarkAllItemsAsRead operation is not supported on Exchange version {:?}. Requires Exchange 2013 or later.",
                    version
                ),
            });
        }

        let folder_ids: Vec<BaseFolderId> = folder_ids
            .iter()
            .map(|id| BaseFolderId::FolderId {
                id: id.to_string(),
                change_key: None,
            })
            .collect();

        let mark_all_items = MarkAllItemsAsRead {
            read_flag: is_read,
            suppress_read_receipts,
            folder_ids,
        };

        let response = self.make_operation_request(mark_all_items, Default::default()).await?;

        let response_messages = response.into_response_messages();

        // Should have exactly one response message for this operation
        let response_class = single_response_or_error(response_messages)?;

        // Process the response
        process_response_message_class(MarkAllItemsAsRead::NAME, response_class)?;

        Ok(())
    }
}
