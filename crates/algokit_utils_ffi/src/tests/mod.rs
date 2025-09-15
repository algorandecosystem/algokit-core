pub mod common;
pub mod traits;
pub mod asset_freeze_tests;

// Re-export for external use
pub use common::{TestResult, TestSuiteResult};
pub use traits::AssetFreezeTestAdapter;
pub use asset_freeze_tests::{run_asset_freeze_test_suite, test_frozen_asset_transfer_simple};