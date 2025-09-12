// Tests for Transaction Sending Features
// - Send Transactions: Submit and wait for transaction confirmation
// - Read-only Calls: Simulate calls without fees for query operations
// - Error Handling: Transform and expose logic errors with source maps
// - Result Processing: Process and decode return values
// - Group Transactions: Send grouped/atomic transactions
// - Custom Signers: Use custom transaction signers
// - Rekey Support: Handle rekey operations
// - Fund App Account: Send payment to app account

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_create_then_call_app
// - test_call_app_with_too_many_args
// - test_call_app_with_rekey
// - test_group_simulate_matches_send
// - test_sign_all_transactions_in_group_with_abi_call_with_transaction_arg
// - test_sign_transaction_in_group_with_different_signer_if_provided
// - Fund app account tests