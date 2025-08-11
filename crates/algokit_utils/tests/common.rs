use indexer_client::IndexerClient;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging for tests. Call this once at the start of any test that needs logging.
/// Safe to call multiple times - will only initialize once across the entire test suite.
pub fn init_test_logging() {
    INIT.call_once(|| {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .format_target(true) // Include target in output for better debugging
            .format_module_path(false) // Keep output cleaner
            .try_init();
    });
}

#[allow(dead_code)] // Used by indexer tests, but compiler doesn't detect cross-module usage
pub async fn wait_for_indexer_transaction(
    indexer_client: &IndexerClient,
    tx_id: &str,
    max_attempts: u32,
) -> Result<(), String> {
    for attempt in 1..=max_attempts {
        match indexer_client
            .search_for_transactions(
                None,
                None,
                None,
                None,
                None,
                None,
                Some(tx_id),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
        {
            Ok(response) => {
                if !response.transactions.is_empty() {
                    return Ok(());
                }
            }
            Err(_) if attempt < max_attempts => {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            }
            Err(e) => return Err(format!("Indexer error: {:?}", e)),
        }

        if attempt < max_attempts {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    Err(format!(
        "Transaction {} not found in indexer after {} attempts",
        tx_id, max_attempts
    ))
}

#[allow(dead_code)] // Used by indexer tests, but compiler doesn't detect cross-module usage
pub async fn wait_for_indexer_application(
    indexer_client: &IndexerClient,
    app_id: u64,
    max_attempts: u32,
) -> Result<(), String> {
    for attempt in 1..=max_attempts {
        match indexer_client
            .search_for_applications(Some(app_id), None, None, None, None)
            .await
        {
            Ok(response) => {
                if !response.applications.is_empty() {
                    return Ok(());
                }
            }
            Err(_) if attempt < max_attempts => {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            }
            Err(e) => return Err(format!("Indexer error: {:?}", e)),
        }

        if attempt < max_attempts {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    Err(format!(
        "Application {} not found in indexer after {} attempts",
        app_id, max_attempts
    ))
}
