//! Send a message via EWS.

use crate::client::{EwsClient, EwsError};
use base64::prelude::{BASE64_STANDARD, Engine};
use ews::{
    ArrayOfRecipients, Message, MessageDisposition, MimeContent, Operation, OperationResponse, Recipient,
    create_item::CreateItem,
};
impl EwsClient {
    /// Sends a message via EWS.
    ///
    /// # Arguments
    ///
    /// * `mime_content` - The MIME content of the message (all headers included)
    /// * `message_id` - The Internet Message ID for the message
    /// * `should_request_dsn` - Whether to request a delivery status notification
    /// * `bcc_recipients` - A slice of BCC recipients for the message
    ///
    /// # Returns
    ///
    /// Ok(()) if the message was sent successfully
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The MIME content is invalid
    /// - Network or authentication errors occur
    /// - The server returns an unexpected response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ews_client_core::client::EwsClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EwsClient::new("https://outlook.office365.com/EWS/Exchange.asmx".parse()?)?;
    /// let mime_content = "From: user@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";
    /// client.send_message(mime_content, "message-id@example.com", false, &[]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(
        &self,
        mime_content: &str,
        message_id: &str,
        should_request_dsn: bool,
        bcc_recipients: &[Recipient],
    ) -> Result<(), EwsError> {
        // Create the BCC recipients array if there are any
        let bcc_recipients_array = if bcc_recipients.is_empty() {
            None
        } else {
            Some(ArrayOfRecipients(bcc_recipients.to_vec()))
        };

        // Create a new message with the MIME content
        let message = Message {
            mime_content: Some(MimeContent {
                character_set: None,
                content: BASE64_STANDARD.encode(mime_content),
            }),
            is_delivery_receipt_requested: Some(should_request_dsn),
            internet_message_id: Some(message_id.to_string()),
            bcc_recipients: bcc_recipients_array,
            ..Default::default()
        };

        // Create the CreateItem operation with SendOnly disposition
        // This tells EWS to send the message immediately without saving to Sent folder
        // (the client is responsible for saving to Sent folder separately)
        let create_item = CreateItem {
            items: vec![ews::RealItem::Message(message)],
            message_disposition: Some(MessageDisposition::SendOnly),
            saved_item_folder_id: None,
        };

        // Send the request
        // Use Silent mode for transport security failures, as the caller should handle them
        let response = self
            .make_operation_request(
                create_item,
                crate::client::OperationRequestOptions {
                    transport_sec_failure_behavior: crate::client::TransportSecFailureBehavior::Silent,
                    ..Default::default()
                },
            )
            .await?;

        // Validate the response
        let response_messages = response.into_response_messages();

        // We expect exactly one response message for a single message send
        if response_messages.len() != 1 {
            return Err(EwsError::Processing {
                message: format!(
                    "expected exactly one response message for SendMessage, got {}",
                    response_messages.len()
                ),
            });
        }

        // Process the response and check for errors
        let response_message = response_messages
            .into_iter()
            .next()
            .ok_or_else(|| EwsError::Processing {
                message: "no response message for SendMessage".to_string(),
            })?;
        crate::client::process_response_message_class(CreateItem::NAME, response_message)?;

        Ok(())
    }
}
