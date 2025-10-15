use crate::{
    ClientManager, PaymentParams, SendParams, TransactionComposer, TransactionComposerParams,
    TransactionComposerSendResult, TransactionSigner,
    clients::{DispenserFundResponse, KmdAccountManager, TestNetDispenserApiClient},
    common::mnemonic,
    transactions::common::TransactionSignerGetter,
};
use algod_client::models::Account;
use algokit_transact::{
    ALGORAND_SECRET_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, Address, AlgorandMsgpack,
    KeyPairAccount, SignedTransaction, Transaction,
};
use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use snafu::Snafu;
use std::{collections::HashMap, sync::Arc};

/// A predicate function used to filter accounts based on their information from algod
pub type AccountPredicate = Option<Box<dyn Fn(&Account) -> bool>>;

/// A signing account that can sign transactions using a secret key
#[derive(Debug, Clone)]
pub struct SigningAccount {
    /// The ed25519 secret key used for signing transactions
    pub secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
}

impl SigningAccount {
    /// Create a new SigningAccount from a secret key
    pub fn new(secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH]) -> Self {
        Self { secret_key }
    }

    /// Generate a new random SigningAccount
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self {
            secret_key: signing_key.to_bytes(),
        }
    }

    /// Create a SigningAccount from a mnemonic string
    pub fn from_mnemonic(mnemonic_str: &str) -> Result<Self, AccountManagerError> {
        let secret_key = mnemonic_to_secret_key(mnemonic_str)?;
        Ok(Self { secret_key })
    }

    /// Create a SigningAccount from secret key bytes (supports 32 or 64 byte keys)
    pub fn from_secret_key(secret_key: &[u8]) -> Result<Self, AccountManagerError> {
        let key_slice = match secret_key.len() {
            ALGORAND_SECRET_KEY_BYTE_LENGTH => secret_key,
            len if len == ALGORAND_SECRET_KEY_BYTE_LENGTH * 2 => {
                &secret_key[..ALGORAND_SECRET_KEY_BYTE_LENGTH]
            }
            other => {
                return Err(AccountManagerError::MnemonicError {
                    message: format!(
                        "Secret key must be {} or {} bytes, got {}",
                        ALGORAND_SECRET_KEY_BYTE_LENGTH,
                        ALGORAND_SECRET_KEY_BYTE_LENGTH * 2,
                        other
                    ),
                });
            }
        };

        let mut key_bytes = [0u8; ALGORAND_SECRET_KEY_BYTE_LENGTH];
        key_bytes.copy_from_slice(key_slice);

        Ok(Self {
            secret_key: key_bytes,
        })
    }

    /// Get the account's address
    pub fn address(&self) -> Address {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        account.address()
    }

    /// Get the account as a KeyPairAccount (for compatibility with algokit_transact)
    pub fn account(&self) -> KeyPairAccount {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let verifying_key: VerifyingKey = (&signing_key).into();
        KeyPairAccount::from_pubkey(&verifying_key.to_bytes())
    }

    /// Get the account's mnemonic (25-word Algorand mnemonic)
    pub fn mnemonic(&self) -> Result<String, AccountManagerError> {
        mnemonic::from_key(&self.secret_key).map_err(|e| AccountManagerError::MnemonicError {
            message: e.to_string(),
        })
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
    client_manager: Arc<ClientManager>,
    kmd_account_manager: Arc<KmdAccountManager>,
}

impl AccountManager {
    pub fn new(client_manager: Arc<ClientManager>) -> Self {
        Self {
            default_signer: None,
            accounts: HashMap::new(),
            client_manager: client_manager.clone(),
            kmd_account_manager: Arc::new(KmdAccountManager::new(client_manager.clone())),
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
        self.client_manager
            .algod()
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
    /// The signing account
    pub fn from_mnemonic(
        &mut self,
        mnemonic_secret: &str,
        sender: Option<Address>,
    ) -> Result<SigningAccount, AccountManagerError> {
        // Convert mnemonic to secret key
        let secret_key = mnemonic_to_secret_key(mnemonic_secret)?;

        // Create signing account
        let signing_account = SigningAccount::new(secret_key);
        let address = signing_account.address();

        // Track the account
        let signer = Arc::new(signing_account.clone());
        let sender_address = sender.unwrap_or_else(|| address.clone());
        self.set_signer(sender_address, signer);

        Ok(signing_account)
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
    /// The signing account
    pub async fn from_environment(
        &mut self,
        name: &str,
        fund_with: Option<u64>,
    ) -> Result<SigningAccount, AccountManagerError> {
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

        let is_localnet = self.client_manager.is_localnet().await.map_err(|e| {
            AccountManagerError::EnvironmentError {
                message: format!("Failed to check if the environment is localnet: {}", e),
            }
        })?;

        // Use KMD to get or create wallet account if on LocalNet and KMD is available
        if is_localnet {
            let kmd_account = self
                .kmd_account_manager
                .get_or_create_local_net_wallet_account(name, fund_with)
                .await?;

            // Create signing account from secret key
            let signing_account = SigningAccount::new(kmd_account.secret_key);

            // Register the signer
            self.set_signer(kmd_account.address.clone(), kmd_account.signer);

            return Ok(signing_account);
        }

        Err(AccountManagerError::EnvironmentError {
            message: format!(
                "Missing environment variable {} when looking for account {}",
                mnemonic_key, name
            ),
        })
    }

    /// Tracks and returns an Algorand account with private key loaded from the given KMD wallet (identified by name).
    ///
    /// # Parameters
    /// * `name` - The name of the wallet to retrieve an account from
    /// * `predicate` - An optional filter to use to find the account (otherwise it will return a random account from the wallet)
    /// * `sender` - The optional sender address to use this signer for (aka a rekeyed account)
    ///
    /// # Returns
    /// The signing account
    pub async fn from_kmd(
        &mut self,
        name: &str,
        predicate: AccountPredicate,
        sender: Option<Address>,
    ) -> Result<SigningAccount, AccountManagerError> {
        let kmd_account = self
            .kmd_account_manager
            .get_wallet_account(name, predicate, sender)
            .await?;

        // Create signing account from secret key
        let signing_account = SigningAccount::new(kmd_account.secret_key);

        // Register the signer
        self.set_signer(kmd_account.address.clone(), kmd_account.signer);

        Ok(signing_account)
    }

    // TODO: PD - implement multisig and logicsig

    /// Tracks and returns a new, random Algorand account with secret key loaded.
    ///
    /// # Returns
    /// The signing account
    pub fn random(&mut self) -> SigningAccount {
        // Generate a random signing account
        let signing_account = SigningAccount::generate();
        let address = signing_account.address();

        // Track the account
        let signer = Arc::new(signing_account.clone());
        self.set_signer(address.clone(), signer);

        signing_account
    }

    /// Returns an account (with private key loaded) that can act as a dispenser from
    /// environment variables, or against default LocalNet if no environment variables present.
    ///
    /// If present, it will load the account mnemonic stored in `DISPENSER_MNEMONIC` environment variable
    /// and optionally `DISPENSER_SENDER` if it's a rekeyed account.
    ///
    /// If not present, it will get the default funded account from LocalNet's `unencrypted-default-wallet`.
    ///
    /// # Returns
    /// The dispenser signing account
    pub async fn dispenser_from_environment(
        &mut self,
    ) -> Result<SigningAccount, AccountManagerError> {
        use std::env;

        const DISPENSER_ACCOUNT: &str = "DISPENSER";

        // Check if DISPENSER_MNEMONIC environment variable is set
        let mnemonic_key = format!("{}_MNEMONIC", DISPENSER_ACCOUNT);

        if env::var(&mnemonic_key).is_ok() {
            // Use from_environment which will handle both mnemonic and optional sender
            self.from_environment(DISPENSER_ACCOUNT, None).await
        } else {
            // Fall back to LocalNet dispenser
            self.local_net_dispenser().await
        }
    }

    /// Returns the default funded account from LocalNet's `unencrypted-default-wallet`.
    ///
    /// This is a convenience method that gets the dispenser account from the default KMD wallet
    /// that comes pre-funded on LocalNet.
    ///
    /// # Returns
    /// The LocalNet dispenser signing account
    pub async fn local_net_dispenser(&mut self) -> Result<SigningAccount, AccountManagerError> {
        let dispenser = self
            .kmd_account_manager
            .get_local_net_dispenser_account()
            .await?;

        // Create signing account from secret key
        let signing_account = SigningAccount::new(dispenser.secret_key);

        // Register the signer
        self.set_signer(dispenser.address.clone(), dispenser.signer);

        Ok(signing_account)
    }

    /// Rekey an account to a new address.
    ///
    /// **Note:** Please be careful with this function and be sure to read the
    /// [official rekey guidance](https://developer.algorand.org/docs/get-details/accounts/rekey/).
    ///
    /// # Returns
    /// The result of the rekey transaction
    pub async fn rekey_account(
        &mut self,
        account: Address,
        rekey_to: Address,
        rekey_to_signer: Option<Arc<dyn TransactionSigner>>,
        send_params: Option<SendParams>,
    ) -> Result<TransactionComposerSendResult, AccountManagerError> {
        // Create composer
        let mut composer = self.get_composer();

        // Add 0 ALGO payment to self with rekey
        composer
            .add_payment(PaymentParams {
                sender: account.clone(),
                receiver: account.clone(),
                amount: 0,
                rekey_to: Some(rekey_to.clone()),
                ..Default::default()
            })
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to add rekey payment: {}", e),
            })?;

        // Send the transaction
        let result =
            composer
                .send(send_params)
                .await
                .map_err(|e| AccountManagerError::AlgodError {
                    message: format!("Failed to send rekey transaction: {}", e),
                })?;

        // If a signer was provided, track it for the rekeyed account
        if let Some(signer) = rekey_to_signer {
            self.rekeyed(account, signer);
        }

        Ok(result) // TODO: review the return
    }

    fn get_composer(&self) -> TransactionComposer {
        TransactionComposer::new(TransactionComposerParams {
            algod_client: self.client_manager.algod().clone(),
            signer_getter: Arc::new(AccountManagerSignerGetter {
                accounts: self.accounts.clone(),
            }),
            composer_config: None,
        })
    }

    async fn get_ensure_funded_amount(
        &self,
        sender: &Address,
        funding_params: &EnsureFundedParams,
    ) -> Result<Option<u64>, AccountManagerError> {
        let account_info = self.get_information(sender).await?;
        let current_spending_balance = account_info.amount.saturating_sub(account_info.min_balance);

        let amount_funded = calculate_fund_amount(
            funding_params.min_spending_balance,
            current_spending_balance,
            funding_params.min_funding_increment.unwrap_or_default(),
        );

        Ok(amount_funded)
    }

    /// Funds a given account using a dispenser account as a funding source such that
    /// the given account has a certain amount of Algo free to spend (accounting for
    /// Algo locked in minimum balance requirement).
    ///
    /// https://developer.algorand.org/docs/get-details/accounts/#minimum-balance
    ///
    /// # Parameters
    /// * `account_to_fund` - The address of the account to fund
    /// * `dispenser_account` - The address of the account to use as a dispenser funding source
    /// * `funding_params` - The parameters specifying the funding requirements
    /// * `send_params` - Optional parameters to control the execution of the transaction
    ///
    /// # Returns
    /// - `Some(EnsureFundedResult)` - The result of executing the dispensing transaction and the `amount_funded` if funds were needed
    /// - `None` - If no funds were needed
    pub async fn ensure_funded(
        &mut self,
        account_to_fund: &Address,
        dispenser_account: &Address,
        funding_params: &EnsureFundedParams,
        send_params: Option<SendParams>,
    ) -> Result<Option<EnsureFundedResult>, AccountManagerError> {
        let amount_funded = self
            .get_ensure_funded_amount(account_to_fund, funding_params)
            .await?;

        if amount_funded.is_none() {
            return Ok(None);
        }

        let amount_funded = amount_funded.unwrap();

        let mut composer = self.get_composer();
        composer
            .add_payment(PaymentParams {
                sender: dispenser_account.clone(),
                receiver: account_to_fund.clone(),
                amount: amount_funded,
                note: funding_params.note.clone(),
                lease: funding_params.lease,
                static_fee: funding_params.static_fee,
                extra_fee: funding_params.extra_fee,
                max_fee: funding_params.max_fee,
                validity_window: funding_params.validity_window,
                first_valid_round: funding_params.first_valid_round,
                last_valid_round: funding_params.last_valid_round,
                ..Default::default()
            })
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to add funding payment: {}", e),
            })?;

        let composer_result =
            composer
                .send(send_params)
                .await
                .map_err(|e| AccountManagerError::AlgodError {
                    message: format!("Failed to send funding transaction: {}", e),
                })?;

        let last_result = composer_result
            .results
            .last()
            .ok_or(AccountManagerError::AlgodError {
                message: "No transaction returned".to_string(),
            })?
            .clone();

        Ok(Some(EnsureFundedResult {
            transaction_id: last_result.transaction_id,
            transaction: last_result.transaction,
            confirmation: last_result.confirmation,
            amount_funded,
        }))
    }

    /// Funds a given account using a dispenser account retrieved from the environment,
    /// per the `dispenser_from_environment` method, as a funding source such that
    /// the given account has a certain amount of Algo free to spend (accounting for
    /// Algo locked in minimum balance requirement).
    ///
    /// The dispenser account is retrieved from the account mnemonic stored in
    /// `DISPENSER_MNEMONIC` environment variable and optionally `DISPENSER_SENDER`
    /// if it's a rekeyed account, or against default LocalNet if no environment variables present.
    ///
    /// https://developer.algorand.org/docs/get-details/accounts/#minimum-balance
    ///
    /// # Parameters
    /// * `account_to_fund` - The address of the account to fund
    /// * `funding_params` - Parameters for funding
    /// * `send_params` - Optional parameters to control the execution of the transaction
    ///
    /// # Returns
    /// - `Some(EnsureFundedResult)` - The result of executing the dispensing transaction and the `amount_funded` if funds were needed
    /// - `None` - If no funds were needed
    pub async fn ensure_funded_from_environment(
        &mut self,
        account_to_fund: &Address,
        funding_params: &EnsureFundedParams,
        send_params: Option<SendParams>,
    ) -> Result<Option<EnsureFundedResult>, AccountManagerError> {
        // Get the dispenser account from environment
        let dispenser_account = self.dispenser_from_environment().await?;
        let dispenser_address = dispenser_account.address();

        // Delegate to ensure_funded with the dispenser account
        self.ensure_funded(
            account_to_fund,
            &dispenser_address,
            funding_params,
            send_params,
        )
        .await
    }

    /// Funds a given account using the TestNet Dispenser API as a funding source such that
    /// the account has a certain amount of Algo free to spend (accounting for Algo locked
    /// in minimum balance requirement).
    ///
    /// https://developer.algorand.org/docs/get-details/accounts/#minimum-balance
    ///
    /// # Parameters
    /// * `account_to_fund` - The address of the account to fund
    /// * `dispenser_client` - The TestNet dispenser funding client
    /// * `funding_params` - Parameters for funding
    ///
    /// # Returns
    /// - `Some(DispenserFundResponse)` - The result of executing the dispensing transaction
    /// - `None` - If no funds were needed
    pub async fn ensure_funded_from_testnet_dispenser_api(
        &mut self,
        account_to_fund: &Address,
        dispenser_client: &TestNetDispenserApiClient,
        funding_params: &EnsureFundedParams,
    ) -> Result<Option<DispenserFundResponse>, AccountManagerError> {
        let is_testnet = self.client_manager.is_testnet().await.map_err(|e| {
            AccountManagerError::EnvironmentError {
                message: format!("Failed to check if the environment is localnet: {}", e),
            }
        })?;
        if !is_testnet {
            return Err(AccountManagerError::EnvironmentError {
                message: "Attempt to fund using TestNet dispenser API on non TestNet network."
                    .to_string(),
            });
        }

        let amount_funded = self
            .get_ensure_funded_amount(account_to_fund, funding_params)
            .await?;

        match amount_funded {
            None => Ok(None),
            Some(amount_funded) => {
                let result = dispenser_client
                    .fund(account_to_fund, amount_funded)
                    .await
                    .map_err(|e| AccountManagerError::DispenserError {
                        message: e.to_string(),
                    })?;

                Ok(Some(result))
            }
        }
    }
}

/// Parameters for ensuring an account is funded
#[derive(Clone, Default, derive_more::Debug)]
pub struct EnsureFundedParams {
    /// Note to attach to the transaction. Max of 1000 bytes.
    pub note: Option<Vec<u8>>,
    /// Prevent multiple transactions with the same lease being included within the validity window.
    ///
    /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
    /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
    pub lease: Option<[u8; 32]>,
    /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
    pub static_fee: Option<u64>,
    /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
    pub extra_fee: Option<u64>,
    /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
    pub max_fee: Option<u64>,
    /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
    pub validity_window: Option<u32>,
    /// Set the first round this transaction is valid.
    /// If left undefined, the value from algod will be used.
    ///
    /// We recommend you only set this when you intentionally want this to be some time in the future.
    pub first_valid_round: Option<u64>,
    /// The last round this transaction is valid. It is recommended to use validity window instead.
    pub last_valid_round: Option<u64>,
    /// The minimum balance of Algo (in microAlgos) that the account should have available to spend
    pub min_spending_balance: u64,
    /// The optional minimum amount to fund if funding is needed (defaults to 0)
    pub min_funding_increment: Option<u64>,
}

/// Result of an ensure funded operation
#[derive(Debug, Clone)]
pub struct EnsureFundedResult {
    /// The transaction ID of the funding transaction
    pub transaction_id: String,
    /// The funding transaction
    pub transaction: Transaction,
    /// The confirmation of the funding transaction
    pub confirmation: algod_client::models::PendingTransactionResponse,
    /// The amount that was funded (in microAlgos)
    pub amount_funded: u64,
}

#[derive(Clone)]
struct AccountManagerSignerGetter {
    accounts: HashMap<Address, Arc<dyn TransactionSigner>>,
}

impl TransactionSignerGetter for AccountManagerSignerGetter {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        self.accounts
            .get(&address)
            .cloned()
            .ok_or_else(|| format!("No signer found for address: {}", address))
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

    #[snafu(display("Dispenser error: {message}"))]
    DispenserError { message: String },
}

/// Convert a 25-word mnemonic into a 32-byte secret key
fn mnemonic_to_secret_key(
    mnemonic: &str,
) -> Result<[u8; ALGORAND_SECRET_KEY_BYTE_LENGTH], AccountManagerError> {
    mnemonic::to_key(mnemonic).map_err(|e| AccountManagerError::MnemonicError {
        message: e.to_string(),
    })
}

/// Calculate the amount needed to fund an account to reach the minimum spending balance.
///
/// # Parameters
/// * `min_spending_balance` - The minimum spending balance required
/// * `current_spending_balance` - The current spending balance of the account
/// * `min_funding_increment` - The minimum amount to fund if funding is needed
///
/// # Returns
/// The amount to fund, or None if no funding is needed
fn calculate_fund_amount(
    min_spending_balance: u64,
    current_spending_balance: u64,
    min_funding_increment: u64,
) -> Option<u64> {
    if min_spending_balance > current_spending_balance {
        let min_fund_amount = min_spending_balance - current_spending_balance;
        Some(min_fund_amount.max(min_funding_increment))
    } else {
        None
    }
}

impl TransactionSignerGetter for AccountManager {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        self.get_signer(address).map_err(|e| e.to_string())
    }
}
