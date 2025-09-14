// Tests for Compilation & Source Maps Features
// - TEAL Compilation: Compile TEAL templates with parameters
// - Source Map Management: Import/export source maps for debugging
// - Deploy-time Controls: Handle updatable/deletable flags
// - Template Substitution: Replace deploy-time parameters

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;
