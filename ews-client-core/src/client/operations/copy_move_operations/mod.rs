pub mod folder;

use ews::{Operation, OperationResponse};

use crate::client::{EwsClient, EwsError, process_response_message_class};

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
                Default::default(),
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
