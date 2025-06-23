// Models module - msgpack models have been moved to algod_client crate
// This module is now empty since all msgpack handling is done in the algod client

use crate::ModelRegistry;

/// Register all models in the registry
pub fn register_all_models(_registry: &mut ModelRegistry) {
    // All msgpack models have been moved to algod_client crate
}
