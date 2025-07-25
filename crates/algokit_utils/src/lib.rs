pub mod clients;
pub mod testing;
pub mod transactions;

// Re-exports for clean UniFFI surface
pub use clients::{
    AlgoClientConfig, AlgoConfig, AlgorandClient, AlgorandNetwork, AlgorandService, ClientManager,
    NetworkDetails, TokenHeader, genesis_id_is_localnet,
};
pub use testing::{
    AlgorandFixture, AlgorandTestContext, algorand_fixture, algorand_fixture_with_config,
};
pub use transactions::{
    AccountCloseParams,

    // Core ABI method call types
    AppCallMethodCallParams,
    // Application transaction types (Python-compatible naming)
    AppCallParams,
    AppCreateMethodCallParams,
    AppCreateParams,
    AppDeleteMethodCallParams,
    AppDeleteParams,
    AppUpdateMethodCallParams,
    AppUpdateParams,

    // Asset transaction types
    AssetClawbackParams,
    AssetCreateParams,
    AssetDestroyParams,
    AssetFreezeParams,
    AssetOptInParams,
    AssetOptOutParams,
    AssetReconfigParams,
    AssetTransferParams,
    AssetUnfreezeParams,

    // Common transaction parameters
    CommonParams,

    ComposerError,
    ComposerTransaction,
    EmptySigner,
    MethodCallParams,

    NonParticipationKeyRegistrationParams,

    OfflineKeyRegistrationParams,
    // Key registration
    OnlineKeyRegistrationParams,
    // Payment
    PaymentParams,

    SendParams,
    SendTransactionComposerResults,

    // Core transaction management
    TransactionComposer,
    // Signing infrastructure
    TransactionSigner,
    TransactionSignerGetter,
};
