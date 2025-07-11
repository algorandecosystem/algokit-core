use std::sync::Arc;

use algokit_utils::{AccountCloseParams, testing::*};
use algokit_utils::{CommonParams, PaymentParams};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_basic_payment_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver");

    let receiver_account = receiver.account().expect("Failed to get receiver account");

    let context = fixture.context().expect("Failed to get context");
    let sender_account = context
        .test_account
        .account()
        .expect("Failed to get sender account");

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_account.address(),
            ..Default::default()
        },
        receiver: receiver_account.address(),
        amount: 500_000, // 0.5 ALGO
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    let result = composer.send(None).await.expect("Failed to send payment");
    let transaction = &result.confirmations[0].txn.transaction;

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.amount, 500_000,
                "Payment amount should be 500_000 microALGOs"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}

#[tokio::test]
async fn test_basic_account_close_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let close_remainder_to = fixture
        .generate_account(None)
        .await
        .expect("Failed to create close_remainder_to account");

    let close_remainder_to_addr = close_remainder_to
        .account()
        .expect("Failed to get close_remainder_to account")
        .address();

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let account_close_params = AccountCloseParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        close_remainder_to: close_remainder_to_addr.clone(),
    };

    let mut composer = context.composer.clone();
    composer
        .add_account_close(account_close_params)
        .expect("Failed to add account close");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send account close");
    let transaction = result.confirmations[0].txn.transaction.clone();

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.receiver, sender_addr,
                "receiver should be set to the sender address"
            );
            assert_eq!(payment_fields.amount, 0, "Account close amount should be 0");
            assert!(
                payment_fields.close_remainder_to.is_some(),
                "close_remainder_to should be set for account close"
            );
            assert_eq!(
                payment_fields.close_remainder_to.unwrap(),
                close_remainder_to_addr,
                "close_remainder_to should match the provided address"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}

#[tokio::test]
async fn test_payment_with_custom_signer() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    // Generate a new account that will be the sender (custom signer)
    let sender_account = fixture
        .generate_account(None)
        .await
        .expect("Failed to create sender account");

    let sender_addr = sender_account
        .address()
        .expect("Failed to get sender address");

    let context = fixture.context().expect("Failed to get context");
    let receiver_addr = context
        .test_account
        .address()
        .expect("Failed to get receiver address");

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            signer: Some(Arc::new(sender_account.clone())),
            ..Default::default()
        },
        receiver: receiver_addr.clone(),
        amount: 100_000,
        close_remainder_to: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    let result = composer
        .send()
        .await
        .expect("Failed to send payment with custom signer");

    let transaction = result.txn.transaction;

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.amount, 100_000,
                "Payment amount should be 100_000 microALGOs"
            );
            assert_eq!(
                payment_fields.receiver, receiver_addr,
                "Payment receiver should match test account address"
            );
            assert_eq!(
                payment_fields.header.sender, sender_addr,
                "Transaction sender should be the custom signer account"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}

#[tokio::test]
async fn test_batch_signing_with_custom_signer() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    // Generate a new account that will be the sender (custom signer)
    let sender_account = fixture
        .generate_account(None)
        .await
        .expect("Failed to create sender account");

    let sender_addr = sender_account
        .address()
        .expect("Failed to get sender address");

    let context = fixture.context().expect("Failed to get context");
    let receiver_addr = context
        .test_account
        .address()
        .expect("Failed to get receiver address");

    // Create a wrapper signer that tracks how many times sign_txns is called
    struct SignerCallTracker {
        inner_signer: Arc<dyn algokit_utils::TxnSigner>,
        call_count: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl algokit_utils::TxnSigner for SignerCallTracker {
        async fn sign_txns(
            &self,
            txns: &[algokit_transact::Transaction],
            indices: &[usize],
        ) -> Result<Vec<algokit_transact::SignedTransaction>, String> {
            // Increment call count
            self.call_count.fetch_add(1, Ordering::SeqCst);

            // Verify we got the expected number of transactions
            assert_eq!(indices.len(), 2, "Should be signing 2 transactions");
            assert_eq!(txns.len(), 2, "Should have 2 transactions in total");

            // Delegate to the real signer
            self.inner_signer.sign_txns(txns, indices).await
        }
    }

    let call_count = Arc::new(AtomicUsize::new(0));
    let tracking_signer = Arc::new(SignerCallTracker {
        inner_signer: Arc::new(sender_account.clone()),
        call_count: call_count.clone(),
    });

    let mut composer = context.composer.clone();
    let foo = Arc::new(sender_account.clone());

    // Add two payment transactions with the same custom signer
    for i in 0..2 {
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                signer: Some(foo.clone()), // Use tracking signer
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: 50_000 + (i * 10_000), // Different amounts to distinguish transactions
            close_remainder_to: None,
        };
        composer
            .add_payment(payment_params)
            .expect("Failed to add payment");
    }

    let result = composer
        .send()
        .await
        .expect("Failed to send payments with custom signer");

    // Verify that sign_txns was called exactly once
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "sign_txns should be called exactly once for both transactions with the same signer"
    );

    // Verify the transaction was processed successfully
    let transaction = result.txn.transaction;
    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            // This will be the first transaction in the group
            assert_eq!(
                payment_fields.header.sender, sender_addr,
                "Transaction sender should be the custom signer account"
            );
            assert_eq!(
                payment_fields.receiver, receiver_addr,
                "Payment receiver should match test account address"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}
