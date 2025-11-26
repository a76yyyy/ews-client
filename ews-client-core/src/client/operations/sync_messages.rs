//! Sync messages in a folder.

use crate::client::{
    EwsClient, EwsError, OperationRequestOptions, process_response_message_class, single_response_or_error,
};
use ews::{
    BaseFolderId, BaseShape, ItemShape, Operation, OperationResponse,
    sync_folder_items::{self, SyncFolderItems},
};

/// Result of syncing messages in a folder
#[derive(Debug, Clone, Default)]
pub struct SyncMessagesResult {
    /// Message IDs that were created
    pub created: Vec<String>,
    /// Message IDs that were updated
    pub updated: Vec<String>,
    /// Message IDs that were deleted
    pub deleted: Vec<String>,
    /// Message IDs with read status changed
    pub read_status_changed: Vec<(String, bool)>,
    /// The new sync state token for the next sync
    pub sync_state: String,
    /// Whether there are more changes to fetch
    pub includes_last_item: bool,
}

/// Detailed message information from sync
#[derive(Debug, Clone)]
pub struct SyncMessageInfo {
    /// The EWS item ID of the message
    pub item_id: String,
    /// Whether the message has been read
    pub is_read: Option<bool>,
    /// The Internet message ID (RFC 2822 Message-ID header)
    pub internet_message_id: Option<String>,
    /// The date and time the message was sent (Unix timestamp)
    pub date_time_sent: Option<i64>,
    /// The sender's email address
    pub from: Option<String>,
    /// The message subject
    pub subject: Option<String>,
    /// Whether the message has attachments
    pub has_attachments: Option<bool>,
    /// The size of the message in bytes
    pub size: Option<usize>,
}

impl EwsClient {
    /// Synchronizes messages in a folder since the last sync state.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The EWS ID of the folder to sync
    /// * `sync_state` - The sync state token from the previous sync (None for initial sync)
    ///
    /// # Returns
    ///
    /// A `SyncMessagesResult` containing the changes and new sync state
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The folder does not exist
    /// - The sync state is invalid
    /// - Network or authentication errors occur
    /// - The server returns an unexpected response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?, Credentials::basic("user", "pass"))?;
    /// let result = client.sync_messages("folder_id", None).await?;
    /// println!("Created: {:?}", result.created);
    /// println!("Updated: {:?}", result.updated);
    /// println!("Deleted: {:?}", result.deleted);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sync_messages(
        &self,
        folder_id: &str,
        sync_state: Option<String>,
    ) -> Result<SyncMessagesResult, EwsError> {
        // Track the final state of each message ID
        // We iterate over changes in chronological order, so the last change wins
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum MessageState {
            Created,
            Updated,
            Deleted,
        }

        let mut all_result = SyncMessagesResult::default();
        let mut current_sync_state = sync_state;
        let mut message_states: std::collections::HashMap<String, MessageState> = std::collections::HashMap::new();
        let mut read_status_map: std::collections::HashMap<String, bool> = std::collections::HashMap::new();

        loop {
            let op = SyncFolderItems {
                item_shape: ItemShape {
                    // Microsoft's guidance is that the sync call should only
                    // fetch IDs for server load reasons.
                    // See <https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/how-to-synchronize-items-by-using-ews-in-exchange>
                    base_shape: BaseShape::IdOnly,
                    ..Default::default()
                },
                sync_folder_id: BaseFolderId::FolderId {
                    id: folder_id.to_string(),
                    change_key: None,
                },
                sync_state: current_sync_state.clone(),
                ignore: None,
                max_changes_returned: 100,
                sync_scope: None,
            };

            let response = self
                .make_operation_request(op, OperationRequestOptions::default())
                .await?;

            let response_messages = response.into_response_messages();

            let response_class = single_response_or_error(response_messages)?;

            let message = process_response_message_class(SyncFolderItems::NAME, response_class)?;

            // Process each change in chronological order
            // The last change for a given message ID wins
            for change in message.changes.inner {
                match change {
                    sync_folder_items::Change::Create { item } => {
                        if let Some(item_id) = &item.inner_message().item_id {
                            message_states.insert(item_id.id.clone(), MessageState::Created);
                        }
                    }
                    sync_folder_items::Change::Update { item } => {
                        if let Some(item_id) = &item.inner_message().item_id {
                            message_states.insert(item_id.id.clone(), MessageState::Updated);
                        }
                    }
                    sync_folder_items::Change::Delete { item_id } => {
                        message_states.insert(item_id.id, MessageState::Deleted);
                    }
                    sync_folder_items::Change::ReadFlagChange { item_id, is_read } => {
                        // Use HashMap to track the latest read status for each message
                        read_status_map.insert(item_id.id, is_read);
                    }
                }
            }

            // Update sync state
            all_result.sync_state = message.sync_state;
            all_result.includes_last_item = message.includes_last_item_in_range;

            // If we've reached the end, break the loop
            if message.includes_last_item_in_range {
                break;
            }

            // Otherwise, prepare for the next iteration
            current_sync_state = Some(all_result.sync_state.clone());
        }

        // Convert the final states to vectors
        // Only include each message ID once, based on its final state
        for (id, state) in message_states {
            match state {
                MessageState::Created => all_result.created.push(id),
                MessageState::Updated => all_result.updated.push(id),
                MessageState::Deleted => all_result.deleted.push(id),
            }
        }

        all_result.read_status_changed = read_status_map.into_iter().collect();

        Ok(all_result)
    }
}
