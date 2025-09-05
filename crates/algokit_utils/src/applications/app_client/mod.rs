use crate::AlgorandClient;
use crate::applications::AppDeployer;
use crate::clients::network_client::NetworkDetails;
use algokit_abi::Arc56Contract;
use algokit_transact::Address;
use std::collections::HashMap;
use std::str::FromStr;
mod abi_integration;
mod compilation;
mod error;
mod error_transformation;
mod params_builder;
mod sender;
mod state_accessor;
mod transaction_builder;
mod types;
mod utils;
pub use error::AppClientError;
use params_builder::ParamsBuilder;
pub use sender::TransactionSender;
pub use state_accessor::StateAccessor;
pub use transaction_builder::TransactionBuilder;
pub use types::{
    AppClientBareCallParams, AppClientJsonParams, AppClientMethodCallParams, AppClientParams,
    AppSourceMaps, FundAppAccountParams,
};

/// A client for interacting with an Algorand smart contract application (ARC-56 focused).
pub struct AppClient {
    app_id: Option<u64>,
    app_spec: Arc56Contract,
    algorand: AlgorandClient,
    default_sender: Option<String>,
    source_maps: Option<AppSourceMaps>,
    app_name: Option<String>,
}

impl AppClient {
    /// Create a new client from parameters.
    pub fn new(params: AppClientParams) -> Self {
        Self {
            app_id: params.app_id,
            app_spec: params.app_spec,
            algorand: params.algorand,
            default_sender: params.default_sender,
            source_maps: params.source_maps,
            app_name: params.app_name,
        }
    }

    /// Create a new client from JSON parameters.
    /// Accepts a JSON string and normalizes into a typed ARC-56 contract.
    pub fn from_json(params: types::AppClientJsonParams) -> Result<Self, AppClientError> {
        let app_spec = Arc56Contract::from_json(params.app_spec_json)
            .map_err(|e| AppClientError::ValidationError(e.to_string()))?;
        Ok(Self::new(AppClientParams {
            app_id: params.app_id,
            app_spec,
            algorand: params.algorand,
            app_name: params.app_name,
            default_sender: params.default_sender,
            source_maps: params.source_maps,
        }))
    }

    /// Construct from the current network using app_spec.networks mapping.
    ///
    /// Matches on either the network alias ("localnet", "testnet", "mainnet")
    /// or the network's genesis hash present in the node's suggested params.
    pub async fn from_network(
        app_spec: Arc56Contract,
        algorand: AlgorandClient,
        app_name: Option<String>,
        default_sender: Option<String>,
        source_maps: Option<AppSourceMaps>,
    ) -> Result<Self, AppClientError> {
        let network = algorand
            .client()
            .network()
            .await
            .map_err(|e| AppClientError::Network(e.to_string()))?;

        let candidate_keys = Self::candidate_network_keys(&network);
        let (app_id, available_keys) = match &app_spec.networks {
            Some(nets) => (
                Self::find_app_id_in_networks(&candidate_keys, nets),
                nets.keys().cloned().collect(),
            ),
            None => (None, Vec::new()),
        };

        let app_id = app_id.ok_or_else(|| AppClientError::AppIdNotFound {
            network_names: candidate_keys.clone(),
            available: available_keys,
        })?;

        Ok(Self::new(AppClientParams {
            app_id: Some(app_id),
            app_spec,
            algorand,
            app_name,
            default_sender,
            source_maps,
        }))
    }

    /// Construct from creator address and application name via indexer lookup.
    pub async fn from_creator_and_name(
        creator_address: &str,
        app_name: &str,
        app_spec: Arc56Contract,
        algorand: AlgorandClient,
        default_sender: Option<String>,
        source_maps: Option<AppSourceMaps>,
        ignore_cache: Option<bool>,
    ) -> Result<Self, AppClientError> {
        let address = Address::from_str(creator_address)
            .map_err(|e| AppClientError::Lookup(format!("Invalid creator address: {}", e)))?;

        let indexer_client = algorand.client().indexer();
        let mut app_deployer = AppDeployer::new(
            algorand.app().clone(),
            algorand.send().clone(),
            Some(indexer_client),
        );

        let lookup = app_deployer
            .get_creator_apps_by_name(&address, ignore_cache)
            .await
            .map_err(|e| AppClientError::Lookup(e.to_string()))?;

        let app_metadata = lookup.apps.get(app_name).ok_or_else(|| {
            AppClientError::Lookup(format!(
                "App not found for creator {} and name {}",
                creator_address, app_name
            ))
        })?;

        Ok(Self::new(AppClientParams {
            app_id: Some(app_metadata.app_id),
            app_spec,
            algorand,
            app_name: Some(app_name.to_string()),
            default_sender,
            source_maps,
        }))
    }

    fn candidate_network_keys(network: &NetworkDetails) -> Vec<String> {
        let mut names = vec![network.genesis_hash.clone()];
        if network.is_localnet {
            names.push("localnet".to_string());
        }
        if network.is_mainnet {
            names.push("mainnet".to_string());
        }
        if network.is_testnet {
            names.push("testnet".to_string());
        }
        names
    }

    fn find_app_id_in_networks(
        candidate_keys: &[String],
        networks: &HashMap<String, algokit_abi::arc56_contract::Network>,
    ) -> Option<u64> {
        for key in candidate_keys {
            if let Some(net) = networks.get(key) {
                return Some(net.app_id);
            }
        }
        None
    }

    pub fn app_id(&self) -> Option<u64> {
        self.app_id
    }
    pub fn app_spec(&self) -> &Arc56Contract {
        &self.app_spec
    }
    pub fn algorand(&self) -> &AlgorandClient {
        &self.algorand
    }
    pub fn app_name(&self) -> Option<&String> {
        self.app_name.as_ref()
    }
    pub fn default_sender(&self) -> Option<&String> {
        self.default_sender.as_ref()
    }

    /// Get the application address if app_id is set.
    pub fn app_address(&self) -> Option<Address> {
        self.app_id.map(|id| Address::from_app_id(&id))
    }

    fn get_sender_address(&self, sender: &Option<String>) -> Result<Address, AppClientError> {
        let sender_str = sender
            .as_ref()
            .or(self.default_sender.as_ref())
            .ok_or_else(|| AppClientError::ValidationError {
                message: format!(
                    "No sender provided and no default sender configured for app {}",
                    self.app_name.as_deref().unwrap_or("<unknown>")
                ),
            })?;
        Address::from_str(sender_str).map_err(|e| AppClientError::ValidationError {
            message: format!("Invalid sender address: {}", e),
        })
    }

    fn get_app_address(&self) -> Result<Address, AppClientError> {
        let app_id = self.app_id.ok_or_else(|| AppClientError::ValidationError {
            message: "Missing app_id".to_string(),
        })?;
        Ok(Address::from_app_id(&app_id))
    }

    /// Direct method: fund the application's account
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
    ) -> Result<
        crate::transactions::SendTransactionResult,
        crate::transactions::TransactionSenderError,
    > {
        let payment = self.params().fund_app_account(&params).map_err(|e| {
            crate::transactions::TransactionSenderError::ValidationError { message: e }
        })?;

        self.algorand.send().payment(payment, None).await
    }

    /// Get raw global state as HashMap<Vec<u8>, AppState>
    pub async fn get_global_state(
        &self,
    ) -> Result<
        std::collections::HashMap<Vec<u8>, crate::clients::app_manager::AppState>,
        AppClientError,
    > {
        self.algorand
            .app()
            .get_global_state(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get raw local state for an address
    pub async fn get_local_state(
        &self,
        address: &str,
    ) -> Result<
        std::collections::HashMap<Vec<u8>, crate::clients::app_manager::AppState>,
        AppClientError,
    > {
        self.algorand
            .app()
            .get_local_state(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
                address,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get all box names for the application
    pub async fn get_box_names(
        &self,
    ) -> Result<Vec<crate::clients::app_manager::BoxName>, AppClientError> {
        self.algorand
            .app()
            .get_box_names(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get the value of a box by raw identifier
    pub async fn get_box_value(
        &self,
        name: &crate::clients::app_manager::BoxIdentifier,
    ) -> Result<Vec<u8>, AppClientError> {
        self.algorand
            .app()
            .get_box_value(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
                name,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get a box value decoded using an ABI type
    pub async fn get_box_value_from_abi_type(
        &self,
        name: &crate::clients::app_manager::BoxIdentifier,
        abi_type: &algokit_abi::ABIType,
    ) -> Result<algokit_abi::ABIValue, AppClientError> {
        self.algorand
            .app()
            .get_box_value_from_abi_type(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
                name,
                abi_type,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get values for multiple boxes
    pub async fn get_box_values(
        &self,
        names: &[crate::clients::app_manager::BoxIdentifier],
    ) -> Result<Vec<Vec<u8>>, AppClientError> {
        self.algorand
            .app()
            .get_box_values(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
                names,
            )
            .await
            .map_err(AppClientError::from)
    }

    /// Get multiple box values decoded using an ABI type
    pub async fn get_box_values_from_abi_type(
        &self,
        names: &[crate::clients::app_manager::BoxIdentifier],
        abi_type: &algokit_abi::ABIType,
    ) -> Result<Vec<algokit_abi::ABIValue>, AppClientError> {
        self.algorand
            .app()
            .get_box_values_from_abi_type(
                self.app_id
                    .ok_or_else(|| AppClientError::ValidationError("Missing app_id".to_string()))?,
                names,
                abi_type,
            )
            .await
            .map_err(AppClientError::from)
    }
}

// -------- Minimal fluent API scaffolding (to be expanded incrementally) --------

impl AppClient {
    pub fn params(&self) -> ParamsBuilder<'_> {
        ParamsBuilder { client: self }
    }
    pub fn create_transaction(&self) -> TransactionBuilder<'_> {
        TransactionBuilder { client: self }
    }
    pub fn send(&self) -> TransactionSender<'_> {
        TransactionSender { client: self }
    }
    pub fn state(&self) -> StateAccessor<'_> {
        StateAccessor::new(self)
    }
}

// Method call parameter building is implemented in params_builder.rs

impl TransactionBuilder<'_> {
    pub async fn call_method(
        &self,
        params: types::AppClientMethodCallParams,
    ) -> Result<crate::transactions::BuiltTransactions, crate::transactions::composer::ComposerError>
    {
        let method_params = self
            .client
            .params()
            .get_method_call_params(&params)
            .await
            .map_err(
                |e| crate::transactions::composer::ComposerError::TransactionError { message: e },
            )?;
        self.client
            .algorand
            .create()
            .app_call_method_call(method_params)
            .await
    }
}

impl TransactionSender<'_> {}
