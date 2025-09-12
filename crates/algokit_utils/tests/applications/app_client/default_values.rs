// Tests for Default Value Resolution
// - Literal Values: Base64-encoded constant values
// - Method Calls: Call other methods to get default values
// - Global/Local State: Read from state storage
// - Box Storage: Read from box storage

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_abi_with_default_arg_method (from const)
// - test_abi_with_default_arg_method (from abi method)
// - test_abi_with_default_arg_method (from global state)
// - test_abi_with_default_arg_method (from local state)
// - test_abi_with_default_arg_method (from box storage)