use algokit_transact::{Address, MultisigSubsignature};

#[derive(Debug, thiserror::Error)]
pub enum MultisigError {
    #[error("Invalid multisig account: {0}")]
    InvalidMultisigSignature(#[from] algokit_transact::AlgoKitTransactError),
}

#[derive(Clone)]
pub struct MultisigAccount {
    pub version: u8,
    pub threshold: u8,
    pub participants: Vec<Address>,
}

impl From<algokit_transact::MultisigSignature> for MultisigAccount {
    fn from(multisig: algokit_transact::MultisigSignature) -> Self {
        Self {
            version: multisig.version,
            threshold: multisig.threshold,
            participants: multisig.participants(),
        }
    }
}

impl TryFrom<MultisigAccount> for algokit_transact::MultisigSignature {
    type Error = MultisigError;
    fn try_from(account: MultisigAccount) -> Result<Self, Self::Error> {
        Ok(algokit_transact::MultisigSignature::from_participants(
            account.version,
            account.threshold,
            account.participants,
        )?)
    }
}

impl TryFrom<MultisigAccount> for Address {
    type Error = MultisigError;
    fn try_from(account: MultisigAccount) -> Result<Self, Self::Error> {
        let msig_signature: algokit_transact::MultisigSignature = account.try_into()?;
        Ok(msig_signature.into())
    }
}

pub struct MultisigSignature {
    pub subsignatures: Vec<Option<MultisigSubsignature>>,
}
