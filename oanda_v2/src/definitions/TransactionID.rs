/// The unique Transaction identifier within each Account.
///
/// Format: String representation of the numerical OANDA-assigned TransactionID
///
/// Example: 1523
struct transaction_id(String);
impl std::ops::Deref for transaction_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
