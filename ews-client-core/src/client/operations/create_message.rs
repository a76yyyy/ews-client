//! Create a message via EWS.

use crate::client::{EwsClient, EwsError, MSGFLAG_READ, MSGFLAG_UNMODIFIED, MSGFLAG_UNSENT};
use base64::prelude::{BASE64_STANDARD, Engine};
use ews::{
    BaseFolderId, ExtendedFieldURI, ExtendedProperty, Message, MessageDisposition, MimeContent, PropertyType, RealItem,
    create_item::CreateItem,
};

/// Result of creating a message on the server
#[derive(Debug, Clone)]
pub struct CreateMessageResult {
    /// The EWS ID of the newly created message
    pub item_id: String,
}

impl EwsClient {
    /// Creates a message on the server.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The EWS ID of the folder to create the message in
    /// * `content` - The MIME content of the message (all headers included)
    /// * `is_draft` - Whether the message is a draft
    /// * `is_read` - Whether the message should be marked as read
    ///
    /// # Returns
    ///
    /// The EWS ID of the newly created message
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The folder does not exist
    /// - The MIME content is invalid
    /// - Network or authentication errors occur
    /// - The server returns an unexpected response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::{EwsClient, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?, Credentials::basic("user", "pass"))?;
    /// let mime_content = b"From: user@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";
    /// let result = client.create_message("folder_id", mime_content, false, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_message(
        &self,
        folder_id: &str,
        content: &[u8],
        is_draft: bool,
        is_read: bool,
    ) -> Result<CreateMessageResult, EwsError> {
        // Create a new message from the binary content
        let mut message = Message {
            mime_content: Some(MimeContent {
                character_set: None,
                content: BASE64_STANDARD.encode(content),
            }),
            is_read: Some(is_read),
            ..Default::default()
        };

        // Set the PR_MESSAGE_FLAGS MAPI property
        // If not set, the EWS server uses MSGFLAG_UNSENT | MSGFLAG_UNMODIFIED as default,
        // which is not what we want.
        //
        // See: https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtagmessageflags-canonical-property
        let mut mapi_flags = MSGFLAG_READ;
        if is_draft {
            mapi_flags |= MSGFLAG_UNSENT;
        } else {
            mapi_flags |= MSGFLAG_UNMODIFIED;
        }

        message.extended_property = Some(vec![ExtendedProperty {
            extended_field_URI: ExtendedFieldURI {
                distinguished_property_set_id: None,
                property_set_id: None,
                property_name: None,
                property_id: None,
                // 3591 (0x0E07) is the PR_MESSAGE_FLAGS MAPI property
                property_tag: Some("3591".into()),
                property_type: PropertyType::Integer,
            },
            value: mapi_flags.to_string(),
        }]);

        let create_item = CreateItem {
            items: vec![RealItem::Message(message)],
            message_disposition: Some(MessageDisposition::SaveOnly),
            saved_item_folder_id: Some(BaseFolderId::FolderId {
                id: folder_id.to_string(),
                change_key: None,
            }),
        };

        let response = self.make_create_item_request(create_item).await?;

        // Extract the item ID from the response
        let items = response.items.inner;
        if items.len() != 1 {
            return Err(EwsError::Processing {
                message: format!("expected exactly one item in CreateItem response, got {}", items.len()),
            });
        }

        let item = items.first().ok_or_else(|| EwsError::Processing {
            message: "no item in CreateItem response".to_string(),
        })?;
        let message = item.inner_message();

        let item_id = message
            .item_id
            .as_ref()
            .ok_or(EwsError::MissingIdInResponse)?
            .id
            .clone();

        Ok(CreateMessageResult { item_id })
    }
}
