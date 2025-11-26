//! Sync folder hierarchy.

use ews::{
    BaseFolderId, BaseShape, Folder, FolderShape, OperationResponse,
    sync_folder_hierarchy::{self, SyncFolderHierarchy},
};
use std::collections::HashMap;

use crate::client::{
    EWS_ROOT_FOLDER, EwsClient, EwsError, OperationRequestOptions, process_response_message_class,
    single_response_or_error, validate_get_folder_response_message,
};

/// The result of a folder hierarchy sync operation.
#[derive(Debug, Clone)]
pub struct FolderHierarchySyncResult {
    /// The new sync state token to use for the next sync
    pub sync_state: String,
    /// Folders that were created since the last sync
    pub created_folders: Vec<FolderInfo>,
    /// Folders that were updated since the last sync
    pub updated_folders: Vec<FolderInfo>,
    /// IDs of folders that were deleted since the last sync
    pub deleted_folder_ids: Vec<String>,
    /// Map of well-known folder IDs to their distinguished names
    /// (e.g., "inbox", "deleteditems", "drafts", etc.)
    pub well_known_folders: Option<HashMap<String, String>>,
}

/// Information about a folder.
#[derive(Debug, Clone)]
pub struct FolderInfo {
    /// The EWS folder ID
    pub folder_id: String,
    /// The parent folder ID
    pub parent_folder_id: String,
    /// The display name of the folder
    pub display_name: String,
    /// The folder class (e.g., "IPF.Note" for mail folders)
    pub folder_class: Option<String>,
    /// Total number of items in the folder
    pub total_count: Option<u32>,
    /// Number of unread items in the folder
    pub unread_count: Option<u32>,
    /// Number of child folders
    pub child_folder_count: Option<u32>,
}

impl EwsClient {
    /// Performs a folder hierarchy sync operation via EWS.
    ///
    /// This will fetch a list of remote changes since the specified sync state,
    /// fetch any folder details needed for creating or updating local folders,
    /// and return the changes.
    ///
    /// On the first sync (when `sync_state` is `None`), this will also fetch
    /// the well-known folder map to identify special folders like inbox, trash, etc.
    ///
    /// # Arguments
    ///
    /// * `sync_state` - The sync state token from the previous sync, or `None` for the first sync
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network or authentication errors occur
    /// - The server returns an error response
    /// - Required folder information is missing from the response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # use url::Url;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new(
    ///     Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?,
    ///     Credentials::basic("user@example.com", "password"),
    /// )?;
    ///
    /// // First sync
    /// let result = client.sync_folder_hierarchy(None).await?;
    /// println!("Created {} folders", result.created_folders.len());
    ///
    /// // Subsequent sync
    /// let result = client.sync_folder_hierarchy(Some(result.sync_state)).await?;
    /// println!("Updated {} folders", result.updated_folders.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sync_folder_hierarchy(
        &self,
        sync_state: Option<String>,
    ) -> Result<FolderHierarchySyncResult, EwsError> {
        // If we have received no sync state, assume that this is the first time
        // syncing this account. In that case, we need to determine which
        // folders are "well-known" (e.g., inbox, trash, etc.) so we can flag them.
        let well_known_folders = if sync_state.is_none() {
            Some(self.get_well_known_folder_map().await?)
        } else {
            None
        };

        let mut all_created_ids = Vec::new();
        let mut all_updated_ids = Vec::new();
        let mut all_deleted_ids = Vec::new();
        let mut current_sync_state = sync_state;
        let mut final_sync_state;

        loop {
            // Folder sync returns results in batches, with sync state providing
            // the mechanism by which we can specify the next batch to receive.
            let op = SyncFolderHierarchy {
                folder_shape: FolderShape {
                    base_shape: BaseShape::IdOnly,
                },
                sync_folder_id: Some(BaseFolderId::DistinguishedFolderId {
                    // Folder sync can happen starting with any folder, but we
                    // always choose "msgfolderroot" as sync is recursive and
                    // this simplifies managing sync state. There is a "root"
                    // folder one level up as well, but it includes calendars,
                    // contacts, etc., which we aren't trying to support yet.
                    id: EWS_ROOT_FOLDER.to_string(),
                    change_key: None,
                }),
                sync_state: current_sync_state.clone(),
            };

            let response = self
                .make_operation_request(op, OperationRequestOptions::default())
                .await?
                .into_response_messages();

            let response = single_response_or_error(response)?;
            let message = process_response_message_class("SyncFolderHierarchy", response)?;

            // Build lists of all of the changed folder IDs
            for change in message.changes.inner {
                match change {
                    sync_folder_hierarchy::Change::Create { folder } => {
                        if let Folder::Folder { folder_id, .. } = folder {
                            let folder_id = folder_id.ok_or(EwsError::MissingIdInResponse)?;
                            all_created_ids.push(folder_id.id);
                        }
                    }
                    sync_folder_hierarchy::Change::Update { folder } => {
                        if let Folder::Folder { folder_id, .. } = folder {
                            let folder_id = folder_id.ok_or(EwsError::MissingIdInResponse)?;
                            all_updated_ids.push(folder_id.id);
                        }
                    }
                    sync_folder_hierarchy::Change::Delete { folder_id } => {
                        all_deleted_ids.push(folder_id.id);
                    }
                }
            }

            final_sync_state = message.sync_state;

            if message.includes_last_folder_in_range {
                // EWS has signaled to us that there are no more changes at this time
                break;
            }

            current_sync_state = Some(final_sync_state.clone());
        }

        // Remove any folders from the update list that were also deleted
        let deleted_set: std::collections::HashSet<_> = all_deleted_ids.iter().collect();
        all_updated_ids.retain(|id| !deleted_set.contains(id));

        // Fetch full details for created and updated folders
        let created_folders = if all_created_ids.is_empty() {
            Vec::new()
        } else {
            self.fetch_folder_details(all_created_ids).await?
        };

        let updated_folders = if all_updated_ids.is_empty() {
            Vec::new()
        } else {
            self.fetch_folder_details(all_updated_ids).await?
        };

        Ok(FolderHierarchySyncResult {
            sync_state: final_sync_state,
            created_folders,
            updated_folders,
            deleted_folder_ids: all_deleted_ids,
            well_known_folders,
        })
    }

    /// Builds a map from remote folder ID to distinguished folder ID.
    ///
    /// This allows translating from the folder ID returned by `GetFolder`
    /// calls and well-known IDs associated with special folders.
    async fn get_well_known_folder_map(&self) -> Result<HashMap<String, String>, EwsError> {
        const DISTINGUISHED_IDS: &[&str] = &[
            EWS_ROOT_FOLDER,
            "inbox",
            "deleteditems",
            "drafts",
            "outbox",
            "sentitems",
            "junkemail",
            // The `archive` distinguished id isn't documented at
            // https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid
            // but it does provide the Exchange account's archive folder when
            // requested, while the other documented `archive*` distinguished
            // ids result in folder not found errors.
            "archive",
        ];

        // We should always request the root folder first to simplify processing
        // the response below.
        assert_eq!(
            DISTINGUISHED_IDS.first(),
            Some(&EWS_ROOT_FOLDER),
            "expected first fetched folder to be root"
        );

        let ids = DISTINGUISHED_IDS
            .iter()
            .map(|id| BaseFolderId::DistinguishedFolderId {
                id: (*id).to_string(),
                change_key: None,
            })
            .collect();

        // Fetch all distinguished folder IDs at once, since we have few enough
        // that they fit within Microsoft's recommended batch size of ten.
        let op = ews::get_folder::GetFolder {
            folder_shape: FolderShape {
                base_shape: BaseShape::IdOnly,
            },
            folder_ids: ids,
        };

        let response = self
            .make_operation_request(op, OperationRequestOptions::default())
            .await?;

        let response_messages = response.into_response_messages();
        crate::client::validate_response_message_count(&response_messages, DISTINGUISHED_IDS.len())?;

        // We expect results from EWS to be in the same order as given in the
        // request. EWS docs aren't explicit about response ordering, but
        // responses don't contain another means of mapping requested ID to response.
        let mut message_iter = DISTINGUISHED_IDS.iter().zip(response_messages);

        // Record the root folder for messages before processing the other responses
        let (_, response_class) = message_iter.next().ok_or_else(|| EwsError::Processing {
            message: "no response for root folder".to_string(),
        })?;
        let message = process_response_message_class("GetFolder", response_class)?;

        // Any error fetching the root folder is fatal, since we can't correctly
        // set the parents of any folders it contains without knowing its ID.
        let _root_folder_id = validate_get_folder_response_message(&message)?;

        // Build the mapping for the remaining folders
        message_iter
            .filter_map(|(&distinguished_id, response_class)| {
                let message = match process_response_message_class("GetFolder", response_class) {
                    Ok(message) => Some(message),

                    // Not every Exchange account will have all queried
                    // well-known folders, so we skip any which were not found.
                    Err(EwsError::ResponseError(ref err))
                        if err.response_code == ews::response::ResponseCode::ErrorFolderNotFound =>
                    {
                        None
                    }

                    // Return any other error
                    Err(err) => {
                        return Some(Err(err));
                    }
                };

                message.map(|message| {
                    // Validate the message (and propagate any error) if it's not `None`
                    match validate_get_folder_response_message(&message) {
                        // Map from EWS folder ID to distinguished ID
                        Ok(folder_id) => Ok((folder_id.id, distinguished_id.to_string())),
                        Err(err) => Err(err),
                    }
                })
            })
            .collect()
    }

    /// Fetches full details for a list of folder IDs.
    async fn fetch_folder_details(&self, ids: Vec<String>) -> Result<Vec<FolderInfo>, EwsError> {
        let folders = self.batch_get_folders(ids).await?;

        folders
            .into_iter()
            .map(|folder| match folder {
                Folder::Folder {
                    folder_id,
                    parent_folder_id,
                    display_name,
                    folder_class,
                    total_count,
                    unread_count,
                    child_folder_count,
                    ..
                } => {
                    let folder_id = folder_id.ok_or(EwsError::MissingIdInResponse)?;
                    let parent_folder_id = parent_folder_id.ok_or(EwsError::MissingIdInResponse)?;
                    let display_name = display_name.ok_or_else(|| EwsError::Processing {
                        message: "folder missing display name".to_string(),
                    })?;

                    Ok(FolderInfo {
                        folder_id: folder_id.id,
                        parent_folder_id: parent_folder_id.id,
                        display_name,
                        folder_class,
                        total_count,
                        unread_count,
                        child_folder_count,
                    })
                }
                _ => Err(EwsError::Processing {
                    message: "unexpected folder type".to_string(),
                }),
            })
            .collect()
    }
}
