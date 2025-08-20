use super::common::CommonParams;
use algokit_utils::transactions::PaymentParams as RustPaymentParams;

#[derive(uniffi::Record)]
pub struct PaymentParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The address of the account receiving the ALGO payment.
    pub receiver: String,

    /// The amount of microALGO to send.
    ///
    /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
    pub amount: u64,
}

impl TryFrom<PaymentParams> for RustPaymentParams {
    type Error = String;

    fn try_from(params: PaymentParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustPaymentParams {
            common_params,
            receiver: params
                .receiver
                .parse()
                .map_err(|_| "Invalid receiver address")?,
            amount: params.amount,
        })
    }
}
