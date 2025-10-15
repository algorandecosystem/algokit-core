use crate::clients::{AccountManagerError, SigningAccount};
use crate::constants::UNENCRYPTED_DEFAULT_WALLET_NAME;
use crate::transactions::TransactionComposerParams;
use crate::{ClientManager, EmptySigner, PaymentParams, TransactionComposer, TransactionSigner};
use algod_client::models::Account;
use algokit_transact::{ALGORAND_SECRET_KEY_BYTE_LENGTH, Address};
use kmd_client::{
    apis::client::KmdClient,
    models::{
        CreateWalletRequest, ExportKeyRequest, GenerateKeyRequest, InitWalletHandleTokenRequest,
        ListKeysRequest,
    },
};
use std::sync::Arc;

/// Represents an account with its address, signer, and secret key
#[derive(Clone)]
pub struct KmdAccount {
    /// The address of the account
    pub address: Address,
    /// The signer that can sign transactions for this account
    pub signer: Arc<dyn TransactionSigner>,
    /// The secret key bytes for the account
    pub secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
}

/// Manages KMD wallets and accounts for LocalNet development
pub struct KmdAccountManager {
    client_manager: Arc<ClientManager>,
}

impl KmdAccountManager {
    pub fn new(client_manager: Arc<ClientManager>) -> Self {
        Self { client_manager }
    }

    async fn kmd(&self) -> Result<Arc<KmdClient>, AccountManagerError> {
        if let Ok(kmd_client) = self.client_manager.kmd() {
            return Ok(kmd_client);
        }

        let is_localnet = self.client_manager.is_localnet().await.map_err(|e| {
            AccountManagerError::EnvironmentError {
                message: format!("Failed to check if the environment is localnet: {}", e),
            }
        })?;

        let kmd_client: Option<KmdClient> = match is_localnet {
            false => None,
            true => {
                let config = ClientManager::get_config_from_environment_or_localnet();
                config
                    .kmd_config
                    .and_then(|cfg| ClientManager::get_kmd_client(&cfg).ok())
            }
        };

        let kmd_client = kmd_client.ok_or_else(|| AccountManagerError::KmdError {
            message: "Attempt to use Kmd client in AlgoKit instance with no Kmd configured"
                .to_string(),
        })?;

        Ok(Arc::new(kmd_client))
    }

    async fn get_wallet_handle(&self, wallet_id: &str) -> Result<String, AccountManagerError> {
        let kmd = self.kmd().await?;
        let response = kmd
            .init_wallet_handle_token(InitWalletHandleTokenRequest {
                wallet_id: Some(wallet_id.to_string()),
                wallet_password: None,
            })
            .await
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to init wallet handle: {}", e),
            })?;

        response
            .wallet_handle_token
            .ok_or_else(|| AccountManagerError::AlgodError {
                message: "No wallet handle token in response".to_string(),
            })
    }

    /// Gets an account from a KMD wallet
    ///
    /// # Arguments
    ///
    /// * `wallet_name` - The name of the wallet
    /// * `predicate` - Optional predicate to filter accounts based on account information from algod
    /// * `sender` - Optional sender address to use instead of the account's address
    ///
    /// # Returns
    ///
    /// Returns a `KmdAccount` that can be used to sign transactions
    pub async fn get_wallet_account(
        &self,
        wallet_name: &str,
        predicate: Option<Box<dyn Fn(&Account) -> bool>>,
        sender: Option<Address>,
    ) -> Result<KmdAccount, AccountManagerError> {
        let kmd = self.kmd().await?;

        // List wallets to find the wallet ID
        let wallets_response =
            kmd.list_wallets()
                .await
                .map_err(|e| AccountManagerError::KmdError {
                    message: format!("Failed to list wallets: {}", e),
                })?;

        let wallet = wallets_response
            .wallets
            .as_ref()
            .and_then(|wallets| {
                wallets
                    .iter()
                    .find(|w| w.name.as_ref() == Some(&wallet_name.to_string()))
            })
            .ok_or_else(|| AccountManagerError::KmdError {
                message: format!("Wallet '{}' not found", wallet_name),
            })?;

        let wallet_id = wallet
            .id
            .as_ref()
            .ok_or_else(|| AccountManagerError::KmdError {
                message: "Wallet has no ID".to_string(),
            })?;

        // Get wallet handle
        let wallet_handle = self.get_wallet_handle(wallet_id).await?;

        // List keys in wallet
        let keys_response = kmd
            .list_keys_in_wallet(ListKeysRequest {
                wallet_handle_token: Some(wallet_handle.clone()),
            })
            .await
            .map_err(|e| AccountManagerError::KmdError {
                message: format!("Failed to list keys: {}", e),
            })?;

        let addresses =
            keys_response
                .addresses
                .as_ref()
                .ok_or_else(|| AccountManagerError::KmdError {
                    message: "No addresses in wallet".to_string(),
                })?;

        if addresses.is_empty() {
            return Err(AccountManagerError::KmdError {
                message: "Wallet has no accounts".to_string(),
            });
        }

        // Apply predicate filter if provided
        let mut selected_index = 0;
        if let Some(pred) = predicate {
            let mut found = false;
            for (i, addr) in addresses.iter().enumerate() {
                // Get account information from algod
                let account_info = self
                    .client_manager
                    .algod()
                    .account_information(addr, None, None)
                    .await
                    .map_err(|e| AccountManagerError::KmdError {
                        message: format!("Failed to get account information: {}", e),
                    })?;

                if pred(&account_info) {
                    selected_index = i;
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(AccountManagerError::KmdError {
                    message: "No account matching predicate found".to_string(),
                });
            }
        }

        let address = &addresses[selected_index];

        // Export the private key
        let export_response = kmd
            .export_key(ExportKeyRequest {
                address: Some(address.clone()),
                wallet_handle_token: Some(wallet_handle.clone()),
                ..Default::default()
            })
            .await
            .map_err(|e| AccountManagerError::KmdError {
                message: format!("Failed to export key: {}", e),
            })?;

        let private_key =
            export_response
                .private_key
                .ok_or_else(|| AccountManagerError::KmdError {
                    message: "No private key in export response".to_string(),
                })?;

        // KMD returns a 64-byte key (32-byte seed + 32-byte public key)
        // We only need the first 32 bytes for the signing key
        let mut key_bytes = [0u8; ALGORAND_SECRET_KEY_BYTE_LENGTH];
        if private_key.len() < ALGORAND_SECRET_KEY_BYTE_LENGTH {
            return Err(AccountManagerError::KmdError {
                message: format!(
                    "Invalid private key length: expected at least {}, got {}",
                    ALGORAND_SECRET_KEY_BYTE_LENGTH,
                    private_key.len()
                ),
            });
        }
        // Take only the first 32 bytes (the seed)
        key_bytes.copy_from_slice(&private_key[..ALGORAND_SECRET_KEY_BYTE_LENGTH]);
        let signing_account = SigningAccount::new(key_bytes);

        let account_address = match sender {
            Some(addr) => addr,
            None => address.parse().map_err(|_| AccountManagerError::KmdError {
                message: "KMD returned invalid address format".into(),
            })?,
        };

        Ok(KmdAccount {
            address: account_address,
            signer: Arc::new(signing_account),
            secret_key: key_bytes,
        })
    }

    /// Gets or creates a LocalNet wallet account
    ///
    /// # Arguments
    ///
    /// * `wallet_name` - The name of the wallet to get or create
    /// * `fund_with` - Optional amount in microALGO to fund the new account with (defaults to 1000 ALGO = 1,000,000,000 microALGO)
    ///
    /// # Returns
    ///
    /// Returns a `KmdAccount` for the wallet account
    pub async fn get_or_create_local_net_wallet_account(
        &self,
        wallet_name: &str,
        fund_with: Option<u64>,
    ) -> Result<KmdAccount, AccountManagerError> {
        let is_localnet = self.client_manager.is_localnet().await.map_err(|e| {
            AccountManagerError::EnvironmentError {
                message: format!("Failed to check if the environment is localnet: {}", e),
            }
        })?;
        if !is_localnet {
            return Err(AccountManagerError::EnvironmentError {
                message: "This feature only works on LocalNet".into(),
            });
        }

        // Try to get existing wallet account first
        if let Ok(account) = self.get_wallet_account(wallet_name, None, None).await {
            return Ok(account);
        }

        let kmd = self.kmd().await?;

        // Wallet doesn't exist or has no accounts, so create it
        let wallet_id = {
            // Create new wallet
            let create_response = kmd
                .create_wallet(CreateWalletRequest {
                    wallet_name: Some(wallet_name.to_string()),
                    ..Default::default()
                })
                .await
                .map_err(|e| AccountManagerError::AlgodError {
                    message: format!("Failed to create wallet: {}", e),
                })?;

            create_response.wallet.and_then(|w| w.id).ok_or_else(|| {
                AccountManagerError::AlgodError {
                    message: "No wallet ID in create response".to_string(),
                }
            })?
        };

        // Get wallet handle
        let wallet_handle = self.get_wallet_handle(&wallet_id).await?;

        // Generate a new key
        let _generate_response = kmd
            .generate_key(GenerateKeyRequest {
                wallet_handle_token: Some(wallet_handle.clone()),
                ..Default::default()
            })
            .await
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to generate key: {}", e),
            })?;

        // Get the account
        let account = self
            .get_wallet_account(wallet_name, None, None)
            .await
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to get wallet account: {}", e),
            })?;

        // Get dispenser account
        let dispenser = self.get_local_net_dispenser_account().await?;

        // Create composer and send payment using the dispenser as the signer getter
        let mut composer = TransactionComposer::new(TransactionComposerParams {
            algod_client: self.client_manager.algod().clone(),
            signer_getter: Arc::new(EmptySigner {}),
            composer_config: None,
        });

        composer
            .add_payment(PaymentParams {
                sender: dispenser.address.clone(),
                receiver: account.address.clone(),
                amount: fund_with.unwrap_or(1_000_000_000), // provided amount or 1000 algos
                signer: Some(dispenser.signer.clone()),
                ..Default::default()
            })
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to add payment: {}", e),
            })?;

        composer
            .send(None)
            .await
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to send funding transaction: {}", e),
            })?;

        Ok(account)
    }

    pub async fn get_local_net_dispenser_account(&self) -> Result<KmdAccount, AccountManagerError> {
        let is_localnet = self.client_manager.is_localnet().await.map_err(|e| {
            AccountManagerError::EnvironmentError {
                message: format!("Failed to check if the environment is localnet: {}", e),
            }
        })?;
        if !is_localnet {
            return Err(AccountManagerError::EnvironmentError {
                message: "This feature only works on LocalNet".into(),
            });
        }

        return self
            .get_wallet_account(UNENCRYPTED_DEFAULT_WALLET_NAME, None, None)
            .await;
    }
}
