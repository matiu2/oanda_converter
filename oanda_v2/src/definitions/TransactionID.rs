/// The unique Transaction identifier within each Account.
///
/// Format: String representation of the numerical OANDA-assigned TransactionID
///
/// Example: 1523
struct TransactionID(String);
_blank_!();
impl std::ops::Deref for TransactionID {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
