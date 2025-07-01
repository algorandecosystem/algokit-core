use crate::address::Address;
use crate::utils::hash;
use crate::{
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, MULTISIG_DOMAIN_SEPARATOR,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigAccount {
    #[serde(rename = "v")]
    pub version: u8,
    #[serde(rename = "thr")]
    pub threshold: u8,
    #[serde(rename = "subsig")]
    pub addrs: Vec<Address>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSubsig {
    #[serde(rename = "s")]
    #[serde_as(as = "Bytes")]
    pub sig: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSignature {
    #[serde(rename = "msig")]
    pub multisig: MultisigAccount,
    #[serde(rename = "subsig")]
    pub subsigs: Vec<MultisigSubsig>,
}

impl From<MultisigAccount> for Address {
    fn from(msig: MultisigAccount) -> Address {
        let mut buffer = Vec::with_capacity(
            MULTISIG_DOMAIN_SEPARATOR.len()
                + 2
                + msig.addrs.len() * ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
        );
        buffer.extend_from_slice(MULTISIG_DOMAIN_SEPARATOR.as_bytes());
        buffer.push(msig.version);
        buffer.push(msig.threshold);
        for addr in &msig.addrs {
            buffer.extend_from_slice(addr.as_bytes());
        }
        let digest = hash(&buffer);

        Address { 0: digest }
    }
}

impl Display for MultisigAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
