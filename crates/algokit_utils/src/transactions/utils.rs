use crate::transactions::TransactionError;
use algod_client::AlgodClient;
use algod_client::models::PendingTransactionResponse;
use std::sync::Arc;

pub async fn wait_for_confirmation(
    algod_client: Arc<AlgodClient>,
    tx_id: &str,
    max_rounds_to_wait: u32,
) -> Result<PendingTransactionResponse, TransactionError> {
    let status = algod_client.get_status().await.map_err(|e| {
        TransactionError::WaitForConfirmationError {
            message: format!("Failed to get Algod status: {:?}", e),
        }
    })?;

    let start_round = status.last_round + 1;
    let mut current_round = start_round;

    while current_round < start_round + max_rounds_to_wait as u64 {
        match algod_client.pending_transaction_information(tx_id).await {
            Ok(response) => {
                // Check for pool errors first - transaction was kicked out of pool
                if !response.pool_error.is_empty() {
                    return Err(TransactionError::WaitForConfirmationError {
                        message: format!(
                            "Transaction {} was rejected; pool error: {}",
                            tx_id,
                            response.pool_error.clone()
                        ),
                    });
                }

                // Check if transaction is confirmed
                if response.confirmed_round.is_some() {
                    return Ok(response);
                }
            }
            Err(error) => {
                // Only retry for 404 errors (transaction not found yet)
                // All other errors indicate permanent issues and should fail fast
                let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api {
                            source: algod_client::apis::AlgodApiError::PendingTransactionInformation {
                                error: algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            }
                        }
                    ) || error.to_string().contains("404");

                if is_retryable {
                    current_round += 1;
                    continue;
                } else {
                    return Err(TransactionError::WaitForConfirmationError {
                        message: error.to_string(),
                    });
                }
            }
        };

        let _ = algod_client.wait_for_block(current_round).await;
        current_round += 1;
    }

    Err(TransactionError::WaitForConfirmationError {
        message: format!(
            "Transaction {} unconfirmed after {} rounds",
            tx_id, max_rounds_to_wait
        ),
    })
}
