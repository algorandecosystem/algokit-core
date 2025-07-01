use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging for tests. Call this once at the start of any test that needs logging.
/// Safe to call multiple times - will only initialize once across the entire test suite.
///
/// This is a shared utility for all integration tests in this crate.
///
/// Configures logging to:
/// - Use test-friendly output (doesn't interfere with test harness)
/// - Default to DEBUG level for comprehensive debugging
/// - Respect RUST_LOG environment variable for CI/debugging
/// - Include targets in output for better debugging
pub fn init_test_logging() {
    INIT.call_once(|| {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .format_target(true) // Include target in output for better debugging
            .format_module_path(false) // Keep output cleaner
            .try_init();
    });
}
