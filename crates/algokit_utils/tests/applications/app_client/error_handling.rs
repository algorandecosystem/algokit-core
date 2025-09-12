// Tests for Error Handling Features
// - Logic Error Exposure: Expose logic errors with details
// - Source Map Support: Use source maps for debugging
// - ARC56 Error Messages: Handle ARC56-specific errors
// - Error Transformer: Transform errors for better debugging

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

// TODO: Implement tests based on Python/TypeScript references:
// - test_exposing_logic_error
// - test_app_client_with_sourcemaps
// - test_export_import_sourcemaps
// - test_arc56_error_messages_with_dynamic_template_vars_cblock_offset
// - test_arc56_undefined_error_message_with_dynamic_template_vars_cblock_offset
// - AppClient registers error transformer to AlgorandClient (TypeScript)