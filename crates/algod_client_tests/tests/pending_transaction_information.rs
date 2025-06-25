use algod_client::apis::pending_transaction_information;
use algod_client::apis::{configuration::Configuration, raw_transaction, transaction_params};
use algod_client_tests::{
    LocalnetManager, NetworkType, TestAccountConfig, TestAccountManager, ALGOD_CONFIG,
};
use algokit_transact::{PaymentTransactionBuilder, Transaction, TransactionHeaderBuilder};
use std::convert::TryInto;
use std::sync::OnceLock;

/// Global configuration instance - idiomatic Rust pattern for shared test state
static CONFIG: OnceLock<Configuration> = OnceLock::new();

/// Get or initialize the algod client configuration
fn get_config() -> &'static Configuration {
    CONFIG.get_or_init(|| ALGOD_CONFIG.clone())
}

#[tokio::test]
async fn test_pending_transaction_broadcast() {
    // ARRANGE - Set up test environment and create a real transaction
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Create account manager and generate test accounts
    let mut account_manager = TestAccountManager::new(get_config().clone());

    let sender_config = TestAccountConfig {
        initial_funds: 10_000_000, // 10 ALGO
        suppress_log: true,
        network_type: NetworkType::LocalNet,
        funding_note: Some("Test sender account".to_string()),
    };

    let receiver_config = TestAccountConfig {
        initial_funds: 1_000_000, // 1 ALGO
        suppress_log: true,
        network_type: NetworkType::LocalNet,
        funding_note: Some("Test receiver account".to_string()),
    };

    let sender = account_manager
        .get_test_account(Some(sender_config))
        .await
        .expect("Failed to create sender account");

    let receiver = account_manager
        .get_test_account(Some(receiver_config))
        .await
        .expect("Failed to create receiver account");

    let sender_addr = sender.address().expect("Failed to get sender address");
    let receiver_addr = receiver.address().expect("Failed to get receiver address");

    // Get transaction parameters
    let params = transaction_params::transaction_params(get_config())
        .await
        .expect("Failed to get transaction params");

    // Convert genesis hash to 32-byte array
    let genesis_hash_bytes: [u8; 32] = params
        .genesis_hash
        .try_into()
        .expect("Genesis hash must be 32 bytes");

    // Build transaction header
    let header = TransactionHeaderBuilder::default()
        .sender(sender_addr.clone())
        .fee(params.min_fee as u64)
        .first_valid(params.last_round as u64)
        .last_valid((params.last_round + 1000) as u64)
        .genesis_id(params.genesis_id.clone())
        .genesis_hash(genesis_hash_bytes)
        .note(b"Test payment transaction".to_vec())
        .build()
        .expect("Failed to build transaction header");

    // Build payment transaction
    let payment_fields = PaymentTransactionBuilder::default()
        .header(header)
        .receiver(receiver_addr)
        .amount(500_000) // 0.5 ALGO
        .build_fields()
        .expect("Failed to build payment fields");

    let transaction = Transaction::Payment(payment_fields);
    let signed_bytes = sender
        .sign_transaction(&transaction)
        .expect("Failed to sign transaction");

    // ACT - Broadcast the transaction
    let response = raw_transaction::raw_transaction(get_config(), signed_bytes)
        .await
        .expect("Failed to broadcast transaction");

    // ASSERT - Verify response has transaction ID
    assert!(
        !response.tx_id.is_empty(),
        "Response should contain a transaction ID"
    );

    let pending_transaction = pending_transaction_information::pending_transaction_information(
        get_config(),
        &response.tx_id,
        Some("msgpack"),
    )
    .await
    .expect("Failed to get pending transaction information");

    assert_eq!(pending_transaction.pool_error, "");
    assert!(pending_transaction.confirmed_round.is_some());
}
