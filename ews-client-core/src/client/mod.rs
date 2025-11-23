mod credentials;
mod error;
mod headers;
pub mod operations;
mod types;

pub use credentials::Credentials;
pub use error::EwsError;
pub use headers::{Mailbox, MessageHeaders, MessagePriority, make_header_string_for_mailbox_list};
pub use operations::{CreateMessageResult, FolderHierarchySyncResult, FolderInfo, SyncMessageInfo, SyncMessagesResult};
pub use types::*;

use std::collections::VecDeque;

use ews::{
    BaseFolderId, BaseItemId, BaseShape, Folder, FolderId, FolderShape, ItemShape, Operation, OperationResponse,
    PathToElement, RealItem, response::ResponseClass,
};
use reqwest::Client;
use url::Url;

/// The root folder ID for EWS mailbox
pub(crate) const EWS_ROOT_FOLDER: &str = "msgfolderroot";

/// The base domains for Office365-hosted accounts. At the time of writing, the
/// only valid domain for Office365 EWS URLs should be `outlook.office365.com`,
/// but we'll throw a few additional Microsoft-owned ones in there in case it
/// changes in the future, as well as anything ending with a `microsoft` TLD.
pub(crate) const OFFICE365_BASE_DOMAINS: [&str; 4] = ["office365.com", "outlook.com", "onmicrosoft.com", ".microsoft"];

/// Options to control the behavior of [`EwsClient::make_operation_request`].
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct OperationRequestOptions {
    /// Behavior to follow when an authentication failure arises.
    pub auth_failure_behavior: AuthFailureBehavior,

    /// Behavior to follow when a transport security failure arises.
    pub transport_sec_failure_behavior: TransportSecFailureBehavior,
}

/// The behavior to follow when an operation request results in an
/// authentication failure.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum AuthFailureBehavior {
    /// Attempt to re-authenticate or fail immediately.
    ///
    /// In a pure library context (without UI), this just means we propagate
    /// the error immediately without retrying.
    #[default]
    ReAuth,

    /// Fail immediately without attempting to authenticate again.
    /// This is useful for operations like connectivity checks where we want
    /// to know if authentication failed without triggering retries.
    Silent,
}

/// The behavior to follow when an operation request results in a transport
/// security failure (e.g. because of an invalid certificate). This specifically
/// controls the behaviour of `EwsClient::make_operation_request`.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum TransportSecFailureBehavior {
    /// Immediately alert the user about the security failure.
    ///
    /// In a pure library context (without UI), this just means we log the error
    /// and propagate it.
    #[default]
    Alert,

    /// Don't alert the user and propagate the failure to the consumer (which
    /// might or might not alert the user).
    Silent,
}

// Flags to use for setting the `PR_MESSAGE_FLAGS` MAPI property.
//
// See
// <https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtagmessageflags-canonical-property>,
// although the specific values are set in `Mapidefs.h` from the Windows SDK:
// <https://github.com/microsoft/MAPIStubLibrary/blob/1d30c31ebf05ef444371520cd4268d6e1fda8a3b/include/MAPIDefS.h#L2143-L2154>
//
// Message flags are of type `PT_LONG`, which corresponds to i32 (signed 32-bit
// integers) according to
// https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/property-types
pub(crate) const MSGFLAG_READ: i32 = 0x00000001;
pub(crate) const MSGFLAG_UNMODIFIED: i32 = 0x00000002;
pub(crate) const MSGFLAG_UNSENT: i32 = 0x00000008;

/// Pure Rust async EWS client
pub struct EwsClient {
    endpoint: Url,
    credentials: Credentials,
    client: Client,
    /// The Exchange Server version detected from the server
    pub(crate) server_version: std::sync::Arc<std::sync::Mutex<ews::server_version::ExchangeServerVersion>>,
}

impl EwsClient {
    /// Create a new EWS client
    pub fn new(endpoint: Url, credentials: Credentials) -> Result<Self, EwsError> {
        Ok(Self {
            endpoint,
            credentials,
            client: Client::new(),
            server_version: std::sync::Arc::new(std::sync::Mutex::new(
                ews::server_version::ExchangeServerVersion::Exchange2013_SP1,
            )),
        })
    }

    /// Get the endpoint URL
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Get a reference to the HTTP client
    pub(crate) fn http_client(&self) -> &Client {
        &self.client
    }

    /// Get a reference to the credentials
    pub(crate) fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Updates the server version from a ServerVersionInfo header
    pub(crate) fn update_server_version(&self, header: ews::server_version::ServerVersionInfo) -> Result<(), EwsError> {
        let version = match header.version {
            Some(version) if !version.is_empty() => version,
            // If the server did not include a version identifier, there's nothing to do
            _ => return Ok(()),
        };

        let version = match ews::server_version::ExchangeServerVersion::try_from(version.as_str()) {
            Ok(version) => version,
            // If the server included an unknown version, default to the most recent known version
            Err(_) => ews::server_version::ExchangeServerVersion::Exchange2013_SP1,
        };

        // Update the in-memory representation
        if let Ok(mut v) = self.server_version.lock() {
            *v = version;
        }

        Ok(())
    }

    /// Check if the endpoint is an Office365 server
    pub fn is_office365(&self) -> bool {
        self.endpoint
            .host_str()
            .map(|domain| {
                OFFICE365_BASE_DOMAINS
                    .iter()
                    .any(|o365_domain| domain.ends_with(o365_domain))
            })
            .unwrap_or(false)
    }

    /// Makes a request to the EWS endpoint to perform an operation.
    ///
    /// This is the core method that handles:
    /// - Serializing the operation to SOAP XML
    /// - Adding authentication headers
    /// - Sending the HTTP request
    /// - Deserializing the response
    /// - Handling errors and retries (for throttling)
    ///
    /// If the entire request or first response is throttled, the request will
    /// be repeatedly retried (after the delay given in the response) until it
    /// succeeds or some other error occurs.
    pub(crate) async fn make_operation_request<Op>(
        &self,
        op: Op,
        options: OperationRequestOptions,
    ) -> Result<Op::Response, EwsError>
    where
        Op: ews::Operation,
    {
        let op_name = <Op as Operation>::NAME;

        // Create SOAP envelope with the operation
        let envelope = ews::soap::Envelope {
            headers: vec![ews::soap::Header::RequestServerVersion {
                version: ews::server_version::ExchangeServerVersion::Exchange2013_SP1,
            }],
            body: op,
        };

        // Serialize to XML
        let request_body = envelope.as_xml_document()?;

        // Loop in case we need to retry the request after a delay
        loop {
            // Send the authenticated request
            let response = match self.send_authenticated_request(&request_body, op_name).await {
                Ok(response) => response,
                Err(err) => {
                    // Handle authentication, network and transport security
                    // failures early because we know how to process them
                    // without requiring more data from the response body.
                    match err {
                        // If the error is an authentication failure, check if we should
                        // retry based on the options. In a pure library context (without UI),
                        // we just propagate the error.
                        EwsError::Authentication
                            if matches!(options.auth_failure_behavior, AuthFailureBehavior::ReAuth) =>
                        {
                            // In ews_xpcom, this would prompt the user for new credentials
                            // and retry. In a pure library, we just log and return the error.
                            log::error!("Authentication failed for operation {}", op_name);
                            return Err(err);
                        }

                        // If auth_failure_behavior is Silent, fail immediately
                        EwsError::Authentication => {
                            log::debug!("Authentication failed for operation {} (silent mode)", op_name);
                            return Err(err);
                        }

                        // For HTTP errors, check if it's a transport security failure
                        // and handle according to options
                        EwsError::Http(ref http_err) => {
                            // In ews_xpcom, this would check for TransportSecurityFailure
                            // and potentially show a certificate error dialog.
                            // In a pure library, we just log based on the behavior setting.
                            match options.transport_sec_failure_behavior {
                                TransportSecFailureBehavior::Alert => {
                                    log::error!("HTTP/Transport error during operation {}: {:?}", op_name, http_err);
                                }
                                TransportSecFailureBehavior::Silent => {
                                    log::debug!(
                                        "HTTP/Transport error during operation {} (silent mode): {:?}",
                                        op_name,
                                        http_err
                                    );
                                }
                            }
                            return Err(err);
                        }

                        _ => return Err(err),
                    }
                }
            };

            // Check HTTP status
            let status = response.status();
            if !status.is_success() {
                log::error!("Request FAILED with status {} for operation {}", status, op_name);
                return Err(EwsError::Processing {
                    message: format!("HTTP request failed with status: {}", status),
                });
            }

            // Get response body
            let response_body = response.bytes().await?;

            // Try to deserialize the response
            let op_result: Result<ews::soap::Envelope<Op::Response>, _> =
                ews::soap::Envelope::from_xml_document(&response_body);

            break match op_result {
                Ok(envelope) => {
                    // If the server responded with a version identifier, store it
                    for header in envelope.headers.iter() {
                        if let ews::soap::Header::ServerVersionInfo(server_version_info) = header {
                            if let Err(e) = self.update_server_version(server_version_info.clone()) {
                                log::warn!("Failed to update server version: {:?}", e);
                            }
                        }
                    }

                    // Check if the first response is a back off message, and retry if so
                    if let Some(ews::response::ResponseClass::Error(ews::response::ResponseError {
                        message_xml: Some(ews::MessageXml::ServerBusy(server_busy)),
                        ..
                    })) = envelope.body.response_messages().first()
                    {
                        let delay_ms = server_busy.back_off_milliseconds;
                        log::debug!(
                            "{} returned busy message, will retry after {} milliseconds",
                            op_name,
                            delay_ms
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms as u64)).await;
                        continue;
                    }

                    Ok(envelope.body)
                }
                Err(err) => {
                    // Check first to see if the request has been throttled and needs to be retried
                    let backoff_delay_ms = maybe_get_backoff_delay_ms(&err);
                    if let Some(backoff_delay_ms) = backoff_delay_ms {
                        log::debug!(
                            "{} request throttled, will retry after {} milliseconds",
                            op_name,
                            backoff_delay_ms
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(backoff_delay_ms as u64)).await;
                        continue;
                    }

                    // If not, propagate the error
                    Err(err.into())
                }
            };
        }
    }

    /// Send an authenticated EWS operation request with the given body.
    ///
    /// This method:
    /// - Gets the Authorization header value from credentials
    /// - Sends the POST request to the EWS endpoint
    /// - Returns the response or an error
    async fn send_authenticated_request(
        &self,
        request_body: &[u8],
        op_name: &str,
    ) -> Result<reqwest::Response, EwsError> {
        // Get the Authorization header value
        let auth_header_value = self.credentials.to_auth_header();

        // Log the request (for debugging)
        log::info!("Making operation request: {}", op_name);

        // Send the request
        let response = self
            .client
            .post(self.endpoint.clone())
            .header("Authorization", auth_header_value)
            .header("Content-Type", "text/xml; charset=utf-8")
            .body(request_body.to_vec())
            .send()
            .await?;

        let response_status = response.status();
        log::info!(
            "Response received for operation {} (status {})",
            op_name,
            response_status
        );

        // Check for authentication errors (401)
        if response_status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(EwsError::Authentication);
        }

        Ok(response)
    }

    /// Fetches items from the remote Exchange server in batches.
    ///
    /// This method handles batching of GetItem requests (max 10 items per request)
    /// to avoid throttling.
    pub(crate) async fn get_items<IdColl>(
        &self,
        ids: IdColl,
        fields: &[&str],
        include_mime_content: bool,
    ) -> Result<Vec<RealItem>, EwsError>
    where
        IdColl: IntoIterator<Item = String>,
    {
        let mut ids: VecDeque<_> = ids.into_iter().collect();
        let mut items = Vec::with_capacity(ids.len());

        while !ids.is_empty() {
            // Batch items in chunks of 10 per Microsoft's recommendation
            let batch_ids: Vec<_> = {
                let range_end = usize::min(10, ids.len());
                ids.drain(0..range_end)
            }
            .map(|id| BaseItemId::ItemId { id, change_key: None })
            .collect();

            let additional_properties: Vec<_> = fields
                .iter()
                .map(|&field| PathToElement::FieldURI {
                    field_URI: String::from(field),
                })
                .collect();

            let additional_properties = if additional_properties.is_empty() {
                None
            } else {
                Some(additional_properties)
            };

            let op = ews::get_item::GetItem {
                item_shape: ItemShape {
                    base_shape: BaseShape::IdOnly,
                    additional_properties,
                    include_mime_content: Some(include_mime_content),
                },
                item_ids: batch_ids,
            };

            let response = self.make_operation_request(op, Default::default()).await?;
            for response_message in response.into_response_messages() {
                let message = process_response_message_class("GetItem", response_message)?;

                let items_len = message.items.inner.len();
                if items_len != 1 {
                    log::warn!("GetItemResponseMessage contained {} items, only 1 expected", items_len);
                }

                items.extend(message.items.inner.into_iter());
            }
        }

        Ok(items)
    }

    /// Fetches folders from the remote Exchange server in batches.
    ///
    /// This method handles batching of GetFolder requests and filters out
    /// non-mail folders.
    pub(crate) async fn batch_get_folders(&self, ids: Vec<String>) -> Result<Vec<Folder>, EwsError> {
        let mut folders = Vec::with_capacity(ids.len());
        let mut ids = ids.into_iter().peekable();
        let mut buf = Vec::with_capacity(10);

        loop {
            // Batch folders in chunks of 10 per Microsoft's recommendation
            for _ in 0..10 {
                match ids.next() {
                    Some(value) => buf.push(value),
                    None => break,
                }
            }

            let to_fetch = buf
                .drain(..)
                .map(|id| BaseFolderId::FolderId { id, change_key: None })
                .collect();

            let op = ews::get_folder::GetFolder {
                folder_shape: FolderShape {
                    base_shape: BaseShape::AllProperties,
                },
                folder_ids: to_fetch,
            };

            let response = self.make_operation_request(op, Default::default()).await?;
            let messages = response.into_response_messages();

            let mut fetched = messages
                .into_iter()
                .filter_map(|response_class| {
                    let message = match process_response_message_class("GetFolder", response_class) {
                        Ok(message) => message,
                        Err(err) => return Some(Err(err)),
                    };
                    if let Err(err) = validate_get_folder_response_message(&message) {
                        return Some(Err(err));
                    }

                    message
                        .folders
                        .inner
                        .into_iter()
                        .next()
                        .and_then(|folder| match &folder {
                            Folder::Folder {
                                folder_class,
                                display_name,
                                ..
                            } => {
                                let folder_class = folder_class.as_ref().map(|s| s.as_str());

                                // Filter out non-mail folders
                                match folder_class {
                                    Some(folder_class) => {
                                        if folder_class == "IPF.Note" || folder_class.starts_with("IPF.Note.") {
                                            Some(Ok(folder))
                                        } else {
                                            log::debug!("Skipping folder with unsupported class: {}", folder_class);
                                            None
                                        }
                                    }
                                    None => {
                                        log::warn!(
                                            "Skipping folder without a class: {}",
                                            display_name.clone().unwrap_or("unknown".to_string())
                                        );
                                        None
                                    }
                                }
                            }
                            _ => None,
                        })
                })
                .collect::<Result<_, _>>()?;

            folders.append(&mut fetched);

            if ids.peek().is_none() {
                break;
            }
        }

        Ok(folders)
    }

    /// Performs a CreateItem operation and processes its response.
    pub(crate) async fn make_create_item_request(
        &self,
        create_item: ews::create_item::CreateItem,
    ) -> Result<ews::ItemResponseMessage, EwsError> {
        self.make_create_item_request_with_options(create_item, Default::default())
            .await
    }

    /// Performs a CreateItem operation with custom options and processes its response.
    pub(crate) async fn make_create_item_request_with_options(
        &self,
        create_item: ews::create_item::CreateItem,
        transport_sec_failure_behavior: TransportSecFailureBehavior,
    ) -> Result<ews::ItemResponseMessage, EwsError> {
        let response = self
            .make_operation_request(
                create_item,
                OperationRequestOptions {
                    transport_sec_failure_behavior,
                    ..Default::default()
                },
            )
            .await?;

        let response_messages = response.into_response_messages();

        // We have only sent one message, therefore the response should only
        // contain one response message.
        let response_message = single_response_or_error(response_messages)?;
        process_response_message_class(ews::create_item::CreateItem::NAME, response_message)
    }

    /// Performs an UpdateItem operation and processes its response.
    pub(crate) async fn make_update_item_request(
        &self,
        update_item: ews::update_item::UpdateItem,
    ) -> Result<ews::update_item::UpdateItemResponse, EwsError> {
        let expected_response_count = update_item.item_changes.len();

        let response = self.make_operation_request(update_item, Default::default()).await?;

        let response_messages = response.response_messages();

        validate_response_message_count(response_messages, expected_response_count)?;

        Ok(response)
    }
}

/// Gets the time to wait before retrying a throttled request, if any.
///
/// When an Exchange server throttles a request, the response will specify a
/// delay which should be observed before the request is retried.
fn maybe_get_backoff_delay_ms(err: &ews::Error) -> Option<u32> {
    if let ews::Error::RequestFault(fault) = err {
        let message_xml = fault.as_ref().detail.as_ref()?.message_xml.as_ref()?;

        match message_xml {
            ews::MessageXml::ServerBusy(server_busy) => Some(server_busy.back_off_milliseconds),
            _ => None,
        }
    } else {
        None
    }
}

/// Look at the response class of a response message, and do nothing, warn or
/// return an error accordingly.
pub(crate) fn process_response_message_class<T>(
    op_name: &str,
    response_class: ResponseClass<T>,
) -> Result<T, EwsError> {
    match response_class {
        ResponseClass::Success(message) => Ok(message),

        ResponseClass::Warning(message) => {
            log::warn!("{} operation encountered unknown warning", op_name);
            Ok(message)
        }

        ResponseClass::Error(err) => Err(err.to_owned().into()),
    }
}

/// Verifies that a response message for a GetFolder request is valid for a
/// standard folder.
///
/// Returns the ID of a valid folder for convenience.
pub(crate) fn validate_get_folder_response_message(
    message: &ews::get_folder::GetFolderResponseMessage,
) -> Result<FolderId, EwsError> {
    if message.folders.inner.len() != 1 {
        return Err(EwsError::Processing {
            message: format!(
                "expected exactly one folder per response message, got {}",
                message.folders.inner.len()
            ),
        });
    }

    // Okay to unwrap as we've verified the length.
    match message.folders.inner.first().unwrap() {
        Folder::Folder { folder_id, .. } => folder_id.clone().ok_or(EwsError::MissingIdInResponse),

        _ => Err(EwsError::Processing {
            message: String::from("expected folder to be of type Folder"),
        }),
    }
}

/// Validates that the response contains the expected number of messages.
pub(crate) fn validate_response_message_count<T>(
    response_messages: &[ResponseClass<T>],
    expected_len: usize,
) -> Result<(), EwsError> {
    if response_messages.len() != expected_len {
        return Err(EwsError::Processing {
            message: format!(
                "response contained an unexpected number of response messages: expected {}, got {}",
                expected_len,
                response_messages.len()
            ),
        });
    }

    Ok(())
}

/// For responses where we expect a single message, extract that message.
///
/// Returns an error if no messages are available, prints a warning but successfully
/// returns the first message if more than one message is available.
pub(crate) fn single_response_or_error<T>(responses: Vec<T>) -> Result<T, EwsError> {
    let responses_len = responses.len();
    let Some(message) = responses.into_iter().next() else {
        return Err(EwsError::Processing {
            message: "expected 1 response message, got none".to_string(),
        });
    };
    if responses_len != 1 {
        log::warn!("expected 1 response message, got {}", responses_len);
    }
    Ok(message)
}
