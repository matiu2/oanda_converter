/// The Trade’s identifier, unique within the Trade’s Account.
///
/// Format: The string representation of the OANDA-assigned TradeID. OANDA-assigned TradeIDs are positive integers, and are derived from the TransactionID of the Transaction that opened the Trade.
///
/// Example: 1523
struct trade_id(String);
impl std::ops::Deref for trade_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
