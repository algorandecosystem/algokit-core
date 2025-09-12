// Tests for Client Management Features
// - Clone Client: Create new instances with modified parameters
// - Network Resolution: Resolve app by network from ARC-56 spec
// - Creator Resolution: Find app by creator address and name
// - App Lookup: Cache and retrieve app metadata
// - App Spec Normalization: Normalize between ARC-32 and ARC-56

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_clone_overriding_default_sender_and_inheriting_app_name
// - test_clone_overriding_app_name
// - test_clone_inheriting_app_name_based_on_default_handling
// - test_resolve_from_network
// - test_normalise_app_spec
// - fromCreatorAndName tests
// - fromNetwork tests