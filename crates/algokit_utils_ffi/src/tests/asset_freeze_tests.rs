use super::common::{TestResult, TestSuiteResult};
use super::traits::AssetFreezeTestAdapter;
use crate::transactions::common::UtilsError;
use std::sync::Arc;

/// Individual test function
fn test_frozen_asset_transfer(
    adapter: Arc<dyn AssetFreezeTestAdapter>,
    freeze_manager: String,
    alice: String,
    bob: String,
    asset_id: u64,
) -> TestResult {
    
    // Setup: Freeze Alice's asset
    if let Err(e) = adapter.setup_frozen_asset(freeze_manager, alice.clone(), asset_id) {
        return TestResult::failure("frozen_asset_transfer", &format!("Setup failed: {}", e));
    }
    
    // Verify asset is frozen
    let is_frozen = match adapter.is_frozen(alice.clone(), asset_id) {
        Ok(frozen) => frozen,
        Err(e) => return TestResult::failure("frozen_asset_transfer", &format!("Failed to check frozen status: {}", e)),
    };
    
    if !is_frozen {
        return TestResult::failure("frozen_asset_transfer", "Asset should be frozen but is not");
    }
    
    // Test: Try to transfer (should fail)
    match adapter.try_transfer_frozen(alice, bob, asset_id, 1000) {
        Ok(_) => TestResult::failure("frozen_asset_transfer", "Transfer should have failed for frozen asset"),
        Err(e) => {
            let error_message = format!("{}", e);
            if error_message.to_lowercase().contains("frozen") {
                TestResult::success("frozen_asset_transfer", "Correctly rejected frozen transfer")
            } else {
                TestResult::failure("frozen_asset_transfer", &format!("Unexpected error: {}", error_message))
            }
        },
    }
}

/// Comprehensive test suite runner - extensible pattern
#[uniffi::export]
pub fn run_asset_freeze_test_suite(
    adapter: Arc<dyn AssetFreezeTestAdapter>,
    freeze_manager: String,
    alice: String,
    bob: String,
    asset_id: u64,
) -> Result<TestSuiteResult, UtilsError> {
    let mut suite = TestSuiteResult::new("asset_freeze");
    
    // Add individual tests - easy to extend with more tests
    suite.add_result(test_frozen_asset_transfer(
        adapter.clone(),
        freeze_manager,
        alice,
        bob,
        asset_id,
    ));
    
    // Future: Add more tests here
    // suite.add_result(test_freeze_permissions(adapter.clone(), ...));
    // suite.add_result(test_unfreeze_workflow(adapter.clone(), ...));
    
    Ok(suite)
}

/// Convenience function for single test (backward compatibility)
#[uniffi::export]
pub fn test_frozen_asset_transfer_simple(
    adapter: Arc<dyn AssetFreezeTestAdapter>,
    freeze_manager: String,
    alice: String,
    bob: String,
    asset_id: u64,
) -> Result<TestResult, UtilsError> {
    Ok(test_frozen_asset_transfer(adapter, freeze_manager, alice, bob, asset_id))
}