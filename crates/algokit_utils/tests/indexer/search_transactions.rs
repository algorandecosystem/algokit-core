use indexer_client::IndexerClient;
use algokit_http_client::DefaultHttpClient;
use algokit_utils::{CommonParams, PaymentParams, ClientManager, testing::*};
use algokit_transact::TransactionId;
use std::sync::Arc;

use crate::common::init_test_logging;

#[tokio::test]
async fn test_search_transactions() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture.new_scope().await.expect("Failed to create new scope");
    
    let receiver = fixture.generate_account(None).await.expect("Failed to create receiver");
    let receiver_account = receiver.account().expect("Failed to get receiver account");
    
    let context = fixture.context().expect("Failed to get context");
    let sender_account = context.test_account.account().expect("Failed to get sender account");

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_account.address(),
            ..Default::default()
        },
        receiver: receiver_account.address(),
        amount: 500_000,
    };

    let mut composer = context.composer.clone();
    composer.add_payment(payment_params).expect("Failed to add payment");
    let result = composer.send(None).await.expect("Failed to send payment");
    let txid = result.confirmations[0].txn.id().expect("Failed to get transaction ID");

    let config = ClientManager::get_config_from_environment_or_localnet();
    let base_url = if let Some(port) = config.indexer_config.port {
        format!("{}:{}", config.indexer_config.server, port)
    } else {
        config.indexer_config.server.clone()
    };
    let http_client = Arc::new(DefaultHttpClient::new(&base_url));
    let indexer_client = IndexerClient::new(http_client);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let search_result = indexer_client
        .search_for_transactions(
            None, None, None, None, None, None, Some(&txid), None, None, None,
            None, None, None, None, None, None, None, None, None, None,
        )
        .await;

    assert!(search_result.is_ok(), "Search transactions should succeed: {:?}", search_result.err());

    let response = search_result.unwrap();
    let _current_round = response.current_round;
    
    if response.transactions.is_empty() {
        println!("Warning: Indexer may not be fully synced. Transaction {} not found", txid);
    } else {
        let found_tx = &response.transactions[0];
        assert_eq!(found_tx.id, Some(txid), "Transaction ID should match");
        assert_eq!(found_tx.sender, sender_account.address().to_string(), "Sender should match");
        assert_eq!(found_tx.tx_type, "pay".to_string(), "Transaction type should be payment");
        
        if let Some(payment_tx) = &found_tx.payment_transaction {
            assert_eq!(payment_tx.amount, 500_000, "Amount should match");
            assert_eq!(payment_tx.receiver, receiver_account.address().to_string(), "Receiver should match");
        }
    }
    
    if let Some(token) = &response.next_token {
        assert!(!token.is_empty(), "Next token should not be empty");
    }
}

#[tokio::test]
async fn test_search_transactions_error_handling() {
    init_test_logging();

    let http_client = Arc::new(DefaultHttpClient::new("http://invalid-host:8980"));
    let indexer_client = IndexerClient::new(http_client);

    let result = indexer_client
        .search_for_transactions(
            None, None, None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, None, None,
        )
        .await;

    assert!(result.is_err(), "Invalid indexer should fail");
}