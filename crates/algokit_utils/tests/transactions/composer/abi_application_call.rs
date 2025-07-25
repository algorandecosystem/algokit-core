use crate::common::init_test_logging;
use algokit_abi::ABIValue;
use algokit_transact::{Address, AlgorandMsgpack, OnApplicationComplete};
use algokit_utils::{
    AppCreateParams,
    testing::*,
    transactions::{AppCallMethodCallParams, CommonParams, MethodCallParams},
};
use rstest::*;

use algokit_abi::{ABIMethod, ABIMethodArg, ABIMethodArgType, ABIType};
use std::path::Path;

/// Creates a ABI-compatible app using the hello_world TEAL programs.
pub async fn create_hello_app(context: &AlgorandTestContext, sender: Address) -> Option<u64> {
    // Read the actual TEAL programs from artifacts
    let approval_teal = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/artifacts/hello_world/approval.teal"),
    )
    .expect("Failed to read approval.teal");

    let clear_teal = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/artifacts/hello_world/clear.teal"),
    )
    .expect("Failed to read clear.teal");

    // Compile TEAL to bytecode using algod client
    let algod_client = &context.algod;

    let approval_result = algod_client
        .teal_compile(approval_teal.as_bytes().to_vec(), Some(false))
        .await
        .expect("Failed to compile approval program");
    let clear_result = algod_client
        .teal_compile(clear_teal.as_bytes().to_vec(), Some(false))
        .await
        .expect("Failed to compile clear state program");

    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: approval_result.result,
        clear_state_program: clear_result.result,
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_create(app_create_params)
        .expect("Failed to add app create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app create");

    // Extract app ID from the transaction confirmation
    if let Some(app_id) = result.confirmations[0].application_index {
        Some(app_id)
    } else {
        None
    }
}

/// Creates ABI method definition for hello(string)string.
pub fn create_hello_method() -> ABIMethod {
    ABIMethod {
        name: "hello".to_string(),
        description: Some("Hello world method with string argument".to_string()),
        args: vec![ABIMethodArg {
            name: Some("name".to_string()),
            description: Some("Name to greet".to_string()),
            arg_type: ABIMethodArgType::Value(ABIType::String),
        }],
        returns: Some(ABIType::String),
    }
}

/// Creates ABI method definition for add(uint64,uint64)uint64.
pub fn create_add_method() -> ABIMethod {
    ABIMethod {
        name: "add".to_string(),
        description: Some("Add two numbers".to_string()),
        args: vec![
            ABIMethodArg {
                name: Some("a".to_string()),
                description: Some("First number".to_string()),
                arg_type: ABIMethodArgType::Value(ABIType::Uint(
                    algokit_abi::abi_type::BitSize::new(64).unwrap(),
                )),
            },
            ABIMethodArg {
                name: Some("b".to_string()),
                description: Some("Second number".to_string()),
                arg_type: ABIMethodArgType::Value(ABIType::Uint(
                    algokit_abi::abi_type::BitSize::new(64).unwrap(),
                )),
            },
        ],
        returns: Some(ABIType::Uint(
            algokit_abi::abi_type::BitSize::new(64).unwrap(),
        )),
    }
}

/// Creates ABI method definition for simple()string.
pub fn create_simple_method() -> ABIMethod {
    ABIMethod {
        name: "simple".to_string(),
        description: Some("Simple method with no args".to_string()),
        args: vec![],
        returns: Some(ABIType::String),
    }
}

/// Creates ABI method definition for test()string.
pub fn create_test_method() -> ABIMethod {
    ABIMethod {
        name: "test".to_string(),
        description: Some("Test method with working selector".to_string()),
        args: vec![],
        returns: Some(ABIType::String),
    }
}

/// Test data for method selector validation scenarios.
/// Returns tuples of (method_name, expected_args_count, should_succeed).
#[fixture]
fn method_test_cases() -> Vec<(&'static str, usize, bool)> {
    vec![
        ("hello", 1, true),  // String argument method
        ("add", 2, true),    // Numeric arguments method
        ("simple", 0, true), // No arguments method (problematic selector)
        ("test", 0, true),   // No arguments method (working selector)
    ]
}

/// Test scenarios for ABI argument validation.
/// Returns tuples of (method_name, args, should_succeed, error_pattern).
#[fixture]
fn argument_validation_cases() -> Vec<(&'static str, Vec<ABIValue>, bool, &'static str)> {
    vec![
        (
            "hello",
            vec![ABIValue::String("world".to_string())],
            true,
            "",
        ),
        ("hello", vec![], false, "argument"), // Wrong arg count
        (
            "add",
            vec![ABIValue::Uint(10u64.into()), ABIValue::Uint(20u64.into())],
            true,
            "",
        ),
        ("add", vec![ABIValue::Uint(10u64.into())], false, "argument"), // Missing second arg
        ("simple", vec![], true, ""),
        (
            "simple",
            vec![ABIValue::String("unexpected".to_string())],
            false,
            "argument",
        ), // Extra arg
    ]
}

#[rstest]
#[tokio::test]
async fn test_abi_method_integration_real_deployment(
    #[values("hello")] method_name: &str, // Start with hello method which is implemented in the TEAL
) {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    // Create real ABI app using compiled TEAL programs
    let app_id = create_hello_app(context, sender_addr.clone())
        .await
        .expect("Failed to create real ABI app");

    let (method, args) = match method_name {
        "hello" => (
            create_hello_method(),
            vec![ABIValue::String("world".to_string())],
        ),
        _ => panic!("Unknown method: {}", method_name),
    };

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: method.clone(),
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Failed to add method call");

    // Test full execution pipeline
    let result = composer
        .send(None)
        .await
        .expect("Failed to send transaction");

    // Atomic operation verification
    assert_eq!(
        result.confirmations.len(),
        1,
        "Should have exactly one confirmation"
    );
    assert!(
        result.confirmations[0].confirmed_round.is_some(),
        "Transaction should be confirmed"
    );

    // Verify method selector encoding in transaction
    let confirmation = &result.confirmations[0];
    let transaction = &confirmation.txn.transaction;
    if let algokit_transact::Transaction::ApplicationCall(app_call) = transaction {
        if let Some(args) = &app_call.args {
            assert!(args.len() >= 1, "Should have at least method selector");
            assert_eq!(args[0].len(), 4, "Method selector should be 4 bytes");

            let expected_selector = method.selector().expect("Failed to get method selector");
            assert_eq!(
                args[0], expected_selector,
                "Method selector should match for {}",
                method_name
            );

            // For hello method, verify it matches expected selector from TEAL
            if method_name == "hello" {
                assert_eq!(
                    args[0],
                    vec![0x02, 0xbe, 0xce, 0x11],
                    "Hello method should have expected selector from TEAL"
                );
            }
        }
    }

    println!(
        "✓ Real ABI deployment test passed for method: {}",
        method_name
    );
}

#[rstest]
#[tokio::test]
async fn test_abi_method_transaction_building(
    #[values("hello", "add", "simple", "test")] method_name: &str,
) {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let (method, args) = match method_name {
        "hello" => (
            create_hello_method(),
            vec![ABIValue::String("world".to_string())],
        ),
        "add" => (
            create_add_method(),
            vec![ABIValue::Uint(10u64.into()), ABIValue::Uint(20u64.into())],
        ),
        "simple" => (create_simple_method(), vec![]),
        "test" => (create_test_method(), vec![]),
        _ => panic!("Unknown method: {}", method_name),
    };

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 1, // Use test app ID for building validation
        method: method.clone(),
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Failed to add method call");

    // Test transaction building (validates core ABI logic without network)
    let transactions = composer
        .build()
        .await
        .expect("Failed to build transactions");

    assert_eq!(
        transactions.len(),
        1,
        "Should build exactly one transaction"
    );

    // Verify method selector encoding in built transaction
    if let algokit_transact::Transaction::ApplicationCall(app_call) = &transactions[0].transaction {
        if let Some(args) = &app_call.args {
            assert!(args.len() >= 1, "Should have at least method selector");
            assert_eq!(args[0].len(), 4, "Method selector should be 4 bytes");

            let expected_selector = method.selector().expect("Failed to get method selector");
            assert_eq!(
                args[0], expected_selector,
                "Method selector should match for {}",
                method_name
            );
        }
    }
}

/// Test ABI argument validation with comprehensive error scenarios.
/// Validates atomic operation principles and error message consistency.
#[rstest]
#[tokio::test]
async fn test_abi_argument_validation(
    argument_validation_cases: Vec<(&'static str, Vec<ABIValue>, bool, &'static str)>,
    #[values(0, 1, 2, 3, 4, 5)] case_index: usize,
) {
    init_test_logging();

    if case_index >= argument_validation_cases.len() {
        return; // Skip if index is out of bounds
    }

    let (method_name, args, should_succeed, error_pattern) = &argument_validation_cases[case_index];

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let method = match *method_name {
        "hello" => create_hello_method(),
        "add" => create_add_method(),
        "simple" => create_simple_method(),
        _ => panic!("Unknown method: {}", method_name),
    };

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 1, // Use test app ID instead of creating real app
        method,
        args: args.clone(),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Adding method call should succeed");

    let build_result = composer.build().await;

    if *should_succeed {
        assert!(
            build_result.is_ok(),
            "Build should succeed for method '{}' with {} args",
            method_name,
            args.len()
        );
    } else {
        assert!(
            build_result.is_err(),
            "Build should fail for method '{}' with invalid args",
            method_name
        );

        if let Err(error) = build_result {
            let error_message = format!("{}", error);
            assert!(
                error_message.contains(error_pattern),
                "Error should contain '{}' for method '{}': {}",
                error_pattern,
                method_name,
                error_message
            );
        }
    }
}

/// Test msgpack serialization robustness for problematic method selectors.
/// Validates encoding/decoding cycle preservation for all selector patterns.
#[rstest]
#[tokio::test]
async fn test_msgpack_selector_serialization(
    #[values(
        ("simple()string", vec![194, 10, 242, 208]),
        ("test()string", vec![78, 101, 68, 118]),
        ("hello(string)string", vec![2, 190, 206, 17]),
        ("add(uint64,uint64)uint64", vec![144, 9, 94, 186])
    )]
    selector_case: (&str, Vec<u8>),
) {
    init_test_logging();

    let (method_signature, expected_selector) = selector_case;

    // Test serialization/deserialization cycle
    let test_address = Address([0; 32]);
    let app_call = algokit_transact::ApplicationCallTransactionFields {
        header: algokit_transact::TransactionHeader {
            sender: test_address,
            fee: Some(1000),
            first_valid: 1,
            last_valid: 100,
            genesis_hash: Some([0; 32]),
            genesis_id: Some("test".to_string()),
            note: None,
            lease: None,
            rekey_to: None,
            group: None,
        },
        app_id: 1234,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![expected_selector.clone()]),
        approval_program: None,
        clear_state_program: None,
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let transaction = algokit_transact::Transaction::ApplicationCall(app_call);
    let encoded = transaction
        .encode()
        .expect(&format!("Failed to encode {}", method_signature));
    let decoded = algokit_transact::Transaction::decode(&encoded)
        .expect(&format!("Failed to decode {}", method_signature));

    // Verify selector preservation through encode/decode cycle
    if let algokit_transact::Transaction::ApplicationCall(decoded_app_call) = decoded {
        if let Some(args) = decoded_app_call.args {
            assert_eq!(
                args[0], expected_selector,
                "Selector for {} should be preserved",
                method_signature
            );
        }
    }
}

/// Test transaction group functionality with multiple ABI method calls using real deployment.
/// Validates atomic group operations and proper group ID assignment with actual TEAL execution.
#[tokio::test]
async fn test_abi_transaction_group_real_deployment() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    // Create real ABI app using compiled TEAL programs
    let app_id = create_hello_app(context, sender_addr.clone())
        .await
        .expect("Failed to create real ABI app");

    let mut composer = context.composer.clone();

    // Add multiple hello method calls to create transaction group
    composer
        .add_method_call(MethodCallParams::AppCall(AppCallMethodCallParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            app_id,
            method: create_hello_method(),
            args: vec![ABIValue::String("first".to_string())],
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: OnApplicationComplete::NoOp,
            note: None,
        }))
        .expect("Failed to add first method call");

    composer
        .add_method_call(MethodCallParams::AppCall(AppCallMethodCallParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            app_id,
            method: create_hello_method(),
            args: vec![ABIValue::String("second".to_string())],
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: OnApplicationComplete::NoOp,
            note: None,
        }))
        .expect("Failed to add second method call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send transaction group");

    // Atomic operation verification - both transactions should succeed or all fail
    assert_eq!(
        result.confirmations.len(),
        2,
        "Should have exactly 2 confirmations"
    );

    for (i, confirmation) in result.confirmations.iter().enumerate() {
        assert!(
            confirmation.confirmed_round.is_some(),
            "Transaction {} should be confirmed",
            i
        );

        // Verify group ID consistency
        if i > 0 {
            assert_eq!(
                confirmation.txn.transaction.header().group,
                result.confirmations[0].txn.transaction.header().group,
                "All transactions should have the same group ID"
            );
        }
    }

    println!(
        "✓ Real deployment transaction group test passed with {} transactions",
        result.confirmations.len()
    );
}

/// Fast transaction group building test without network calls.
/// Validates atomic group operations and proper group ID assignment for rapid testing.
#[tokio::test]
async fn test_abi_transaction_group_building() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let mut composer = context.composer.clone();

    // Add multiple method calls to create transaction group
    composer
        .add_method_call(MethodCallParams::AppCall(AppCallMethodCallParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            app_id: 1, // Use test app ID for building validation
            method: create_hello_method(),
            args: vec![ABIValue::String("first".to_string())],
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: OnApplicationComplete::NoOp,
            note: None,
        }))
        .expect("Failed to add first method call");

    composer
        .add_method_call(MethodCallParams::AppCall(AppCallMethodCallParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            app_id: 1, // Use test app ID for building validation
            method: create_add_method(),
            args: vec![ABIValue::Uint(5u64.into()), ABIValue::Uint(10u64.into())],
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: OnApplicationComplete::NoOp,
            note: None,
        }))
        .expect("Failed to add second method call");

    let transactions = composer
        .build()
        .await
        .expect("Failed to build transaction group");

    // Atomic operation verification - both transactions should be built
    assert_eq!(transactions.len(), 2, "Should have exactly 2 transactions");

    // Verify group ID consistency
    let first_group_id = transactions[0].transaction.header().group;
    let second_group_id = transactions[1].transaction.header().group;

    assert_eq!(
        first_group_id, second_group_id,
        "All transactions should have the same group ID"
    );
    assert!(
        first_group_id.is_some(),
        "Group ID should be set for transaction groups"
    );
}

/// Test algod client integration with error handling and core logic validation.
/// Validates that core ABI logic works independently of network issues.
#[tokio::test]
async fn test_abi_core_logic_validation() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 1,                    // Use test app ID
        method: create_test_method(), // Use working method for core validation
        args: vec![],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Failed to add method call");

    // Test transaction building - this validates core ABI logic without network
    let built_transactions = composer
        .build()
        .await
        .expect("Failed to build transactions");
    assert!(
        !built_transactions.is_empty(),
        "Should build at least one transaction"
    );

    // Verify transaction structure
    if let algokit_transact::Transaction::ApplicationCall(app_call) =
        &built_transactions[0].transaction
    {
        assert_eq!(app_call.app_id, 1, "App ID should match");
        assert_eq!(
            app_call.on_complete,
            OnApplicationComplete::NoOp,
            "OnComplete should match"
        );

        if let Some(args) = &app_call.args {
            assert!(args.len() >= 1, "Should have method selector");
            assert_eq!(args[0].len(), 4, "Method selector should be 4 bytes");
        }
    }

    println!("✓ Core ABI logic validation passed - transaction building works correctly");
}

/// Test boundary conditions and edge cases for protocol constraints.
/// Validates proper handling of transaction group size limits.
#[rstest]
#[tokio::test]
async fn test_protocol_boundary_conditions(
    #[values(1, 2, 15, 16)] transaction_count: usize, // Test around group size limits
) {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let mut composer = context.composer.clone();

    // Add specified number of transactions
    for i in 0..transaction_count {
        composer
            .add_method_call(MethodCallParams::AppCall(AppCallMethodCallParams {
                common_params: CommonParams {
                    sender: sender_addr.clone(),
                    ..Default::default()
                },
                app_id: 1, // Use test app ID
                method: create_test_method(),
                args: vec![],
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: OnApplicationComplete::NoOp,
                note: Some(format!("Transaction {}", i).into_bytes()),
            }))
            .expect(&format!("Failed to add transaction {}", i));
    }

    // Test building - should work for valid transaction counts
    let build_result = composer.build().await;

    if transaction_count <= 16 {
        assert!(
            build_result.is_ok(),
            "Should build group with {} transactions",
            transaction_count
        );
        let transactions = build_result.unwrap();
        assert_eq!(
            transactions.len(),
            transaction_count,
            "Should have {} transactions",
            transaction_count
        );

        // Verify group ID consistency for groups > 1
        if transaction_count > 1 {
            let first_group_id = transactions[0].transaction.header().group;
            for (i, tx) in transactions.iter().enumerate() {
                assert_eq!(
                    tx.transaction.header().group,
                    first_group_id,
                    "Transaction {} should have same group ID",
                    i
                );
            }
            assert!(
                first_group_id.is_some(),
                "Group ID should be set for multi-transaction groups"
            );
        }
    } else {
        assert!(
            build_result.is_err(),
            "Should fail for group with {} transactions",
            transaction_count
        );
    }
}

/// Test ARC-4 tuple packing for methods with 15+ arguments
#[tokio::test]
async fn test_abi_tuple_packing_implementation() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    // Create method with exactly 15 arguments to test tuple packing threshold
    let method = create_method_with_15_args();
    let args = create_15_test_arguments();

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 1,
        method: method.clone(),
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Failed to add method call");

    let transactions = composer
        .build()
        .await
        .expect("Failed to build transactions with 15 args");

    assert_eq!(
        transactions.len(),
        1,
        "Should build exactly one transaction"
    );

    // Verify ARC-4 compliant argument structure
    if let algokit_transact::Transaction::ApplicationCall(app_call) = &transactions[0].transaction {
        if let Some(args) = &app_call.args {
            // Should have: selector + 14 individual args + 1 tuple (containing 1 arg)
            assert_eq!(args.len(), 16, "Should have 16 total ApplicationArgs");

            // Verify selector
            assert_eq!(args[0].len(), 4, "Method selector should be 4 bytes");

            // Verify individual arguments (slots 1-14)
            for i in 1..15 {
                assert!(
                    !args[i].is_empty(),
                    "Individual arg {} should not be empty",
                    i
                );
            }

            // Verify tuple argument in slot 15
            let tuple_arg = &args[15];
            assert!(!tuple_arg.is_empty(), "Tuple argument should not be empty");

            println!("✓ ARC-4 tuple packing test passed: 15 args -> 14 individual + 1 tuple");
        }
    }
}

/// Test with 16+ arguments to verify multiple args in tuple
#[tokio::test]
async fn test_abi_tuple_packing_multiple_in_tuple() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    // Create method with 17 arguments to test multiple args in tuple
    let method = create_method_with_17_args();
    let args = create_17_test_arguments();

    let method_call_params = MethodCallParams::AppCall(AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 1,
        method: method.clone(),
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
        note: None,
    });

    let mut composer = context.composer.clone();
    composer
        .add_method_call(method_call_params)
        .expect("Failed to add method call");

    let transactions = composer
        .build()
        .await
        .expect("Failed to build transactions with 17 args");

    assert_eq!(
        transactions.len(),
        1,
        "Should build exactly one transaction"
    );

    // Verify ARC-4 compliant argument structure
    if let algokit_transact::Transaction::ApplicationCall(app_call) = &transactions[0].transaction {
        if let Some(args) = &app_call.args {
            // Should have: selector + 14 individual args + 1 tuple (containing 3 args)
            assert_eq!(args.len(), 16, "Should have 16 total ApplicationArgs");

            // Verify tuple argument contains encoded data for 3 arguments
            let tuple_arg = &args[15];
            assert!(!tuple_arg.is_empty(), "Tuple argument should not be empty");
            assert!(
                tuple_arg.len() > 3,
                "Tuple should contain encoded data for multiple args"
            );

            println!(
                "✓ ARC-4 tuple packing test passed: 17 args -> 14 individual + 1 tuple(3 args)"
            );
        }
    }
}

/// Helper to create method with 15 arguments for testing
fn create_method_with_15_args() -> ABIMethod {
    let mut args = Vec::new();
    for i in 0..15 {
        args.push(ABIMethodArg {
            name: Some(format!("arg{}", i)),
            description: Some(format!("Test argument {}", i)),
            arg_type: ABIMethodArgType::Value(ABIType::Uint(
                algokit_abi::abi_type::BitSize::new(64).unwrap(),
            )),
        });
    }

    ABIMethod {
        name: "test15args".to_string(),
        description: Some("Test method with 15 arguments".to_string()),
        args,
        returns: Some(ABIType::String),
    }
}

/// Helper to create method with 17 arguments for testing
fn create_method_with_17_args() -> ABIMethod {
    let mut args = Vec::new();
    for i in 0..17 {
        args.push(ABIMethodArg {
            name: Some(format!("arg{}", i)),
            description: Some(format!("Test argument {}", i)),
            arg_type: ABIMethodArgType::Value(ABIType::Uint(
                algokit_abi::abi_type::BitSize::new(64).unwrap(),
            )),
        });
    }

    ABIMethod {
        name: "test17args".to_string(),
        description: Some("Test method with 17 arguments".to_string()),
        args,
        returns: Some(ABIType::String),
    }
}

/// Helper to create 15 test arguments
fn create_15_test_arguments() -> Vec<ABIValue> {
    (0..15).map(|i| ABIValue::Uint((i as u64).into())).collect()
}

/// Helper to create 17 test arguments
fn create_17_test_arguments() -> Vec<ABIValue> {
    (0..17).map(|i| ABIValue::Uint((i as u64).into())).collect()
}
