use crate::{
    TransactionSigner,
    clients::{KmdAccountManager, genesis_id_is_localnet},
    common::mnemonic,
    transactions::common::TransactionSignerGetter,
};
use algod_client::{AlgodClient, models::Account};
use algokit_transact::{
    ALGORAND_SECRET_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, Address, AlgorandMsgpack,
    KeyPairAccount, SignedTransaction, Transaction,
};
use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use snafu::Snafu;
use std::{collections::HashMap, sync::Arc};

/// A signing account that can sign transactions using a secret key
#[derive(Debug, Clone)]
pub struct SigningAccount {
    /// The ed25519 secret key used for signing transactions
    secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
}

impl SigningAccount {
    // TODO: maybe convert this to mnemonic
    /// Create a new SigningAccount from a secret key
    pub fn new(secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH]) -> Self {
        Self { secret_key }
    }

    /// Get the account's address
    pub fn address(&self) -> Address {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        account.address()
    }
}

#[async_trait]
impl TransactionSigner for SigningAccount {
    async fn sign_transactions(
        &self,
        txns: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String> {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let signer_account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        let signer_address = signer_account.address();

        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    let tx = txns[idx].clone();
                    let encoded_tx = tx
                        .encode()
                        .map_err(|e| format!("Failed to encode transaction: {:?}", e))?;
                    let sig: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH] =
                        signing_key.sign(&encoded_tx).to_bytes();

                    let auth_address = if tx.header().sender != signer_address {
                        Some(signer_address.clone())
                    } else {
                        None
                    };

                    Ok(SignedTransaction {
                        transaction: tx,
                        signature: Some(sig),
                        auth_address,
                        multisignature: None,
                    })
                } else {
                    Err(format!("Index {} out of bounds for transactions", idx))
                }
            })
            .collect()
    }
}

pub struct AccountManager {
    default_signer: Option<Arc<dyn TransactionSigner>>,
    accounts: HashMap<Address, Arc<dyn TransactionSigner>>,
    algod: Arc<AlgodClient>,
    kmd_account_manager: Option<Arc<KmdAccountManager>>,
}

impl AccountManager {
    pub fn new(
        algod: Arc<AlgodClient>,
        kmd_account_manager: Option<Arc<KmdAccountManager>>,
    ) -> Self {
        Self {
            default_signer: None,
            accounts: HashMap::new(),
            algod,
            kmd_account_manager,
        }
    }

    pub fn set_default_signer(&mut self, default_signer: Arc<dyn TransactionSigner>) {
        self.default_signer = Some(default_signer);
    }

    pub fn set_signer(&mut self, sender: Address, signer: Arc<dyn TransactionSigner>) {
        self.accounts.insert(sender, signer);
    }

    /// Takes all registered signers from the given `AccountManager` and adds them to this `AccountManager`.
    ///
    /// This is useful for situations where you have multiple contexts you are building accounts in such as unit tests.
    ///
    /// # Parameters
    /// * `another_account_manager` - Another account manager with signers registered
    /// * `overwrite_existing` - Whether or not to overwrite any signers that have the same sender address with the ones in the other account manager or not (default: true)
    pub fn set_signers(
        &mut self,
        another_account_manager: &AccountManager,
        overwrite_existing: bool,
    ) {
        if overwrite_existing {
            // Keep existing accounts, then add/overwrite with new ones
            for (address, signer) in &another_account_manager.accounts {
                self.accounts.insert(address.clone(), Arc::clone(signer));
            }
        } else {
            // Only add accounts that don't already exist
            for (address, signer) in &another_account_manager.accounts {
                self.accounts
                    .entry(address.clone())
                    .or_insert_with(|| Arc::clone(signer));
            }
        }
    }

    pub fn get_signer(
        &self,
        sender: Address,
    ) -> Result<Arc<dyn TransactionSigner>, AccountManagerError> {
        self.accounts
            .get(&sender)
            .cloned()
            .or(self.default_signer.clone())
            .ok_or_else(|| AccountManagerError::SignerNotFound {
                address: sender.to_string(),
            })
    }

    /// Returns the given sender account's current status.
    /// # Parameters
    /// * `sender` - The account / address to look up
    ///
    /// # Returns
    /// The account information
    pub async fn get_information(&self, sender: &Address) -> Result<Account, AccountManagerError> {
        self.algod
            .account_information(&sender.to_string(), None, None)
            .await
            .map_err(|e| AccountManagerError::AlgodError {
                message: e.to_string(),
            })
    }

    /// Tracks and returns an Algorand account with secret key loaded (i.e. that can sign transactions) by taking the mnemonic secret.
    ///
    /// # Parameters
    /// * `mnemonic_secret` - The mnemonic secret representing the private key of an account; **Note: Be careful how the mnemonic is handled**,
    ///   never commit it into source control and ideally load it from the environment (ideally via a secret storage service) rather than the file system.
    /// * `sender` - The optional sender address to use this signer for (aka a rekeyed account)
    ///
    /// # Returns
    /// The account's address
    pub fn from_mnemonic(
        &mut self,
        mnemonic_secret: &str,
        sender: Option<Address>,
    ) -> Result<Address, AccountManagerError> {
        // Convert mnemonic to secret key
        let secret_key = mnemonic_to_secret_key(mnemonic_secret)?;

        // Create signing account
        let signing_account = SigningAccount::new(secret_key);
        let address = signing_account.address();

        // Track the account
        let signer = Arc::new(signing_account);
        let sender_address = sender.unwrap_or_else(|| address.clone());
        self.set_signer(sender_address, signer);

        Ok(address)
    }

    /// Tracks and returns an Algorand account that is a rekeyed version of the given account to a new sender.
    ///
    /// # Parameters
    /// * `sender` - The sender address to use as the new sender
    /// * `signer` - The signer to use for this rekeyed account
    ///
    /// # Returns
    /// The sender address
    pub fn rekeyed(&mut self, sender: Address, signer: Arc<dyn TransactionSigner>) {
        self.set_signer(sender.clone(), signer);
    }

    /// Tracks and returns an Algorand account with private key loaded by convention from environment variables based on the given name identifier.
    ///
    /// ## Convention:
    /// * **Non-LocalNet:** will load `{NAME}_MNEMONIC` environment variable as a mnemonic secret; **Note: Be careful how the mnemonic is handled**,
    ///   never commit it into source control and ideally load it via a secret storage service rather than the file system.
    ///   If `{NAME}_SENDER` environment variable is defined then it will use that for the sender address (i.e. to support rekeyed accounts)
    /// * **LocalNet:** will load the account from a KMD wallet called `{NAME}` and if that wallet doesn't exist it will create it and fund the account for you
    ///
    /// This allows you to write code that will work seamlessly in production and local development (LocalNet) without manual config locally (including when you reset the LocalNet).
    ///
    /// # Parameters
    /// * `name` - The name identifier of the account
    /// * `_fund_with` - The optional amount in microAlgos to fund the account with when it gets created (when targeting LocalNet), if not specified then 1000 ALGO will be funded from the dispenser account
    ///
    /// # Returns
    /// The account's address
    pub async fn from_environment(
        &mut self,
        name: &str,
        fund_with: Option<u64>,
    ) -> Result<Address, AccountManagerError> {
        use std::env;
        use std::str::FromStr;

        let name_upper = name.to_uppercase();
        let mnemonic_key = format!("{}_MNEMONIC", name_upper);
        let sender_key = format!("{}_SENDER", name_upper);

        // Check for mnemonic in environment variables
        if let Ok(account_mnemonic) = env::var(&mnemonic_key) {
            let sender = env::var(&sender_key)
                .ok()
                .and_then(|s| Address::from_str(&s).ok());

            return self.from_mnemonic(&account_mnemonic, sender);
        }

        // Check if we're on LocalNet by checking genesis ID
        let genesis =
            self.algod
                .get_genesis()
                .await
                .map_err(|e| AccountManagerError::AlgodError {
                    message: format!("Failed to get genesis information: {}", e),
                })?;

        let is_localnet = genesis_id_is_localnet(&genesis.id);

        // Use KMD to get or create wallet account if on LocalNet and KMD is available
        if is_localnet {
            if let Some(kmd_manager) = &self.kmd_account_manager {
                let kmd_account = kmd_manager
                    .get_or_create_wallet_account(name, fund_with)
                    .await?;

                // Register the signer
                self.set_signer(kmd_account.address.clone(), kmd_account.signer);

                return Ok(kmd_account.address);
            }
        }

        Err(AccountManagerError::EnvironmentError {
            message: format!(
                "Missing environment variable {} when looking for account {}",
                mnemonic_key, name
            ),
        })
    }
}

#[derive(Debug, Snafu)]
pub enum AccountManagerError {
    #[snafu(display("No signer found for address: {address}"))]
    SignerNotFound { address: String },

    #[snafu(display("Algod error: {message}"))]
    AlgodError { message: String },

    #[snafu(display("Mnemonic error: {message}"))]
    MnemonicError { message: String },

    #[snafu(display("Environment error: {message}"))]
    EnvironmentError { message: String },

    #[snafu(display("KMD error: {message}"))]
    KmdError { message: String },
}

/// Convert a 25-word mnemonic into a 32-byte secret key
fn mnemonic_to_secret_key(
    mnemonic: &str,
) -> Result<[u8; ALGORAND_SECRET_KEY_BYTE_LENGTH], AccountManagerError> {
    mnemonic::to_key(mnemonic).map_err(|e| AccountManagerError::MnemonicError {
        message: e.to_string(),
    })
}

impl TransactionSignerGetter for AccountManager {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        self.get_signer(address).map_err(|e| e.to_string())
    }
}
