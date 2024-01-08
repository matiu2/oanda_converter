use serde::{Serialize, Deserialize};
/// The Trade’s identifier, unique within the Trade’s Account.
///
/// Format: The string representation of the OANDA-assigned
/// TradeID. OANDA-assigned TradeIDs are positive integers, and
/// are derived from the TransactionID of the Transaction that
/// opened the Trade.
///
/// Example: 1523
pub struct TradeID(String);
impl std::ops::Deref for TradeID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
