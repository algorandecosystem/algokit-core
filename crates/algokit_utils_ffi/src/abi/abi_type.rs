#[uniffi::export(with_foreign)]
pub trait ABIType: Send + Sync {
    fn to_string(&self) -> String;
}
