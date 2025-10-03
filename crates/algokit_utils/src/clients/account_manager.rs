use crate::{
    Composer, PaymentParams, SendParams, SendTransactionComposerResults, TransactionSigner,
    clients::{KmdAccountManager, genesis_id_is_localnet},
    common::mnemonic,
    transactions::{ComposerParams, common::TransactionSignerGetter},
};
use algod_client::{AlgodClient, models::Account};
use algokit_transact::{
    ALGORAND_SECRET_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, Address, AlgorandMsgpack,
    KeyPairAccount, SignedTransaction, Transaction,
};
use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
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

    /// Tracks and returns an Algorand account with private key loaded from the given KMD wallet (identified by name).
    ///
    /// # Parameters
    /// * `name` - The name of the wallet to retrieve an account from
    /// * `predicate` - An optional filter to use to find the account (otherwise it will return a random account from the wallet)
    /// * `sender` - The optional sender address to use this signer for (aka a rekeyed account)
    ///
    /// # Returns
    /// The account's address
    pub async fn from_kmd(
        &mut self,
        name: &str,
        predicate: Option<Box<dyn Fn(&Account) -> bool + Send + Sync>>,
        sender: Option<Address>,
    ) -> Result<Address, AccountManagerError> {
        let kmd_manager =
            self.kmd_account_manager
                .as_ref()
                .ok_or_else(|| AccountManagerError::KmdError {
                    message: "KMD client not available".to_string(),
                })?;

        let kmd_account = kmd_manager
            .get_wallet_account(name, predicate, sender)
            .await?;

        // Register the signer
        self.set_signer(kmd_account.address.clone(), kmd_account.signer);

        Ok(kmd_account.address)
    }

    // TODO: implement multisig and logicsig

    /// Tracks and returns a new, random Algorand account with secret key loaded.
    ///
    /// # Returns
    /// The account's address
    pub fn random(&mut self) -> Address {
        // Generate a random signing key using ed25519_dalek
        let signing_key = SigningKey::generate(&mut OsRng);
        let secret_key = signing_key.to_bytes();

        // Create signing account
        let signing_account = SigningAccount::new(secret_key);
        let address = signing_account.address();

        // Track the account
        let signer = Arc::new(signing_account);
        self.set_signer(address.clone(), signer);

        address
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
    /// The dispenser account's address
    pub async fn dispenser_from_environment(&mut self) -> Result<Address, AccountManagerError> {
        use std::env;

        const DISPENSER_ACCOUNT: &str = "DISPENSER";

        // Check if DISPENSER_MNEMONIC environment variable is set
        let mnemonic_key = format!("{}_MNEMONIC", DISPENSER_ACCOUNT);

        if env::var(&mnemonic_key).is_ok() {
            // Use from_environment which will handle both mnemonic and optional sender
            self.from_environment(DISPENSER_ACCOUNT, None).await
        } else {
            // Fall back to LocalNet dispenser
            self.localnet_dispenser().await
        }
    }

    /// Returns the default funded account from LocalNet's `unencrypted-default-wallet`.
    ///
    /// This is a convenience method that gets the dispenser account from the default KMD wallet
    /// that comes pre-funded on LocalNet.
    ///
    /// # Returns
    /// The LocalNet dispenser account's address
    pub async fn localnet_dispenser(&mut self) -> Result<Address, AccountManagerError> {
        let kmd_manager =
            self.kmd_account_manager
                .as_ref()
                .ok_or_else(|| AccountManagerError::KmdError {
                    message: "KMD client not available".to_string(),
                })?;

        let dispenser = kmd_manager.get_localnet_dispenser_account().await?;

        // Register the signer
        self.set_signer(dispenser.address.clone(), dispenser.signer);

        Ok(dispenser.address)
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
    ) -> Result<SendTransactionComposerResults, AccountManagerError> {
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

    fn get_composer(&self) -> Composer {
        Composer::new(ComposerParams {
            algod_client: self.algod.clone(),
            signer_getter: Arc::new(AccountManagerSignerGetter {
                accounts: self.accounts.clone(),
            }),
            composer_config: None,
        })
    }

    /// Get the amount of ALGOs needed to fund an account to reach the minimum spending balance.
    ///
    /// # Parameters
    /// * `sender` - The account address to check
    /// * `min_spending_balance` - The minimum spending balance (in microALGOs) the account should have above the minimum balance requirement
    /// * `min_funding_increment` - When funding, the minimum amount to transfer (defaults to 0)
    ///
    /// # Returns
    /// The amount in microALGOs needed to fund the account, or None if the account already has sufficient balance
    async fn get_ensure_funded_amount(
        &self,
        sender: &Address,
        min_spending_balance: u64,
        min_funding_increment: u64,
    ) -> Result<Option<u64>, AccountManagerError> {
        let account_info = self.get_information(sender).await?;
        let current_spending_balance = account_info.amount.saturating_sub(account_info.min_balance);

        let amount_funded = calculate_fund_amount(
            min_spending_balance,
            current_spending_balance,
            min_funding_increment,
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
    /// * `min_spending_balance` - The minimum balance of Algo (in microAlgos) that the account should have available to spend (i.e. on top of minimum balance requirement)
    /// * `min_funding_increment` - The minimum amount to fund if funding is needed (defaults to 0)
    /// * `send_params` - Optional parameters to control the execution of the transaction
    ///
    /// # Returns
    /// - `Some(EnsureFundedResult)` - The result of executing the dispensing transaction and the `amount_funded` if funds were needed
    /// - `None` - If no funds were needed
    pub async fn ensure_funded(
        &mut self,
        account_to_fund: &Address,
        dispenser_account: &Address,
        min_spending_balance: u64,
        min_funding_increment: u64,
        send_params: Option<SendParams>,
    ) -> Result<Option<EnsureFundedResult>, AccountManagerError> {
        let amount_funded = self
            .get_ensure_funded_amount(account_to_fund, min_spending_balance, min_funding_increment)
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
                ..Default::default()
            })
            .map_err(|e| AccountManagerError::AlgodError {
                message: format!("Failed to add funding payment: {}", e),
            })?;

        let result =
            composer
                .send(send_params)
                .await
                .map_err(|e| AccountManagerError::AlgodError {
                    message: format!("Failed to send funding transaction: {}", e),
                })?;

        // TODO: review this return
        Ok(Some(EnsureFundedResult {
            transaction_id: result.transaction_ids[0].clone(),
            transaction: result.transactions[0].clone(),
            confirmation: result.confirmations[0].clone(),
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
    /// * `min_spending_balance` - The minimum balance of Algo (in microAlgos) that the account should have available to spend (i.e. on top of minimum balance requirement)
    /// * `min_funding_increment` - The minimum amount to fund if funding is needed (defaults to 0)
    /// * `send_params` - Optional parameters to control the execution of the transaction
    ///
    /// # Returns
    /// - `Some(EnsureFundedResult)` - The result of executing the dispensing transaction and the `amount_funded` if funds were needed
    /// - `None` - If no funds were needed
    pub async fn ensure_funded_from_environment(
        &mut self,
        account_to_fund: &Address,
        min_spending_balance: u64,
        min_funding_increment: u64,
        send_params: Option<SendParams>,
    ) -> Result<Option<EnsureFundedResult>, AccountManagerError> {
        // Get the dispenser account from environment
        let dispenser_account = self.dispenser_from_environment().await?;

        // Delegate to ensure_funded with the dispenser account
        self.ensure_funded(
            account_to_fund,
            &dispenser_account,
            min_spending_balance,
            min_funding_increment,
            send_params,
        )
        .await
    }
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
