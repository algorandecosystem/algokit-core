// Tests for Transaction Creation Features
// - Create Transactions: Build unsigned transactions for all call types
// - Method Calls: Create ABI method-based transactions
// - Bare Calls: Create raw application transactions
// - Batch/Atomic: Support for atomic transaction composition
// - Create App: Create application transactions
// - Update App: Update application transactions
// - Delete App: Delete application transactions
// - Transaction with Boxes: Handle box references
// - Transaction with ABI Args: Handle ABI arguments
// - Foreign References: Handle foreign app/asset references

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_create_app
// - test_create_app_with_abi
// - test_update_app_with_abi
// - test_delete_app_with_abi
// - test_bare_create_abi_delete
// - test_construct_transaction_with_boxes
// - test_construct_transaction_with_abi_encoding_including_transaction
// - test_construct_transaction_with_abi_encoding_including_foreign_references_not_in_signature