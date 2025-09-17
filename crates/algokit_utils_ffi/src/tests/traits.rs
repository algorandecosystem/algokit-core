use std::sync::Arc;

use crate::transactions::{common::UtilsError, composer::Composer};

/// Asset freeze specific test adapter
#[uniffi::export(with_foreign)]
pub trait AssetFreezeTestAdapter: Send + Sync {
    /// Get a descriptive name for this adapter (useful for logging)
    fn adapter_name(&self) -> String;

    /// Setup: Create a composer and freeze an asset
    fn setup_frozen_asset(
        &self,
        freeze_manager: String,
        target: String,
        asset_id: u64,
    ) -> Result<(), UtilsError>;

    /// Test: Try to transfer frozen asset (should fail)
    fn try_transfer_frozen(
        &self,
        from: String,
        to: String,
        asset_id: u64,
        amount: u64,
    ) -> Result<(), UtilsError>;

    /// Verify: Check if asset is frozen
    fn is_frozen(&self, address: String, asset_id: u64) -> Result<bool, UtilsError>;
}

#[uniffi::export(with_foreign)]
pub trait ComposerTestAdapter: Send + Sync {
    fn new(&self) -> Arc<Composer>;
}

// Can implement a native rust version, so that we can test rust as well using the same tests

// impl ComposerTestAdapter for Composer {
//     fn new(&self) -> Arc<Composer> {
//         Arc::new(Composer::new())
//     }
// }
