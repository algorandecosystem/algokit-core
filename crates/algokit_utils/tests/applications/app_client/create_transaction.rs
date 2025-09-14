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
