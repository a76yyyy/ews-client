//! Performs a connectivity check to the EWS server.

use crate::client::{
    AuthFailureBehavior, EWS_ROOT_FOLDER, EwsClient, EwsError, OperationRequestOptions, process_response_message_class,
    single_response_or_error, validate_get_folder_response_message,
};
use ews::{BaseFolderId, BaseShape, FolderShape, Operation, OperationResponse, get_folder::GetFolder};

impl EwsClient {
    /// Performs a connectivity check to the EWS server.
    ///
    /// Because EWS does not have a dedicated endpoint to test connectivity and
    /// authentication, we try to look up the ID of the account's root mail
    /// folder, since it produces a fairly small request and represents the
    /// first operation performed when adding a new account.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Authentication fails
    /// - Network connection fails
    /// - The server returns an error response
    /// - The root folder cannot be found
    pub async fn check_connectivity(&self) -> Result<(), EwsError> {
        // Request the EWS ID of the root folder
        let get_root_folder = GetFolder {
            folder_shape: FolderShape {
                base_shape: BaseShape::IdOnly,
            },
            folder_ids: vec![BaseFolderId::DistinguishedFolderId {
                id: EWS_ROOT_FOLDER.to_string(),
                change_key: None,
            }],
        };

        let response_messages = self
            // Make authentication failure silent, since all we want to know is
            // whether our credentials are valid.
            .make_operation_request(
                get_root_folder,
                OperationRequestOptions {
                    auth_failure_behavior: AuthFailureBehavior::Silent,
                    ..Default::default()
                },
            )
            .await?
            .into_response_messages();

        // Get the first (and only) response message so we can inspect it
        let response_class = single_response_or_error(response_messages)?;
        let message = process_response_message_class(GetFolder::NAME, response_class)?;

        // Any error fetching the root folder is fatal, since it likely means
        // all subsequent requests will fail, and that we won't manage to sync
        // the folder list later.
        validate_get_folder_response_message(&message)?;

        Ok(())
    }
}
