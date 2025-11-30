//! Copy and move operations for folders and items

pub mod folder;

pub mod item;

use ews::{BaseFolderId, Operation, OperationResponse};

use crate::client::{EwsClient, EwsError, OperationRequestOptions, process_response_message_class};

/// Trait for EWS operations that copy or move folders or items.
pub trait CopyMoveOperation: Operation + Clone {
    /// Maps from the EWS response object to the collection of EWS IDs for the
    /// moved/copied objects.
    fn response_to_ids(response: Vec<<Self::Response as OperationResponse>::Message>) -> Vec<String>;
}

impl EwsClient {
    /// Performs a generic copy or move operation.
    pub(crate) async fn copy_move_generic<Op>(
        &self,
        destination_id: &str,
        ids: &[&str],
    ) -> Result<Vec<String>, EwsError>
    where
        Op: CopyMoveOperation + CopyMoveOperationBuilder,
    {
        let response = self
            .make_operation_request(
                Op::operation_builder(self, destination_id.to_string(), ids),
                OperationRequestOptions::default(),
            )
            .await?;

        let response_messages = response.into_response_messages();

        // Validate response count
        if response_messages.len() != ids.len() {
            return Err(EwsError::UnexpectedResponseMessageCount {
                expected: ids.len(),
                actual: response_messages.len(),
            });
        }

        let messages = response_messages
            .into_iter()
            .map(|response_class| process_response_message_class(<Op as Operation>::NAME, response_class))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Op::response_to_ids(messages))
    }
}

/// Extension trait for building copy/move operations.
pub trait CopyMoveOperationBuilder: CopyMoveOperation {
    /// Builds the operation from the given parameters.
    fn operation_builder(client: &EwsClient, destination_id: String, ids: &[&str]) -> Self;
}

pub(crate) fn create_base_folder_id(id: String) -> BaseFolderId {
    if is_distinguished_folder_id(&id) {
        BaseFolderId::DistinguishedFolderId { id, change_key: None }
    } else {
        BaseFolderId::FolderId { id, change_key: None }
    }
}

fn is_distinguished_folder_id(id: &str) -> bool {
    matches!(
        id,
        "calendar"
            | "contacts"
            | "deleteditems"
            | "drafts"
            | "inbox"
            | "journal"
            | "notes"
            | "outbox"
            | "sentitems"
            | "tasks"
            | "msgfolderroot"
            | "root"
            | "junkemail"
            | "searchfolders"
            | "voicemail"
            | "recoverableitemsroot"
            | "recoverableitemsdeletions"
            | "recoverableitemsversions"
            | "recoverableitemspurges"
            | "archiveroot"
            | "archivemsgfolderroot"
            | "archivedeleteditems"
            | "archiveinbox"
            | "archiverecoverableitemsroot"
            | "archiverecoverableitemsdeletions"
            | "archiverecoverableitemsversions"
            | "archiverecoverableitemspurges"
            | "syncissues"
            | "conflicts"
            | "localfailures"
            | "serverfailures"
            | "recipientcache"
            | "quickcontacts"
            | "conversationhistory"
            | "adminauditlogs"
            | "todosearch"
            | "mycontacts"
            | "directory"
            | "imcontactlist"
            | "peopleconnect"
            | "favorites"
    )
}
