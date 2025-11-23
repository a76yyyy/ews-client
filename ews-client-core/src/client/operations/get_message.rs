use crate::client::{EwsClient, EwsError};
use base64::prelude::{BASE64_STANDARD, Engine};

impl EwsClient {
    /// Fetches a single message by its EWS ID and returns its MIME content.
    ///
    /// This method:
    /// - Fetches the message with MIME content included
    /// - Decodes the base64-encoded MIME content
    /// - Returns the raw message bytes
    ///
    /// # Arguments
    ///
    /// * `id` - The EWS item ID of the message to fetch
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The message cannot be found
    /// - The message has no MIME content
    /// - The MIME content is not validly base64 encoded
    /// - Network or authentication errors occur
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
    /// let mime_content = client.get_message("AAMkAD...").await?;
    /// println!("Message size: {} bytes", mime_content.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_message(&self, id: impl Into<String>) -> Result<Vec<u8>, EwsError> {
        let id = id.into();

        // Fetch the item with MIME content
        let items = self.get_items([id.clone()], &[], true).await?;

        if items.len() != 1 {
            return Err(EwsError::Processing {
                message: format!("provided single ID to GetItem operation, got {} responses", items.len()),
            });
        }

        // Extract the Internet Message Format content of the message from the response
        let item = items.into_iter().next().unwrap();
        let message = item.inner_message();

        let raw_mime = message.mime_content.as_ref().ok_or_else(|| EwsError::Processing {
            message: "item has no content".to_string(),
        })?;

        // EWS returns the content of the email base64-encoded on top of any
        // encoding within the message
        let mime_content = BASE64_STANDARD
            .decode(&raw_mime.content)
            .map_err(|_| EwsError::Processing {
                message: "MIME content for item is not validly base64 encoded".to_string(),
            })?;

        Ok(mime_content)
    }
}
