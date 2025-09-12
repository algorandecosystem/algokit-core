// Tests for Parameter Creation Features
// - Method Call Params: Create ABI method call parameters
// - Bare Call Params: Create raw/bare application call parameters
// - Default Value Resolution: Resolve default values from literals, methods, state, or boxes
// - Struct Handling: Convert ARC-56 structs to ABI tuples
// - Fund App Account Params: Create payment parameters for funding

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_create_app_with_constructor_deploy_time_params
// - test_construct_transaction_with_abi_encoding_including_transaction
// - test_construct_transaction_with_abi_encoding_including_foreign_references_not_in_signature