use serde::{Serialize, Deserialize};
/// The unique Transaction identifier within each Account.
///
/// Format: String representation of the numerical OANDA-
/// assigned TransactionID
///
/// Example: 1523
pub struct TransactionID(String);
impl std::ops::Deref for TransactionID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
