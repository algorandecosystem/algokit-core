use crate::clients::app_manager::AppManager;
use crate::clients::asset_manager::AssetManager;
use crate::clients::client_manager::ClientManager;
use crate::clients::network_client::{AlgoConfig, AlgorandService};
use crate::transactions::{
    Composer, ComposerParams, TransactionComposerConfig, TransactionCreator, TransactionSender,
};
use crate::{AccountManager, TransactionSigner};
use algod_client::models::TransactionParams;
use algokit_transact::Address;
use std::sync::{Arc, Mutex};

pub struct AlgorandClient {
    client_manager: Arc<ClientManager>,
    asset_manager: Arc<AssetManager>,
    app_manager: Arc<AppManager>,
    transaction_sender: Arc<TransactionSender>,
    transaction_creator: Arc<TransactionCreator>,
    account_manager: Arc<Mutex<AccountManager>>,
    default_composer_config: Option<TransactionComposerConfig>,
}

/// A client that brokers easy access to Algorand functionality.
pub struct AlgorandClientParams {
    pub client_config: AlgoConfig,
    pub composer_config: Option<TransactionComposerConfig>,
}

impl AlgorandClient {
    pub fn new(params: &AlgorandClientParams) -> Self {
        let client_manager = ClientManager::new(&params.client_config).unwrap();
        let algod_client = client_manager.algod();

        let account_manager = Arc::new(Mutex::new(AccountManager::new()));

        // TODO: an easy way to create algorand client without this newGroup
        // TODO: convert this to trait
        let new_group = {
            let algod_client = algod_client.clone();
            let account_manager = account_manager.clone();
            let default_composer_config = params.composer_config.clone();
            move |composer_config: Option<TransactionComposerConfig>| {
                Composer::new(ComposerParams {
                    algod_client: algod_client.clone(),
                    signer_getter: account_manager.clone(),
                    composer_config: composer_config.or_else(|| default_composer_config.clone()),
                })
            }
        };

        let asset_manager = Arc::new(AssetManager::new(algod_client.clone(), new_group.clone()));
        let app_manager = Arc::new(AppManager::new(algod_client.clone()));

        // Create closure for new_group function
        let transaction_sender = TransactionSender::new(
            new_group.clone(),
            asset_manager.clone(),
            app_manager.clone(),
        );

        // Create closure for TransactionCreator
        let transaction_creator = TransactionCreator::new(new_group.clone());

        Self {
            client_manager: client_manager.into(),
            account_manager: account_manager.clone(),
            asset_manager: asset_manager.clone(),
            app_manager: app_manager.clone(),
            transaction_sender: transaction_sender.into(),
            transaction_creator: transaction_creator.into(),
            default_composer_config: params.composer_config.clone(),
        }
    }

    pub async fn get_suggested_params(
        &self,
    ) -> Result<TransactionParams, Box<dyn std::error::Error>> {
        Ok(self.client_manager.algod().transaction_params().await?)
    }

    pub fn client(&self) -> Arc<ClientManager> {
        self.client_manager.clone()
    }

    /// Get access to the AssetManager for asset operations
    pub fn asset(&self) -> Arc<AssetManager> {
        self.asset_manager.clone()
    }

    /// Get access to the AppManager for app operations
    pub fn app(&self) -> Arc<AppManager> {
        self.app_manager.clone()
    }

    /// Get access to the TransactionSender for sending transactions
    pub fn send(&self) -> Arc<TransactionSender> {
        self.transaction_sender.clone()
    }

    /// Get access to the TransactionCreator for building transactions
    pub fn create(&self) -> Arc<TransactionCreator> {
        self.transaction_creator.clone()
    }

    pub fn account(&self) -> Arc<Mutex<AccountManager>> {
        self.account_manager.clone()
    }

    /// Create a new transaction composer for building transaction groups
    pub fn new_group(&self, params: Option<TransactionComposerConfig>) -> Composer {
        Composer::new(ComposerParams {
            algod_client: self.client_manager.algod().clone(),
            signer_getter: self.account_manager.clone(),
            composer_config: params.or_else(|| self.default_composer_config.clone()),
        })
    }

    pub fn default_localnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_default_localnet_config(AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_default_localnet_config(
                    AlgorandService::Indexer,
                )),
                kmd_config: Some(ClientManager::get_default_localnet_config(
                    AlgorandService::Kmd,
                )),
            },
            composer_config: params,
        })
    }

    pub fn testnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_algonode_config("testnet", AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_algonode_config(
                    "testnet",
                    AlgorandService::Indexer,
                )),
                kmd_config: None,
            },
            composer_config: params,
        })
    }

    pub fn mainnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_algonode_config("mainnet", AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_algonode_config(
                    "mainnet",
                    AlgorandService::Indexer,
                )),
                kmd_config: None,
            },
            composer_config: params,
        })
    }

    pub fn from_environment(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: ClientManager::get_config_from_environment_or_localnet(),
            composer_config: params,
        })
    }

    pub fn set_signer(&mut self, sender: Address, signer: Arc<dyn TransactionSigner>) {
        self.account_manager
            .lock()
            .unwrap()
            .set_signer(sender, signer);
    }
}
