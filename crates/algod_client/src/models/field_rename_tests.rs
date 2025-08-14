#[cfg(test)]
mod field_rename_tests {
    use algod_client::models::{AppCallLogs, PendingTransactionResponse};
    use algokit_transact::{SignedTransaction, Transaction, TransactionHeader, PaymentTransactionFields};
    use serde_json;

    #[test]
    fn test_pending_transaction_response_field_renaming() {
        // Create a JSON string with the wire format field names
        let json_str = r#"{
            "application-index": 123,
            "asset-index": 456,
            "pool-error": "",
            "txn": {
                "sig": null,
                "msig": null,
                "lsig": null,
                "txn": {
                    "type": "pay",
                    "snd": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                    "rcv": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                    "amt": 1000,
                    "fee": 1000,
                    "fv": 1,
                    "lv": 1000
                }
            }
        }"#;

        // Test deserialization
        let response: PendingTransactionResponse = serde_json::from_str(json_str).unwrap();
        
        // Check that the fields are correctly mapped to the new field names
        assert_eq!(response.app_id, Some(123));
        assert_eq!(response.asset_id, Some(456));

        // Test serialization back to JSON
        let serialized = serde_json::to_string(&response).unwrap();
        
        // Verify that the JSON still uses the correct wire format field names
        assert!(serialized.contains("\"application-index\":123"));
        assert!(serialized.contains("\"asset-index\":456"));
    }

    #[test]
    fn test_app_call_logs_field_renaming() {
        let app_logs_json = r#"{
            "logs": ["YWJjZA=="],
            "application-index": 789,
            "txId": "ABCDEF123456"
        }"#;

        // Test deserialization
        let app_logs: AppCallLogs = serde_json::from_str(app_logs_json).unwrap();
        assert_eq!(app_logs.app_id, 789);
        assert_eq!(app_logs.tx_id, "ABCDEF123456");

        // Test serialization
        let app_logs_serialized = serde_json::to_string(&app_logs).unwrap();
        assert!(app_logs_serialized.contains("\"application-index\":789"));
    }

    #[test]
    fn test_field_name_consistency() {
        // Test that field names in structures are developer-friendly
        let mut response = PendingTransactionResponse {
            app_id: Some(123),
            asset_id: Some(456),
            pool_error: "".to_string(),
            txn: SignedTransaction {
                transaction: Transaction::Payment(PaymentTransactionFields {
                    header: TransactionHeader {
                        sender: Default::default(),
                        fee: None,
                        first_valid: 1,
                        last_valid: 1000,
                        genesis_hash: None,
                        genesis_id: None,
                        note: None,
                        rekey_to: None,
                        lease: None,
                        group: None,
                    },
                    receiver: Default::default(),
                    amount: 1000,
                    close_remainder_to: None,
                }),
                signature: None,
                auth_address: None,
                multisignature: None,
            },
            ..Default::default()
        };

        // Test that we can access fields with developer-friendly names
        assert_eq!(response.app_id, Some(123));
        assert_eq!(response.asset_id, Some(456));

        // Test that we can modify fields
        response.app_id = Some(999);
        response.asset_id = Some(888);
        
        assert_eq!(response.app_id, Some(999));
        assert_eq!(response.asset_id, Some(888));
    }
}