use crate::{test_utils::TransactionHeaderMother, KeyRegistrationTransactionBuilder};

pub struct KeyRegistrationTransactionMother {}

impl KeyRegistrationTransactionMother {
    pub fn online_key_registration() -> KeyRegistrationTransactionBuilder {
        KeyRegistrationTransactionBuilder::default()
            .header(TransactionHeaderMother::simple_testnet().build().unwrap())
            .vote_key([1u8; 32])
            .selection_key([2u8; 32])
            .state_proof_key([3u8; 64])
            .vote_first(100)
            .vote_last(1000)
            .vote_key_dilution(1000)
            .to_owned()
    }

    pub fn offline_key_registration() -> KeyRegistrationTransactionBuilder {
        KeyRegistrationTransactionBuilder::default()
            .header(TransactionHeaderMother::simple_testnet().build().unwrap())
            .to_owned()
    }

    pub fn non_participating_key_registration() -> KeyRegistrationTransactionBuilder {
        KeyRegistrationTransactionBuilder::default()
            .header(TransactionHeaderMother::simple_testnet().build().unwrap())
            .non_participation(true)
            .to_owned()
    }
}
